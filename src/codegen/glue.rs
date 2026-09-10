// crate/src/codegen/glue.rs

#[derive(Debug, PartialEq, Clone)]
pub enum GlueFunc {
    PrintNum,
    PrintlnNum,
    Println,
}

#[derive(Debug, PartialEq, Clone)]
struct UsedFunc {
    print_num: bool,
    println_num: bool,
    println: bool,
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
            println: false,
        };

        for func in &self.used_func {
            match func {
                GlueFunc::PrintlnNum => {
                    used_func.println_num = true;
                },
                GlueFunc::PrintNum => {
                    used_func.print_num = true;
                },
                GlueFunc::Println => {
                    used_func.println = true;
                }
            }
        }

        used_func
    }

    pub fn generate_code(&mut self) -> String {
        let used_func = self.check_used_func();

        let import_object = if !used_func.print_num && !used_func.println_num && !used_func.println {
            "const importObject = {};\n".to_string()
        } else {
            "const importObject = {\n  env: {\n    print_num: function(number) { console.log(number.toString()); },\n    println_num: function(number) { console.log(number.toString() + '\\n'); },\n    println: function() { console.log(''); }\n  }\n};\n".to_string()
        };

        self.code = import_object;
        self.code.push_str(
            "let memory;\n\
            WebAssembly.instantiateStreaming(fetch('main.wasm'), importObject)\n\
            .then(obj => {\n\
                memory = obj.instance.exports.memory;\n\
                obj.instance.exports.main();\n\
            });\n"
        );

        self.code.clone()
    }
}