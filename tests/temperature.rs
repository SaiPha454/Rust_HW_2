use assert_cmd::Command;

#[test]
fn convert_fah_to_celsius(){

    let fah = ["-f", "50"];
    let mut cmd = Command::cargo_bin("temperature").unwrap();
    cmd.args(fah).assert().success();
}

#[test]
fn convert_celsius_to_fah(){

    let cel = ["-c", "100"];
    let mut cmd = Command::cargo_bin("temperature").unwrap();
    cmd.args(cel).assert().success();
}