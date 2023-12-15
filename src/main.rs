use std::{io, io::Write, process::Command};
use clap::Parser;
use args::Args;

mod args;

fn main() {
    let args = Args::parse();
    let cmd = args.command;
    // println!("cmd: {}", cmd);
    let command = &mut Command::new("cmd");

    match args.source {
        Some(path) => {
            // println!("path: {}", path);
            command.stdin(std::process::Stdio::from(
                std::fs::File::open(&path).expect(format!("failed to open file: {path}").as_str()
                )));
        }
        None => {
            command.stdin(std::process::Stdio::piped());
        }
    }

    let mut arg = "/C ".to_owned();
    arg.push_str(cmd.as_str());

    // println!("arg: {}", arg);

    let output = command
        .arg(arg)
        .output().expect(format!("failed to execute process: {cmd}").as_str());

    match args.destination {
        None => io::stdout().write_all(&output.stdout).expect("failed to write to stdout"),
        Some(path) => {
            std::fs::write(&path, &output.stdout)
                .expect(format!("failed to write file: {path}").as_str());
        }
    }

    io::stderr().write_all(&output.stderr).expect("failed to write to stderr");
}

// C:/temp/input.txt
// C:/temp/output.txt
