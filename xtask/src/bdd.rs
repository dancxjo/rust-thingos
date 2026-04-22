use xshell::Shell;

use crate::common::Result;

pub fn bdd(
    sh: &Shell,
    feature: Option<String>,
    tags: Option<String>,
    archs: Vec<String>,
    _loglevel: Option<String>,
) -> Result<()> {
    let effective_loglevel = "4";

    for arch in archs {
        println!("xtask: bdd running tests for arch={arch}...");
        let mut cmd = xshell::cmd!(sh, "cargo run -p bdd");
        cmd = cmd.env("BDD_ARCH", &arch);
        cmd = cmd.env("BDD_LOGLEVEL", effective_loglevel);
        if let Some(f) = &feature {
            cmd = cmd.env("BDD_FEATURE", f);
        }
        if let Some(t) = &tags {
            // Cucumber CLI handles tags via --tags, passed through cargo run
            cmd = cmd.args(["--", "--tags", t]);
        }
        cmd.run()?;
    }
    Ok(())
}
