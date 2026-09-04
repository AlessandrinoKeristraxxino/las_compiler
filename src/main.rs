// crate/src/main.rs

mod lexer;
mod parser;
mod codegen;
mod errors;

use lexer::tokenizer::Lexer;
use parser::ast::Parser;
use codegen::wasm::WasmCodegen;

use clap::{Arg, Command};
use std::{env, fs::{
        self,
        read_to_string,
    }, io
};

use crate::codegen::glue::GlueCodegen;

fn main() -> io::Result<()> {
    let matches = Command::new("las")
        .version("1.0.0")
        .author("Alessandro Napoli - alessandro.luigi.napoli@gmail.com")
        .about("CLI for LampScript")
        
        .subcommand(
            Command::new("new")
                .about("Create a new LampScript project")
                .arg(
                    Arg::new("Project Name")
                        .required(true)
                        .help("Name of the project")
                ),
        )
        .subcommand(
            Command::new("build")
            .about("Build your LampScript project")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            fs::write("main.las", "let x = 10;\nprintln!(x);")?;
            fs::write(
                "config.lasd",
                format!(
                    "project = {{ name: {} }}",
                    sub_matches.get_one::<String>("Project Name").unwrap()
                ),
            )?;
            let _ = fs::write("README.md", "");
            let _ = fs::write(".gitignore", "/target");
        },
        Some(("build", _sub_matches)) => {
            let _ = compile();
            println!("Compilation completed succeffully")
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn compile() -> io::Result<()> {
    let code_str = read_to_string("test/main.las")?;
    let source_code: Vec<char> = code_str.chars().collect();

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.lexing();

    let mut parser = Parser::new(&tokens);
    let statements = parser.parse();

    let mut wasm_codegen = WasmCodegen::new(&statements);
    let (wasm_code, used_func) = wasm_codegen.generate_code();

    let mut glue_codegen = GlueCodegen::new(used_func);
    let glue_code = glue_codegen.generate_code();

    let _ = fs::create_dir_all("target")?;
    let _ = env::set_current_dir("target")?;

    let _ = fs::write("main.wasm", wasm_code)?;
    let _ = fs::write("glue.js", glue_code)?;
    let _ = fs::write("index.html", "<DOCTYPE html><html><head><title>Simple LAS Page</title></head><body><script src=\"glue.js\"></script></body></html>")?;

    Ok(())
}