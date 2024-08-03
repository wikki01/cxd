use std::process::Command;

use assert_cmd::prelude::*;
use predicates::{prelude::*, str::contains};

use crate::util::TempCacheDir;

#[test]
fn short() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd.env("CXD_CACHE_DIR", dir.as_ref());

    cmd.arg("-a")
        .arg("test")
        .arg("echo")
        .arg("hi")
        .assert()
        .success()
        .stdout(contains("test"));

    Ok(())
}

#[test]
fn long() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd.env("CXD_CACHE_DIR", dir.as_ref());

    cmd.arg("--add")
        .arg("test")
        .arg("echo")
        .arg("hi")
        .assert()
        .success()
        .stdout(contains("test"));

    Ok(())
}

#[test]
fn cwd_long() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--cwd")
        .arg("test")
        .arg("pwd")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains(std::env::current_dir()?.to_str().unwrap()));

    Ok(())
}

#[test]
fn cwd_short() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("-c")
        .arg("test")
        .arg("pwd")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains(std::env::current_dir()?.to_str().unwrap()));

    Ok(())
}

#[test]
fn dir_long() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--dir")
        .arg(std::env::current_dir()?.to_str().unwrap())
        .arg("test")
        .arg("pwd")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains(std::env::current_dir()?.to_str().unwrap()));

    Ok(())
}

#[test]
fn dir_short() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("-d")
        .arg(std::env::current_dir()?.to_str().unwrap())
        .arg("test")
        .arg("pwd")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains(std::env::current_dir()?.to_str().unwrap()));

    Ok(())
}

#[test]
fn add_exec() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("test")
        .arg("echo")
        .arg("hi")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test").assert().success().stdout(contains("hi"));

    Ok(())
}

#[test]
fn add_env_long() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--env")
        .arg("CXD_TEST=hi")
        .arg("test")
        .arg("printenv")
        .arg("CXD_TEST")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test").assert().success().stdout(contains("hi"));

    Ok(())
}

#[test]
fn add_env_short() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("-e")
        .arg("CXD_TEST=hi")
        .arg("test")
        .arg("printenv")
        .arg("CXD_TEST")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test").assert().success().stdout(contains("hi"));

    Ok(())
}

#[test]
fn add_env_multiple() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--env")
        .arg("CXD_TEST1=hi")
        .arg("--env")
        .arg("CXD_TEST2=there")
        .arg("test")
        .arg("sh")
        .arg("-c")
        .arg("printenv CXD_TEST1 && printenv CXD_TEST2")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains("hi").and(contains("there")));

    Ok(())
}

#[test]
fn add_cwd() -> anyhow::Result<()> {
    let mut cmd1 = Command::cargo_bin("cxd")?;
    let dir = TempCacheDir::new()?;
    cmd1.env("CXD_CACHE_DIR", dir.as_ref());

    cmd1.arg("--add")
        .arg("--env")
        .arg("CXD_TEST1=hi")
        .arg("--env")
        .arg("CXD_TEST2=there")
        .arg("test")
        .arg("sh")
        .arg("-c")
        .arg("printenv CXD_TEST1 && printenv CXD_TEST2")
        .assert()
        .success()
        .stdout(contains("test"));

    let mut cmd2 = Command::cargo_bin("cxd")?;
    cmd2.env("CXD_CACHE_DIR", dir.as_ref());

    cmd2.arg("test")
        .assert()
        .success()
        .stdout(contains("hi").and(contains("there")));

    Ok(())
}
