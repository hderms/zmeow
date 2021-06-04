use assert_cmd::prelude::*;
use std::process::Command;

fn command(file_name: &str) -> Command {
    let mut cmd = Command::cargo_bin("zmeow").unwrap();
    cmd.arg(file_name);
    cmd
}

#[test]
fn it_should_read_gzipped_data() {
    command("./tests/data/foo.gz")
        .assert()
        .success()
        .stdout(predicates::str::contains("123456789"))
        .stdout(predicates::str::contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
}
