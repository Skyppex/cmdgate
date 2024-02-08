use std::{io, io::Write, process::Command};
use clap::Parser;
use args::Args;

mod args;

fn main() {
    let args = Args::parse();
    let cmd = args.command;
    let verbose = args.verbose;
    
    if verbose {
        eprintln!("CMDGATE -> command: {}", cmd);
        
        match args.source {
            Some(ref path) => eprintln!("source: {}", path),
            None => eprintln!("CMDGATE -> source: stdin"),
        }

        match args.destination {
            Some(ref path) => eprintln!("destination: {}", path),
            None => eprintln!("CMDGATE -> destination: stdout"),
        }
    }

    let command = &mut Command::new("cmd");

    match args.source {
        Some(path) => {
            command.stdin(std::process::Stdio::from(
                std::fs::File::open(&path)
                    .expect(format!("failed to open file: {path}").as_str())
                ));
        }
        None => {
            command.stdin(std::process::Stdio::piped());
        }
    }

    let command_parts = cmd.split_whitespace();
    let mut cmd_args = command_parts.collect::<Vec<&str>>();
    cmd_args.insert(0, "/C");

    if verbose {
        eprintln!("CMDGATE -> cmd args: {}", cmd_args.join(" "));
    }

    let output = command
        .args(cmd_args.clone())
        .output().expect(format!("failed to execute process: cmd {}", cmd_args.join(" ")).as_str());

    let stdout = if args.utf8 {
        String::from_utf8_lossy(&output.stdout)
            .as_bytes()
            .to_vec()
    } else {
        output.stdout
    };
        
    match args.destination {
        None => io::stdout().write_all(&stdout)
            .expect("failed to write to stdout"),
        Some(path) => {
            std::fs::write(&path, &stdout)
                .expect(format!("failed to write file: {path}").as_str());
        }
    }

    io::stderr().write_all(&output.stderr).expect("failed to write to stderr");
}

// C:/temp/input.txt
// C:/temp/output.txt
