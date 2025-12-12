use assert_cmd::Command;

#[test]
fn test_cli_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("env-cli"));

    Ok(())
}

#[test]
fn test_cli_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("0.1.0"));

    Ok(())
}

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("init");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Initializing"));

    Ok(())
}

#[test]
fn test_scan_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("env")?;

    cmd.arg("scan");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Scanning"));

    Ok(())
}