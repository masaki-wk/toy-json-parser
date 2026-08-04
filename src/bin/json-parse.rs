//! JSON parser.

fn main() -> anyhow::Result<()> {
    use clap::Parser as _;
    let args = app::Args::parse();
    app::run(args)
}

mod app {
    use anyhow::Result;
    use clap::Parser;

    #[derive(Parser, Debug)]
    pub struct Args {
        /// JSON file
        #[arg(default_value = "-")]
        file: String,
    }

    pub fn run(args: Args) -> Result<()> {
        use std::fs::File;
        use std::io::{self, BufReader, Read};
        use toy_json_parser::{BufReadCharsExt as _, Lexer, Parser};

        let reader: BufReader<Box<dyn Read>> = BufReader::new(if args.file == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(File::open(args.file)?)
        });

        let lexer = Lexer::new(reader.chars());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(value) => {
                println!("{:?}", value);
                Ok(())
            }
            Err(diag) => {
                anyhow::bail!(format!("{:?}", diag))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Result, ensure};
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    #[test]
    fn test() -> Result<()> {
        let input = String::from("[0, 1]");
        let mut child = Command::new("cargo").args(["run", "--bin", "json-parse"]).stdin(Stdio::piped()).spawn()?;
        child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
        let status = child.wait()?;
        ensure!(status.success());
        Ok(())
    }
}
