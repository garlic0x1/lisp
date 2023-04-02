pub mod value;
pub mod core;
pub mod environment;
pub mod eval;
use crate::value::Value::*;
use crate::{environment::*, value::*};
use anyhow::{Result, bail};
use edn_rs::Edn;
use eval::eval;
use std::{str::FromStr, io::Write};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Arguments {
    /// Run a file
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// evaluate a string
    Eval { lisp: String },
    /// run a file as a program
    Run { file: String },
    /// start a REPL
    Repl,
}

// crude repl for testing
// input must be one line
fn repl() -> Result<()> {
    let pipe = std::io::stdin();
    let mut env = Env::from_core(core::core());

    loop {
        print!("{}", "lisp> ");
        std::io::stdout().lock().flush()?;
        let mut line = String::new();
        pipe.read_line(&mut line)?;

        if line == ",quit".to_string() {
            break;
        }

        match Edn::from_str(&line) {
            Ok(edn) => {
                match eval(&Expr(edn), &mut env) {
                    Ok(res) => println!(";; => {}", res),
                    Err(err) => println!("{}", err),
                }
            },
            Err(err) => println!("{}", err),
        };
    }

    Ok(())
}

fn eval_str(lisp: &str) -> Result<()> {
    let mut env = Env::from_core(core::core());
    let res = eval(&Expr(Edn::from_str(lisp)?), &mut env)?;
    println!("{}", res.to_string());
    Ok(())
}

fn run(filename: &str) -> Result<()> {
    let mut env = Env::from_core(core::core());
    let lisp = format!("(load \"{}\")", filename);
    match eval(&Expr(Edn::from_str(&lisp)?), &mut env) {
        Ok(res) => println!(";; => {}", res),
        Err(err) => bail!(err),
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    match &args.command {
        Commands::Eval { lisp } => eval_str(lisp),
        Commands::Run { file } => run(file),
        Commands::Repl => repl(),
    }
}
