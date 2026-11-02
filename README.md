# Neutron

[![Rust](https://github.com/Pulsar-Engine/Neutron/actions/workflows/rust.yml/badge.svg)](https://github.com/Pulsar-Engine/Neutron/actions/workflows/rust.yml)

Neutron is the scripting language of the [Pulsar](https://github.com/Pulsar-Engine) game engine, written in Rust. It is small and statically typed, with a keyword-based syntax meant to be easy to read for people new to programming.

```
class Demo then
    var total int
    var done bool

    func compute(a, b) then
        var sum int
        sum = (a + b) * 2
        if sum > 10 then
            sum = sum + 1
        else
            sum = sum - 1
        end
        ret sum
    end

    func run() then
        var i int
        i = 0
        total = 0
        done = false
        while done == false then
            total = total + i
            i = i + 1
            if i > 5 then
                done = true
            end
        end
        for j = 0 3
            total = total + j
        end
    end
end
```

## Language

- Types: `int`, `float`, `bool`, `string`
- Typed variable declarations (`var x int`) and assignments
- Classes and functions with parameters, `ret` to return a value
- `if` / `else`, `while`, `for i = start end`
- Arithmetic `+ - * /` with operator precedence and negative numbers, comparisons `< > ==`
- Nested blocks, closed with `end`

## Pipeline

A source file goes through four stages, each in its own module:

| Stage | File | What it does |
|---|---|---|
| Lexer | `src/lexer.rs` | Turns the source into tokens: keywords, identifiers, int and float literals, strings, operators |
| Parser | `src/parser.rs` | Recursive descent parser that builds the AST defined in `src/ast.rs`, handling operator precedence |
| Semantic analysis | `src/semantic.rs`, `src/symbol_table.rs` | Static type checking, undeclared variables, redeclarations in the same scope, boolean conditions, integer `for` bounds, unknown functions |
| Interpreter | `src/interpreter.rs` | Tree-walking interpreter: registers functions, then calls `run()` as the entry point |

Errors at any stage stop the program with a message saying what is wrong (type mismatch, undefined variable, ...).

## Usage

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/Pulsar-Engine/Neutron.git
cd Neutron
cargo build
cargo run -- path/to/program.neutron
```

## Tests

```bash
cargo test
```

Each stage has its own test file in `tests/`: lexer, parser, semantic analysis and symbol table. The CI runs the build and the tests on every push.

## Limits

- Errors are reported with `panic!`, without line or column numbers.
- No arrays, no class instances yet: classes group functions and variables but cannot be instantiated.
- The interpreter walks the AST directly. A bytecode VM would be the next step for performance inside the engine.
