use std::{io::Read, process::Stdio};

fn main() {
    println!("Test piped stdout");

    let mut p = std::process::Command::new("/bin/echo")
        .arg("TEST MESSAGE")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    assert!(p.wait().is_ok());

    let mut output = Vec::<u8>::new();
    p.stdout.as_mut().unwrap().read_to_end(&mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap().trim(),
        "TEST MESSAGE",
        "Expected message not piped into stdout"
    );

    println!("Test passed");
}
