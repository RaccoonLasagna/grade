use assert_cmd::Command;

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.assert().success().stdout("Input something!\n");
}

#[test]
fn Invalid_over100() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("101").assert().success().stdout("Invalid score\n");
}

#[test]
fn A_plus() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("95").assert().success().stdout("Excellent with A+\n");
}

#[test]
fn A() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("81").assert().success().stdout("A\n");
}

#[test]
fn B() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("71").assert().success().stdout("B\n");
}

#[test]
fn C() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("61").assert().success().stdout("C\n");
}

#[test]
fn D() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("50").assert().success().stdout("D\n");
}

#[test]
fn Failed() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("0").assert().success().stdout("Failed with F\n");
}

#[test]
fn Invalid_under0() {    
    let mut cmd = Command::cargo_bin("HW4_1_grade").unwrap();
    cmd.arg("-1").assert().success().stdout("Invalid score\n");
}