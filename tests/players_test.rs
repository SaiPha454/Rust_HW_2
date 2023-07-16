use assert_cmd::Command;

#[test]
fn list_players_without_arguments() {

    let mut cmd = Command::cargo_bin("list_players").unwrap();

    let expected = format!("Player 1: N/A\nPlayer 2: N/A\n");
    cmd.assert().success().stdout(expected);
}

#[test]
fn list_players_with_one_arg() {

    let arg1= "Jhon";
    let expected = format!("Player 1: {arg1}\nPlayer 2: N/A\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg(arg1);
    cmd.assert().success().stdout(expected);
}

#[test]
fn list_players_with_two_args() {

    let arg1= "Jhon";
    let arg2 = "Jarvis";
    let expected = format!("Player 1: {arg1}\nPlayer 2: {arg2}\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.args(vec![arg1, arg2]);
    cmd.assert().success().stdout(expected);
}

#[test]
fn list_players_with_extra_args() {

    let arg1= "Jhon";
    let arg2 = "Jarvis";
    let arg3 = "Bobby";
    let expected = format!("Player 1: {arg1}\nPlayer 2: {arg2}\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.args(vec![arg1, arg2, arg3]);
    cmd.assert().success().stdout(expected);
}