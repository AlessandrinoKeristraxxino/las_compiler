// crate/src/codegen/wasm.rs

use crate::parser::ast::{Expr, Stmt};
use crate::codegen::glue::GlueFunc;

#[derive(Debug, PartialEq, Clone)]
pub struct WasmCodegen {
    statements: Vec<Stmt>,
    code: String,
    current: usize,
    used_func: Vec<GlueFunc>,
    declared_locals: Vec<String>,
}

impl WasmCodegen {
    pub fn new(statements: &Vec<Stmt>) -> Self {
        Self {
            statements: statements.clone(),
            code: String::new(),
            current: 0,
            used_func: Vec::new(),
            declared_locals: Vec::new(),
        }
    }

    fn generate_statement(&mut self) {
        match &self.statements[self.current] {
            Stmt::Let {
                name,
                value,
            } => {
                if let Expr::Number(num) = value {
                    if !self.declared_locals.contains(name) {
                        self.declared_locals.push(name.clone());
                    }

                    let code_str = format!("
                        (local.set ${} (i64.const {}))",
                        name, num
                    );

                    self.code.push_str(&code_str);
                }
            },
            Stmt::Print(identifier) => {
                if let Expr::Variable(name) = identifier {
                    let code_str = format!("
                        (call $print_num (local.get ${}))",
                        name
                    );

                    self.code.push_str(&code_str);
                } else if let Expr::Number(value) = identifier {
                    let code_str = format!("
                        (call $print_num (i64.const {}))",
                        value
                    );

                    self.code.push_str(&code_str);
                }

                self.used_func.push(GlueFunc::PrintNum);
            },
            Stmt::Println(expression) => {
                if let Some(expression) = expression {
                    if let Expr::Variable(name) = expression {
                        let code_str = format!("
                        (call $println_num (local.get ${}))",
                            name
                        );

                        self.code.push_str(&code_str);
                    } else if let Expr::Number(value) = expression {
                        let code_str = format!("
                        (call $println_num (i64.const {}))",
                            value
                        );

                        self.code.push_str(&code_str);
                    }

                    self.used_func.push(GlueFunc::PrintlnNum);
                } else {
                    self.code.push_str("
                        (call $println)");
                    self.used_func.push(GlueFunc::Println);
                }
            }
        }
    }

    pub fn generate_code(&mut self) -> (String, Vec<GlueFunc>) {
        let header = "(module\n\
            (import \"env\" \"print_num\" (func $print_num (param i64)))\n\
            (import \"env\" \"println_num\" (func $println_num (param i64)))\n\
            (import \"env\" \"println\" (func $println))\n\
            (func $main\n";
        let footer = ")\n(export \"main\" (func $main))\n)";

        while self.current < self.statements.len() {
            self.generate_statement();
            self.current += 1;
        }

        let locals_decl: String = self.declared_locals
            .iter()
            .map(|name| format!("(local ${} i64)\n", name))
            .collect();

        self.code = format!("{}{}{}{}", header, locals_decl, self.code, footer);

        (self.code.clone(), self.used_func.clone())
    }
}