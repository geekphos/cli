use std::{
    io::{self, BufRead},
    process::{Command, Stdio},
};

pub(crate) fn exec_command(command: &str, args: &Vec<&str>) {
    // exec command
    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("执行命令失败");
    let stdout = child.stdout.take().unwrap();

    let reader = io::BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    let status = child.wait().expect("等待命令执行失败");
    println!("Exited with status: {}", status);
}
