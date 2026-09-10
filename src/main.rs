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
    }, io::{self, Read, Write}, net::TcpListener
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
                    Arg::new("project_name")
                        .required(true)
                        .help("Name of the project")
                ),
        )
        .subcommand(
            Command::new("build")
            .about("Build your LampScript project")
        )
        .subcommand(
            Command::new("serve")
            .about("Build and serve your LampScript project")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let project_name = sub_matches.get_one::<String>("project_name").unwrap();
            
            let _ = fs::create_dir_all(project_name);
            let _ = env::set_current_dir(format!("{}", project_name));

            let _ = fs::create_dir_all("src")?;
            let _ = fs::write("src/main.las", "let x = 10;\nprintln!(x);\nprintln!(10);")?;
            let _ = fs::write(
                "config.lasd",
                format!(
                    "load std::config::Project;\n\n
                    Project = {{ name: {} }}",
                    project_name
                ),
            )?;
            let _ = fs::write("README.md", "");
            let _ = fs::write(".gitignore", "/target");
        },
        Some(("build", _sub_matches)) => {
            compile()?;
            println!("Compilation completed successfully")
        },
        Some(("serve", _sub_matches)) => {
            compile()?;
            serve()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn compile() -> io::Result<()> {
    let code_str = read_to_string("src/main.las")?;
    let source_code: Vec<char> = code_str.chars().collect();

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.lexing();

    let mut parser = Parser::new(&tokens);
    let statements = parser.parse();

    let mut wasm_codegen = WasmCodegen::new(&statements);
    let (wat_code, used_func) = wasm_codegen.generate_code();
    let wasm_code = wat::parse_str(&wat_code)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    let mut glue_codegen = GlueCodegen::new(used_func);
    let glue_code = glue_codegen.generate_code();

    let _ = fs::create_dir_all("target")?;
    let _ = env::set_current_dir("target")?;

    let _ = fs::write("main.wasm", wasm_code)?;
    let _ = fs::write("glue.js", glue_code)?;
    let _ = fs::write("index.html", "<DOCTYPE html><html><head><title>Simple LAS Page</title></head><body><script src=\"glue.js\"></script></body></html>")?;

    Ok(())
}

fn serve() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Serving the project at http://127.0.0.1:8000");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut request = [0; 1024];
        stream.read(&mut request)?;

        let request_line = String::from_utf8_lossy(&request);
        let path = request_line
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        let file_name = match path {
            "/" | "/index.html" => "index.html",
            "/glue.js" => "glue.js",
            "/main.wasm" => "main.wasm",
            "/favicon.ico" => {
                write_response(&mut stream, "204 No Content", "image/x-icon", &[])?;
                continue;
            }
            _ => {
                write_response(&mut stream, "404 Not Found", "text/plain", b"Not Found")?;
                continue;
            }
        };

        match fs::read(file_name) {
            Ok(contents) => {
                let content_type = match file_name {
                    "main.wasm" => "application/wasm",
                    "glue.js" => "text/javascript; charset=utf-8",
                    _ => "text/html; charset=utf-8",
                };
                write_response(&mut stream, "200 OK", content_type, &contents)?;
            }
            Err(_) => {
                write_response(&mut stream, "404 Not Found", "text/plain", b"Not Found")?;
            }
        }
    }

    Ok(())
}

fn write_response(
    stream: &mut std::net::TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)
}