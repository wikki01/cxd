use assert_cmd::Command;
use predicates::str::contains;

use crate::util::TempCacheDir;

#[test]
fn short() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--cwd")
        .arg("test")
        .arg("echo")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("-r")
        .arg("test")
        .assert()
        .success()
        .stdout(contains("test"));

    Ok(())
}

#[test]
fn long() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--cwd")
        .arg("test")
        .arg("echo")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("--remove")
        .arg("test")
        .assert()
        .success()
        .stdout(contains("test"));

    Ok(())
}

#[test]
fn id_short() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--cwd")
        .arg("test")
        .arg("echo")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("--remove")
        .arg("-i")
        .arg("1")
        .assert()
        .success()
        .stdout(contains("1"));

    Ok(())
}

#[test]
fn id_long() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--cwd")
        .arg("test")
        .arg("echo")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("--remove")
        .arg("--id")
        .arg("1")
        .assert()
        .success()
        .stdout(contains("1"));

    Ok(())
}
