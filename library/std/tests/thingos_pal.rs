#![cfg(target_os = "thingos")]

use std::alloc::{GlobalAlloc, Layout, System};
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{self, Command};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod common;

const PROCESS_CHILD_ENV: &str = "THINGOS_PAL_PROCESS_CHILD";
const PROCESS_OUTPUT_STRESS_CHILD_ENV: &str = "THINGOS_PAL_PROCESS_OUTPUT_STRESS_CHILD";
const PROCESS_OUTPUT_STRESS_CHUNK_SIZE: usize = 4096;
const PROCESS_OUTPUT_STRESS_CHUNKS: usize = 128;

#[test]
fn test_alloc() {
    let layout = Layout::from_size_align(4096, 16).unwrap();
    unsafe {
        let ptr = System.alloc(layout);
        assert!(!ptr.is_null(), "System allocator returned null");
        std::ptr::write_bytes(ptr, 0xA5, layout.size());
        System.dealloc(ptr, layout);
    }
}

#[test]
fn test_thread_spawn_join() {
    let handle = thread::spawn(|| 0x2A_u32);
    assert_eq!(handle.join().unwrap(), 0x2A);
}

#[test]
fn test_thread_name() {
    let handle = thread::Builder::new()
        .name("thingos-pal-thread".to_string())
        .spawn(|| thread::current().name().map(str::to_owned))
        .unwrap();

    assert_eq!(handle.join().unwrap().as_deref(), Some("thingos-pal-thread"));
}

#[test]
fn test_env_roundtrip() {
    let key = "THINGOS_PAL_ENV_ROUNDTRIP";
    let value = "value-123";

    unsafe {
        std::env::set_var(key, value);
    }
    assert_eq!(std::env::var(key).unwrap(), value);

    unsafe {
        std::env::remove_var(key);
    }
    assert!(std::env::var(key).is_err());
}

#[test]
fn test_args() {
    let args: Vec<_> = std::env::args().collect();
    assert!(!args.is_empty(), "std::env::args() should include argv[0]");
}

#[test]
fn test_fs_create_read_write() {
    let tmp = common::tmpdir();
    let path = tmp.join("thingos-pal-fs.txt");
    let payload = b"thingos fs roundtrip";

    let mut f = File::create(&path).unwrap();
    f.write_all(payload).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    assert_eq!(buf, payload);

    drop(f);
    fs::remove_file(&path).unwrap();
    assert!(!path.exists());
}

#[test]
fn test_tcp_connect_loopback() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        let (mut conn, _) = listener.accept().unwrap();
        let mut recv = [0u8; 64];
        let n = conn.read(&mut recv).unwrap();
        conn.write_all(&recv[..n]).unwrap();
    });

    let mut client = TcpStream::connect(addr).unwrap();
    client.write_all(b"echo-thingos").unwrap();
    let mut out = vec![0u8; "echo-thingos".len()];
    client.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"echo-thingos");

    server.join().unwrap();
}

#[test]
fn test_process_spawn_exit() {
    if std::env::var_os(PROCESS_CHILD_ENV).is_some() {
        process::exit(23);
    }

    let me = std::env::current_exe().unwrap();
    let status = Command::new(me)
        .env(PROCESS_CHILD_ENV, "1")
        .arg("--exact")
        .arg("test_process_spawn_exit")
        .arg("--nocapture")
        .status()
        .unwrap();

    assert_eq!(status.code(), Some(23));
}

#[test]
fn test_process_output_interleaved_stress() {
    if std::env::var_os(PROCESS_OUTPUT_STRESS_CHILD_ENV).is_some() {
        let mut out = std::io::stdout().lock();
        let mut err = std::io::stderr().lock();
        let out_chunk = [b'o'; PROCESS_OUTPUT_STRESS_CHUNK_SIZE];
        let err_chunk = [b'e'; PROCESS_OUTPUT_STRESS_CHUNK_SIZE];

        for _ in 0..PROCESS_OUTPUT_STRESS_CHUNKS {
            out.write_all(&out_chunk).unwrap();
            err.write_all(&err_chunk).unwrap();
        }
        out.write_all(b"stdout-done\n").unwrap();
        err.write_all(b"stderr-done\n").unwrap();
        out.flush().unwrap();
        err.flush().unwrap();
        process::exit(0);
    }

    let me = std::env::current_exe().unwrap();
    let (tx, rx) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let output = Command::new(me)
            .env(PROCESS_OUTPUT_STRESS_CHILD_ENV, "1")
            .arg("--exact")
            .arg("test_process_output_interleaved_stress")
            .arg("--nocapture")
            .output();
        tx.send(output).unwrap();
    });

    let timeout = Duration::from_secs(10);
    let output = rx
        .recv_timeout(timeout)
        .expect("Command::output timed out (possible stdout/stderr drain deadlock)")
        .unwrap();

    assert!(output.status.success(), "child status: {:?}", output.status);
    assert!(output.stdout.ends_with(b"stdout-done\n"));
    assert!(output.stderr.ends_with(b"stderr-done\n"));

    let expected_min = PROCESS_OUTPUT_STRESS_CHUNK_SIZE * PROCESS_OUTPUT_STRESS_CHUNKS;
    assert!(output.stdout.len() >= expected_min);
    assert!(output.stderr.len() >= expected_min);
}

#[test]
fn test_time_monotonic() {
    let a = Instant::now();
    thread::sleep(Duration::from_millis(1));
    let b = Instant::now();
    assert!(b >= a, "Instant::now() must be non-decreasing");
}
