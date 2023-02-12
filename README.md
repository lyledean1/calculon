# Calculon

Simple PoC using LALRPOP and llvm-sys for my own learning:
- To parse a grammar, which in this case is a simple one line arithmetic i.e `((5*5) + 2)` using [LALRPOP](https://crates.io/crates/lalrpop) taken from this [tutorial](http://lalrpop.github.io/lalrpop/tutorial/004_full_expressions.html)
- To then compile that to LLVM IR with [llvm-sys](https://crates.io/crates/llvm-sys)
- To generate that from LLVM IR to a native executable using clang 

```mermaid
flowchart LR;
    CalculonLanguage-->Parser;
    Parser-->|LALRPOP| AST;
    AST-->CalculonCalculations;
    CalculonCalculations-->|llvm-sys| LLVM-IR;
    LLVM-IR-->|clang| Binary;
```

## Run

Some examples are in the example folder, just run 

```
cargo run example/calculator.calculon
```


