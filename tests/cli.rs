use assert_cmd::prelude::*;
use dwrench::create_test_csv_file;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dwrench")?;

    cmd.arg("csvconvert")
        .arg("parquet")
        .arg("nonexistingfile.csv")
        .arg("testfile.parquet");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn invalid_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dwrench")?;

    cmd.arg("invalidsubcommandy");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
    Ok(())
}

#[test]
fn csv_convert_works() -> Result<(), Box<dyn std::error::Error>> {
    create_test_csv_file("testfile.csv");
    let mut cmd = Command::cargo_bin("dwrench")?;

    cmd.arg("csvconvert")
        .arg("parquet")
        .arg("testfile.csv")
        .arg("testfile.parquet");
    cmd.assert().success();
    Ok(())
}
