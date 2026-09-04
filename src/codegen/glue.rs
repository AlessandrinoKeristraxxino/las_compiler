// crate/src/codegen/glue.rs

#[derive(Debug, PartialEq, Clone)]
pub enum GlueFunc {
    PrintNum,
    PrintlnNum,
}

#[derive(Debug, PartialEq, Clone)]
struct UsedFunc {
    print_num: bool,
    println_num: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlueCodegen {
    code: String,
    used_func: Vec<GlueFunc>,
}

impl GlueCodegen {
    pub fn new(used_func: Vec<GlueFunc>) -> Self {
        Self {
            code: String::new(),
            used_func,
        }
    }

    fn check_used_func(&mut self) -> UsedFunc {
        let mut used_func = UsedFunc {
            print_num: false,
            println_num: false,
        };

        for func in &self.used_func {
            match func {
                GlueFunc::PrintlnNum => {
                    used_func.println_num = true;
                },
                GlueFunc::PrintNum => {
                    used_func.print_num = true;
                }
            }
        }

        used_func
    }

    pub fn generate_code(&mut self) -> String {
        let used_func = self.check_used_func();

        if used_func.print_num == false && used_func.println_num == false {
            self.code = format!("
                const importObject = {{}};\n"
            );
        } else {
            self.code = format!("
                const importObject = {{\n
                    env: {{\n
                        print_num: function(number) {{ console.log(number.to_string()) }},\n
                        println_num: function(number) {{ console.log(`${{number.to_string()}}\\n`) }}\n
                    }}\n
                }};\n"
            );
        }

        self.code.push_str(&format!("
            WebAssembly.instantiateStreaming(fetch('main.wasm', importObject)\n
                .then(obj => {{\n
                    memory = obj.instance.exports.memory;\n
                    obj.instance.exports.main();\n
                }});"
        ));

        self.code.clone()
    }
}