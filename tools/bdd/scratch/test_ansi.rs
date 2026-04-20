use regex::Regex;

fn strip_ansi(s: &str) -> String {
    let re = Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]")
        .expect("Invalid ANSI regex");
    re.replace_all(s, "").replace('\r', "")
}

fn main() {
    let s = "\u{1b}[2J\u{1b}[01;01H";
    let clean = strip_ansi(s);
    println!("Raw: {:?}", s);
    println!("Clean: {:?}", clean);
    assert_eq!(clean, "");
}
