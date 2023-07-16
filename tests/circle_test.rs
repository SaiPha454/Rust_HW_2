use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn circle_area_without_radius(){
    let mut cmd = Command::cargo_bin("circle").unwrap();
    cmd.assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn circle_area_with_radius(){

    let radius = "5";

    let mut cmd = Command::cargo_bin("circle").unwrap();
    cmd.arg(radius)
    .assert()
    .success();
   
}

#[test]
fn circle_area_with_radius_with_stdout(){

    let radius = "5";

    let radius_cal = &radius.to_string().parse().unwrap_or(0.0);
    let circle_area =  3.14159 * radius_cal * radius_cal;
    let expected = format!("The area of the circle with radius = {radius} is : {circle_area}\n");

    let mut cmd = Command::cargo_bin("circle").unwrap();

    cmd.arg(radius)
    .assert()
    .success().stdout(expected);
    
}