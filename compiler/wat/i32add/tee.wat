(module
    (func (result i32)
        (local $foo i32)
        (local $bar i32)

        (i32.const 10)
        (local.tee $foo)

        (i32.const 20)
        (local.set $bar)

        (local.get $bar)
        i32.add
    )
)
