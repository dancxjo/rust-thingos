use xshell::Shell;

use crate::common::Result;

pub fn bdd(
    sh: &Shell,
    feature: Option<String>,
    tags: Option<String>,
    archs: Vec<String>,
    loglevel: Option<String>,
) -> Result<()> {
    for arch in archs {
        println!("xtask: bdd running tests for arch={arch}...");
        let mut cmd = xshell::cmd!(sh, "cargo run -p bdd");
        cmd = cmd.env("BDD_ARCH", &arch);
        if let Some(f) = &feature {
            cmd = cmd.env("BDD_FEATURE", f);
        }
        if let Some(l) = &loglevel {
            cmd = cmd.env("BDD_LOGLEVEL", l);
        }
        if let Some(t) = &tags {
            // Cucumber CLI handles tags via --tags, passed through cargo run
            cmd = cmd.args(["--", "--tags", t]);
        }
        cmd.run()?;
    }
    Ok(())
}
