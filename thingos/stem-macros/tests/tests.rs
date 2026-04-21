#[stem_macros::main]
fn entry() -> ! {
    panic!("macro expansion smoke test should not execute entry")
}

#[test]
fn main_macro_expands_with_stem_entrypoint() {
    let _entrypoint: unsafe extern "C" fn(usize) -> ! = stem_user_main;
}
