use std::{io, io::prelude::*, error::Error, fs::File, path::PathBuf};
use structopt::StructOpt;
use putt::parser::eval_from_str;

#[derive(StructOpt, Debug)]
#[structopt(name = "putt")]
struct PuttCLI {
    #[structopt(name = "FILE")]
    /// File to read
    path: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn Error>> {
    let putt = PuttCLI::from_args();

    // Load file or open REPL
    if let Some(path) = putt.path {
        let mut file = File::open(path)?;
        let mut fstring = String::new();
        file.read_to_string(&mut fstring)?;

        let output = eval_from_str(&fstring)?;
        println!("{:?}", output)

    }
    else {
        print!("> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        // Start reading lines
        for line in stdin.lock().lines() {
            // TODO: Parse tokens
            println!("{}", line.unwrap());
            print!("> ");
            io::stdout().flush()?;
        }
    }

    Ok(())
}
