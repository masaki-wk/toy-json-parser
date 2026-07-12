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
        #[arg(help = "JSON string")]
        code: String,
    }

    pub fn run(args: Args) -> Result<()> {
        use toy_json_parser::{Lexer, Parser};
        let lexer = Lexer::new(args.code.chars());
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
