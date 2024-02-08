use std::{io, io::Write, process::Command};
use clap::Parser;
use args::Args;

mod args;

fn main() {
    let args = Args::parse();
    let cmd = args.command;
    let verbose = args.verbose;
    
    if verbose {
        eprintln!("CMDGATE: command = {}", cmd);
    }

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

    let mut arg = String::from("/C ");
    arg.push_str(&cmd);

    if verbose {
        eprintln!("CMDGATE: cmd args = /C {}", cmd);
    }

    let output = command
        .arg("/C")
        .arg("dir")
        .output().expect(format!("failed to execute process: cmd {arg}").as_str());

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
