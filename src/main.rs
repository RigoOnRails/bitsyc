use std::path::Path;

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

use anyhow::{Result, bail};
use clap::Parser as ArgumentParser;

#[derive(ArgumentParser)]
#[command(version, about)]
struct Arguments {
    /// The Bitsy source file to compile.
    file: Option<String>,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    let Some(file) = arguments.file else {
        bail!("We're gonna need a file. 😀🔪");
    };

    let path = Path::new(&file);
    if !path.exists() {
        bail!("Can't find that file. Check the path? 🤔");
    }

    if path.extension().map(|ext| ext.to_str().unwrap()) != Some("bitsy") {
        bail!("The file must have a .bitsy extension. 🤠");
    }

    let Ok(input) = std::fs::read_to_string(path) else {
        bail!("Couldn't read the file. Is it valid UTF-8? 🤔");
    };

    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer);

    Ok(())
}
