(module
    (import "env" "print_str" (func $print_str (param i32 i32)))
    (import "env" "print_num" (func $print_num (param i64)))

    (memory (export "memory") 1)
    (data (i32.const 0) "Operatore non valido")
    (data (i32.const 21) "Fine del programma")

    (func $calculate (param $op i32) (param $a i64) (param $b i64) (result i64)
        local.get $op
        i32.const 0
        i32.eq
        if (result i64)
            local.get $a
            local.get $b
            i64.add
        else
            local.get $op
            i32.const 1
            i32.eq
            if (result i64)
                local.get $a
                local.get $b
                i64.sub
            else
                local.get $op
                i32.const 2
                i32.eq
                if (result i64)
                    local.get $a
                    local.get $b
                    i64.mul
                else
                    local.get $op
                    i32.const 3
                    i32.eq
                    if (result i64)
                        local.get $a
                        local.get $b
                        i64.div_s
                    else
                        i32.const 0
                        i32.const 20
                        call $print_str
                        i64.const 0
                    end
                end
            end
        end
    )

    (func $main
        (local $result i64)

        (local.set $result
            (call $calculate (i32.const 0) (i64.const 10) (i64.const 12))
        )
        
        (call $calculate (i32.const 0) (i64.const 10) (i64.const 12))
        drop

        (call $print_num (local.get $result))
        (drop (local.get $result))

        (local.set $result
            (call $calculate (i32.const 2) (i64.const 13) (i64.const 7))
        )
        (call $print_num (local.get $result))
        (drop (local.get $result))
    )

    (export "calculate" (func $calculate))
    (export "main" (func $main))
)
