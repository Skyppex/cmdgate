use args::Args;
use clap::Parser;
use std::{
    env,
    io::{self, Write},
    process::Command,
};

mod args;

fn main() {
    let args = Args::parse();
    let name = env::args().next().unwrap();
    let cmd = args.command;
    let verbose = args.verbose;

    if verbose {
        eprintln!("{} -> inner command: {}", name, cmd);
        eprintln!("{} -> outer command: cmd.exe /C \"{}\"", name, cmd);

        match args.source {
            Some(ref path) => eprintln!("source: {}", path),
            None => eprintln!("{} -> source: stdin", name),
        }

        match args.destination {
            Some(ref path) => eprintln!("destination: {}", path),
            None => eprintln!("{} -> destination: stdout", name),
        }
    }

    let command = &mut Command::new("cmd.exe");

    match args.source {
        Some(path) => {
            command.stdin(std::process::Stdio::from(
                std::fs::File::open(&path)
                    .unwrap_or_else(|_| panic!("failed to open file: {path}")),
            ));
        }
        None => {
            command.stdin(std::process::Stdio::piped());
        }
    }

    let output = command
        .args(["/C", &cmd])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute process: cmd {}", cmd));

    let stdout = if args.utf8 {
        String::from_utf8_lossy(&output.stdout).as_bytes().to_vec()
    } else {
        output.stdout
    };

    match args.destination {
        None => io::stdout()
            .write_all(&stdout)
            .expect("failed to write to stdout"),
        Some(path) => {
            std::fs::write(&path, &stdout)
                .unwrap_or_else(|_| panic!("failed to write file: {path}"));
        }
    }

    io::stderr()
        .write_all(&output.stderr)
        .expect("failed to write to stderr");
}

// C:/temp/input.txt
// C:/temp/output.txt
