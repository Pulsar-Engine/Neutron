use std::env;
use std::fs;

use neutron::lexer::Lexer;
use neutron::parser::Parser;
use neutron::semantic::analyze;
use neutron::symbol_table::SymbolTable;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: neutron <source_file.neutron>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .expect("Could not read source file.");

    let mut parser = Parser::new(Lexer::new(&source));
    let ast = parser.parse_program();

    let mut symbol_table = SymbolTable::new();
    analyze(&ast, &mut symbol_table);

    println!("✅ Program is valid!");
}
