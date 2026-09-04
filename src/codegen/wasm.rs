// crate/src/codegen/wasm.rs

use crate::parser::ast::{Expr, Stmt};
use crate::codegen::glue::GlueFunc;

#[derive(Debug, PartialEq, Clone)]
pub struct WasmCodegen {
    statements: Vec<Stmt>,
    code: String,
    current: usize,
    used_func: Vec<GlueFunc>,
}

impl WasmCodegen {
    pub fn new(statements: &Vec<Stmt>) -> Self {
        Self {
            statements: statements.clone(),
            code: String::new(),
            current: 0,
            used_func: Vec::new(),
        }
    }

    fn generate_statement(&mut self) {
        match &self.statements[self.current] {
            Stmt::Let {
                name,
                value,
            } => {
                if let Expr::Number(num) = value {
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
                        (call $print_num (local.get {}))",
                        name
                    );

                    self.used_func.push(GlueFunc::PrintNum);
                    self.code.push_str(&code_str);
                }
            },
            Stmt::Println(identifier) => {
                if let Expr::Variable(name) = identifier {
                    let code_str = format!("
                        (call $println_num (local.get {}))",
                        name
                    );

                    self.used_func.push(GlueFunc::PrintlnNum);
                    self.code.push_str(&code_str);
                }
            }
        }
    }

    pub fn generate_code(&mut self) -> (String, Vec<GlueFunc>) {
        let first_code = format!("
            (module\n
                (import \"env\" \"print_num\" (func $print_num (param i64)))\n
                (import \"env\" \"printnl_num\" (func $println_num (param i64)))\n

                (func $main\n
            "
        );
        let final_code = format!("
                )\n
                (export \"main\" (func $main))\n
            )"
        );
        
        self.code.push_str(&first_code);
        while self.current <= self.statements.len() {
            self.generate_statement();
            self.current += 1;
        }
        self.code.push_str(&final_code);

        (self.code.clone(), self.used_func.clone())
    }
}