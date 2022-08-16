use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn start() {
    let child = tokio::process::Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M"])
        .arg("-jar")
        .arg("server.jar")
        .arg("nogui")
        .current_dir(std::env::current_dir().unwrap())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn();

    let mut provoke = match child {
        Ok(c) => c,
        Err(err) => panic!("Can't start the server: {}", err),
    };

    let stdout = provoke
        .stdout
        .take()
        .expect("child did not have a handle to stdout");
    let stderr = provoke
        .stderr
        .take()
        .expect("child did not have a handle to stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => println!("Stdout: {}", line),
                    Err(_) => break,
                    _ => (),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => println!("Stderr: {}", line),
                    Err(_) => break,
                    _ => (),
                }
            }
            result = provoke.wait() => {
                if let Ok(exit_code) = result { println!("Child process exited with {}", exit_code) }
                break // child process exited
            }
        };
    }
}
