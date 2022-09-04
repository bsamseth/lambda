# Lambda Calculus Interpreter

A(nother) Lambda calculus interpreter, implemented in Rust. Because what better way is
there to learn a programming language, than to implement the best language there is in it.

Eventually I want to add a compiler, so that you can run highly optimized Lambda
calculus programs. That's for a future improvement though. 


## Run

Pass the expression to evaluate to `evaluate` and watch as it transforms into its
smallest form. Variables are normalized and uses the numbers `1`, `2`, etc.

You can use a backslash (`\`) to represent λ if you don't have a λ handy. Or use a λ if
you do. It handles either.

```
$ cargo build --release
$ ./target/release/evaluate '(\x.λy. x y) (λx.x)'
'(\x.λy. x y) (λx.x)' -> λ1.1
```

