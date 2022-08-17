use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let mut cmd = Command::new("python");
    cmd.arg("test.py");

    let mut processes = Vec::new();

    for _ in 0..4 {
        let process = cmd.spawn().expect("Failed to spawn process");
        processes.push(process);
    }

    thread::sleep(Duration::from_secs(3));

    for mut process in processes {
        let pid = process.id();
        let result = process.wait().expect("failed to retrieve process");
        let code = result.code().unwrap();
        println!("Process {pid} exited with status code: {code}");
    }
}
