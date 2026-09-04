(module
    (import "env" "print_str" (func $print_str (param i32 i32)))
    (memory (export "memory") 1)
    (data (i32.const 1) "Hello World")

    (func $main 
        (call $print_str (i32.const 1) (i32.const 10))
    )

    (export "main" (func $main))
)