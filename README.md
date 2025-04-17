# 🧠 Neutron — A Beginner-Friendly Programming Language for Game Development

**Neutron** is a small, statically typed programming language designed for simplicity and learning.  
It currently supports classes, functions, variables, arithmetic, conditions, loops, and semantic analysis.

---

## 🚀 Features

✅ Lexer (tokenizer)  
✅ Parser (AST generation)  
✅ Semantic analysis (type checking, scope management)  
✅ Support for:
- `int`, `float`, `bool`, `string`
- Variable declarations and assignments
- `if / else`
- `while`, `for`
- `return`
- Operators: `+`, `-`, `*`, `/`, `<`, `>`, `==`
- Classes and functions
- Nested control structures

---

## 🛠️ Project Structure

neutron/ ├── Cargo.toml # Rust project manifest ├── README.md # You're reading it! ├── examples/ # Sample Neutron programs │ └── demo.neutron ├── src/ # Source files of the Neutron compiler frontend │ ├── ast.rs # Abstract Syntax Tree definitions │ ├── lexer.rs # Tokenizer (lexical analysis) │ ├── parser.rs # Parser (builds the AST) │ ├── semantic.rs # Semantic analysis (type checking, scopes, etc.) │ ├── symbol_table.rs # Symbol table for variables/functions │ ├── lib.rs # Library entry point (exports modules) │ └── main.rs # CLI: validate and analyze .neutron programs ├── tests/ # Unit and integration tests │ ├── lexer_tests.rs │ ├── parser_tests.rs │ └── semantic_tests.rs

---

## 📦 Getting Started

### 🔧 Requirements
- [Rust](https://www.rust-lang.org/tools/install)

### 📥 Clone the repo
```bash
git clone https://github.com/yourusername/neutron.git
cd neutron

🔄 Build the project
cargo build

✅ Run all tests
cargo test

▶️ Run a .neutron file
cargo run -- examples/demo.neutron

🧪 Run the development main (AST debug mode)
cargo run --bin test_main

📄 Example: demo.neutron

class Demo then
    var result int
    var ready bool
    var scale float

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
        var total int
        var done bool

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

        result = total
        ready = true
        scale = 1.5
    end
end

