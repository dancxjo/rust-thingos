//! Path resolution engine for the VFS.
//!
//! Implements iterative resolution of absolute paths with support for:
//! - Multi-component paths: `/a/b/c`
//! - Current-directory component (`.`): ignored
//! - Parent-directory component (`..`): pops the last resolved component
//! - Mount-point crossings: delegated to the global mount table
//! - Symbolic link following: up to [`MAX_SYMLINK_DEPTH`] levels
//!
//! # Design
//! The engine performs a single-pass lookup first for speed. If it hits a symlink
//! or fails with an error that suggests intermediate symlinks, it falls back
//! to a component-by-component walk.
//!
//! The public entry points are:
//! - [`resolve`] — resolve with symlink following (for `open`, `stat`, etc.)
//! - [`resolve_no_follow`] — resolve the path without following the final symlink
//!   (for `readlink`, `lstat`-style operations)

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::errors::{Errno, SysResult};
use crate::vfs::VfsNode;

/// Maximum number of components allowed in a path before returning `ENAMETOOLONG`.
const MAX_COMPONENTS: usize = 64;

/// Maximum number of symlink expansions before returning `ELOOP`.
const MAX_SYMLINK_DEPTH: usize = 40;

/// Resolve an absolute path to a VFS node, following symlinks.
pub fn resolve(path: &str) -> SysResult<Arc<dyn VfsNode>> {
    resolve_ext(path, true, 0)
}

/// Resolve an absolute path **without** following the final component if it is
//! a symlink. Symlinks in intermediate path components are still followed.
pub fn resolve_no_follow(path: &str) -> SysResult<Arc<dyn VfsNode>> {
    resolve_ext(path, false, 0)
}

/// Internal entry point for path resolution.
fn resolve_ext(path: &str, follow_final: bool, depth: usize) -> SysResult<Arc<dyn VfsNode>> {
    if depth > MAX_SYMLINK_DEPTH {
        return Err(Errno::ELOOP);
    }

    let normalised = normalise(path)?;

    // 1. Fast path: try to resolve the whole thing at once.
    // This works if there are no intermediate symlinks that cross mount points.
    match crate::vfs::mount::lookup(&normalised) {
        Ok(node) => {
            if follow_final {
                let stat = node.stat()?;
                if stat.is_symlink() {
                    let target = node.readlink()?;
                    let new_path = join_symlink(&normalised, &target)?;
                    return resolve_ext(&new_path, true, depth + 1);
                }
            }
            return Ok(node);
        }
        Err(Errno::ENOENT) | Err(Errno::ENOTDIR) => {
            // If it's a single component, it's just missing.
            if !normalised[1..].contains('/') {
                return Err(Errno::ENOENT);
            }
        }
        Err(e) => return Err(e),
    }

    // 2. Slow path: walk component by component.
    walk_path(&normalised, follow_final, depth)
}

/// Walk the path component by component to handle intermediate symlinks.
fn walk_path(path: &str, follow_final: bool, depth: usize) -> SysResult<Arc<dyn VfsNode>> {
    let components: Vec<&str> = path[1..].split('/').filter(|c| !c.is_empty()).collect();
    if components.is_empty() {
        return crate::vfs::mount::lookup("/");
    }

    let mut current_path = String::with_capacity(path.len());
    for (i, component) in components.iter().enumerate() {
        current_path.push('/');
        current_path.push_str(component);

        let is_last = i == components.len() - 1;
        let node = crate::vfs::mount::lookup(&current_path)?;

        // Check if we need to follow this component.
        // We follow if it's NOT the last component, OR if it's the last and follow_final is true.
        if !is_last || follow_final {
            let stat = node.stat()?;
            if stat.is_symlink() {
                let target = node.readlink()?;
                
                // Construct the remaining path.
                let remaining = components[i + 1..].join("/");
                let new_base = resolve_relative(&current_path, &target);
                
                let new_path = if remaining.is_empty() {
                    new_base
                } else {
                    alloc::format!("{}/{}", new_base, remaining)
                };
                
                return resolve_ext(&new_path, follow_final, depth + 1);
            }
            
            // If it's NOT the last component, it MUST be a directory.
            if !is_last && !stat.is_dir() {
                return Err(Errno::ENOTDIR);
            }
        }

        if is_last {
            return Ok(node);
        }
    }

    unreachable!()
}

/// Join a base path and a symlink target.
fn join_symlink(base: &str, target: &str) -> SysResult<String> {
    if target.starts_with('/') {
        Ok(target.to_string())
    } else {
        let parent = match base.rfind('/') {
            Some(0) => "/",
            Some(idx) => &base[..idx],
            None => "/",
        };
        if parent == "/" {
            Ok(alloc::format!("/{}", target))
        } else {
            Ok(alloc::format!("{}/{}", parent, target))
        }
    }
}

/// Resolve a target relative to a current path.
fn resolve_relative(current: &str, target: &str) -> String {
    if target.starts_with('/') {
        target.to_string()
    } else {
        let parent = match current.rfind('/') {
            Some(0) => "/",
            Some(idx) => &current[..idx],
            None => "/",
        };
        if parent == "/" {
            alloc::format!("/{}", target)
        } else {
            alloc::format!("{}/{}", parent, target)
        }
    }
}

/// Normalise an absolute path, resolving `.` and `..` components.
pub fn normalise(path: &str) -> SysResult<String> {
    if path.is_empty() || !path.starts_with('/') {
        return Err(Errno::EINVAL);
    }

    let mut components: Vec<&str> = Vec::new();

    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            name => {
                if components.len() >= MAX_COMPONENTS {
                    return Err(Errno::ENAMETOOLONG);
                }
                components.push(name);
            }
        }
    }

    if components.is_empty() {
        return Ok(String::from("/"));
    }

    let capacity: usize = components.iter().map(|c| c.len() + 1).sum();
    let mut result = String::with_capacity(capacity);
    for c in &components {
        result.push('/');
        result.push_str(c);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise() {
        assert_eq!(normalise("/a/b/c").unwrap(), "/a/b/c");
        assert_eq!(normalise("/a/./b").unwrap(), "/a/b");
        assert_eq!(normalise("/a/b/../c").unwrap(), "/a/c");
        assert_eq!(normalise("/../../a").unwrap(), "/a");
        assert_eq!(normalise("//a//b").unwrap(), "/a/b");
        assert_eq!(normalise("/").unwrap(), "/");
    }
}
