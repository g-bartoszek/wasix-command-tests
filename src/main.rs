use std::{io::Read, process::{Stdio, }};

fn main() {
    test_ls();
    test_stdout();
    test_env();
    test_stderr();
    test_kill();
}

fn test_ls() {
    println!("Test spawn ls");

    let mut p = std::process::Command::new("/bin/ls")
        .arg("/")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    assert_eq!(p.wait().unwrap().code().unwrap(), 0);
}

fn test_stdout() {
    println!("Test spawn echo");

    let mut p = std::process::Command::new("/bin/echo")
        .arg("TEST MESSAGE")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    assert!(p.wait().is_ok());

    let mut output = Vec::<u8>::new();
    p.stdout.as_mut().unwrap().read_to_end(&mut output).unwrap();
    assert_eq!(String::from_utf8(output).unwrap().trim(), "TEST MESSAGE");
}

fn test_env() {
    println!("Test env variable");

    std::env::set_var("TEST_VARIABLE", "TEST_VARIABLE_VALUE");

    let mut p = std::process::Command::new("/bin/printenv")
        .arg("TEST_VARIABLE")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    assert!(p.wait().is_ok());

    let mut output = Vec::<u8>::new();
    p.stdout.as_mut().unwrap().read_to_end(&mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap().trim(),
        "TEST_VARIABLE_VALUE"
    );
}

fn test_stderr() {
    println!("Test stderr");

    let mut p = std::process::Command::new("/bin/ls")
        .arg("nonexisting")
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    assert_eq!(p.wait().unwrap().code().unwrap(), 2);

    let mut output = Vec::<u8>::new();
    p.stderr.as_mut().unwrap().read_to_end(&mut output).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap().trim(),
        "/bin/ls: cannot access 'nonexisting': No such file or directory"
    );
}

fn test_kill() {
    println!("Test kill");

    let mut p = std::process::Command::new("/bin/sleep")
        .arg("100")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    p.kill().unwrap();

    assert!(p.wait().unwrap().code().is_none());
}
