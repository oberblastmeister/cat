use std::io::{self, Write};
use std::process::Command;

use assert_cmd::prelude::*;
use tempfile::NamedTempFile;
use predicates::prelude::*;
use anyhow::Result;

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("cat")?;

    cmd.arg("/test/this/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a file"));

    Ok(())
}
