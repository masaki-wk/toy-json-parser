//! JSON tokenizer.

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

        /// Verbosely output
        #[arg(short, long)]
        verbose: bool,
    }

    pub fn run(args: Args) -> Result<()> {
        use std::fs::File;
        use std::io::{self, BufReader, Read};
        use toy_json_parser::{BufReadCharsExt as _, Lexer};

        let reader: BufReader<Box<dyn Read>> = BufReader::new(if args.file == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(File::open(args.file)?)
        });

        let lexer = Lexer::new(reader.chars());
        if args.verbose {
            for result in lexer {
                match result {
                    Ok(token) => {
                        let kind = &token.kind;
                        let span = &token.span;
                        println!("{token}: {kind:?}, {span}")
                    }
                    Err(error) => {
                        let kind = error.kind;
                        let s = error.string;
                        let loc = error.location;
                        println!(r#"Error: {kind:?}, "{s}", {loc}"#)
                    }
                }
            }
        } else {
            for (i, result) in lexer.enumerate() {
                if i > 0 {
                    print!(" ")
                }
                match result {
                    Ok(token) => print!("{token}"),
                    Err(error) => print!("{error}"),
                }
            }
            println!()
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Result, ensure};
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    #[test]
    fn test() -> Result<()> {
        let input = String::from("{[]}");
        let mut child = Command::new("cargo")
            .args(["run", "--example", "json-tokenize"])
            .stdin(Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
        let status = child.wait()?;
        ensure!(status.success());
        Ok(())
    }
}
