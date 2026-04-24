use std::env;
use std::io::{self, Write};

fn expand_escapes(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some('\'') => out.push('\''),
            Some('\"') => out.push('\"'),
            Some('0') => {
                let mut octal = 0u8;
                let mut count = 0;
                while count < 3 {
                    if let Some(&next) = chars.peek() {
                        if next >= '0' && next <= '7' {
                            octal = octal * 8 + (next as u8 - b'0');
                            chars.next();
                            count += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                out.push(octal as char);
            }
            Some(next) => {
                out.push('\\');
                out.push(next);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let format_str = &args[1];
    let expanded = expand_escapes(format_str);
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = handle.write_all(expanded.as_bytes());
}
