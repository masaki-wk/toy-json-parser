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
        #[arg(help = "JSON string")]
        code: String,
    }

    pub fn run(args: Args) -> Result<()> {
        use toy_json_parser::Lexer;
        let lexer = Lexer::new(args.code.chars());
        for token in lexer {
            println!("{:?}", token)
        }
        Ok(())
    }
}
