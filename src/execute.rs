use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run(exec: &str, args: [&str; 5]) {
    let child = tokio::process::Command::new(exec)
        .args(args)
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
                    Ok(Some(line)) => println!("INFO: {}", line),
                    Err(_) => break,
                    _ => (),
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => println!("ERROR: {}", line),
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
