use std::sync::OnceLock;
use regex::Regex;

pub fn strip_ansi(s: &str) -> String {
    static ANSI_RE: OnceLock<Regex> = OnceLock::new();
    let re = ANSI_RE.get_or_init(|| {
        Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]")
            .expect("Invalid ANSI regex")
    });
    re.replace_all(s, "").replace('\r', "")
}

fn main() {
    let log = std::fs::read_to_string("docs/behavior/x86_64/scheduler-entry-latency-instrumentation/scheduler-loop-entry-is-reached-before-deferred-boot-framebuffer-paint/serial.log").unwrap();
    let first = "Scheduler initialized".to_string();
    let second = "Entering scheduler loop.".to_string();

    let clean_log = strip_ansi(&log).to_lowercase();
    let clean_first = strip_ansi(&first).to_lowercase();
    let clean_second = strip_ansi(&second).to_lowercase();

    println!("clean_first: '{}'", clean_first);
    println!("clean_second: '{}'", clean_second);

    let first_pos = clean_log.find(&clean_first);
    if let Some(f_idx) = first_pos {
        println!("Found first at index {}", f_idx);
        let start_from = f_idx + clean_first.len();
        if clean_log[start_from..].contains(&clean_second) {
            println!("Found second after first!");
        } else {
            println!("Did NOT find second after first!");
            if let Some(any_idx) = clean_log.find(&clean_second) {
                println!("Second found at index {} (BEFORE first)", any_idx);
            } else {
                println!("Second NOT found anywhere!");
            }
        }
    } else {
        println!("First NOT found!");
    }
}
