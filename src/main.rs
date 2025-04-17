mod ast;
mod lexer;
mod parser;
mod semantic;
mod symbol_table;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source_code = std::fs::read_to_string("integration_test.neutron")
    .expect("Could not read test file");

    println!("🧾 Source code:\n{}\n", source_code);

    // 🔍 LEXER
    println!("🔍 Tokens:");
    let mut lexer = Lexer::new(&source_code);
    loop {
        let token = lexer.get_next_token();
        if token == lexer::Token::EOF {
            break;
        }
        println!("{:?}", token);
    }

    // 🌳 PARSER
    let mut parser = Parser::new(Lexer::new(&source_code));
    let ast = parser.parse_program();

    println!("\n🌳 AST:");
    println!("{:#?}", ast);

    // 🧠 SEMANTIC ANALYSIS (avec catch_unwind)
    println!("\n🧠 Semantic Analysis:");
    let analysis_result = std::panic::catch_unwind(|| {
        let mut symbol_table = symbol_table::SymbolTable::new();
        semantic::analyze(&ast, &mut symbol_table);
        symbol_table
    });

    match analysis_result {
        Ok(symbol_table) => {
            println!("✅ Semantic analysis passed");
            println!("\n📦 Symbol Table:");
            println!("{:#?}", symbol_table);
        }
        Err(e) => {
            println!("❌ Semantic error: {:?}", e);
        }
    }
}
