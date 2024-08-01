use std::process::Command;

use assert_cmd::prelude::*;
use predicates::{prelude::*, str::contains};

#[test]
fn short_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("-h")
        .assert()
        .success()
        .stdout(contains("Show the long version of this help").and(contains("cxd")));
    Ok(())
}

#[test]
fn long_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Show the short version of this help").and(contains("cxd")));
    Ok(())
}

#[test]
fn add_long_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--help")
        .arg("--add")
        .assert()
        .success()
        .stdout(contains("Add a new command").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn add_long_help_rev() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--add")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Add a new command").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn remove_long_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--help")
        .arg("--remove")
        .assert()
        .success()
        .stdout(contains("Remove a command").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn remove_long_help_rev() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--remove")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Remove a command").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn list_long_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--help")
        .arg("--list")
        .assert()
        .success()
        .stdout(contains("List available commands").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn list_long_help_rev() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--list")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("List available commands").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn clear_long_help() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--help")
        .arg("--clear")
        .assert()
        .success()
        .stdout(contains("Clear all commands").and(contains("cxd").not()));
    Ok(())
}

#[test]
fn clear_long_help_rev() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;

    cmd.arg("--clear")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Clear all commands").and(contains("cxd").not()));
    Ok(())
}
