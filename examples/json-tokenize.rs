//! JSON tokenizer.

fn main() -> anyhow::Result<()> {
    use clap::Parser as _;
    let args = app::Args::parse();
    app::run(args)
}

mod app {
    use anyhow::Result;
    use clap::Parser;

    /// JSON tokenizer.
    #[derive(Parser, Debug)]
    pub struct Args {
        /// JSON file
        #[arg(default_value = "-")]
        file: String,

        /// Enable Verbose output
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
            let results = lexer
                .map(|x| match x {
                    Ok(token) => Ok((token.to_string(), format!("{:?}", token.kind()), token.span().start().to_string())),
                    Err(error) => Err(format!(r#"Error: {:?}, "{}", {}"#, error.kind, error.string, error.location)),
                })
                .collect::<Vec<_>>();
            let token_lens = results.iter().filter_map(|x| x.as_ref().ok()).map(|(x, y, z)| (x.len(), y.len(), z.len()));
            let width_raw = token_lens.clone().map(|(x, _, _)| x).max().unwrap_or(0);
            let width_kind = token_lens.clone().map(|(_, x, _)| x).max().unwrap_or(0);
            let width_span = token_lens.clone().map(|(_, _, x)| x).max().unwrap_or(0);
            for x in results {
                match x {
                    Ok((raw, kind, span)) => {
                        println!("{raw:width_raw$} {kind:width_kind$} {span:width_span$}")
                    }
                    Err(s) => {
                        println!("{s}")
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
