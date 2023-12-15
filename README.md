# cmdgate

`cmdgate` provides a way to call cmd commands from other CLIs such as bash.

## Installation

I didn't want to push a binary to the repo so you will have to build it yourself using `cargo` and `rustc`. Make sure you have that installed!
Then pull the repo onto your machine, navigate to the path containing the `cargo.toml` file.
Make sure to take a look at the code so you know its not malware (i had some problems with my anti malware software netralizing the binary when i ran it.
Run `cargo build --release` and the exe will be dumped in the `./target/release` folder. From there you have the binary and can do with it what you wish. :D

## Usage

At any time when running the command you can use `-h` or `--help` to see all the options available.

### Options
- `-s or --source` -> Path to file to read from, otherwise it uses stdin
- `-d or --destination` -> Path to file to write to, otherwise it uses stdout
- `-c or --command` -> cmd command to run (REQUIRED)
- `-v or --version` -> This will print the version number (which will probably never change)

## Pull Requests & Issues

If you have some functionality you wish to add then make a PR.
If you find a bug or want to discuss something about the tool make an issue out of it and we can discuss it :D

## License

This tool is under `CC0` so feel free to do whatever you want with it with no obligation to credit me.
