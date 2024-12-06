mod lexer;
mod parser;
mod ast;
mod semantic;
mod symbol_table;

use lexer::Lexer;
use parser::Parser;
use symbol_table::SymbolTable;

fn main() {
    let source_code = "
        class MyClass then
            var x
            func myFunc then
                x = 42
            end
        end
    ";

    let mut lexer = Lexer::new(source_code);
    let _tokens = {
        let mut _tokens = Vec::new();
        while let token = lexer.get_next_token() {
            if token == lexer::Token::EOF {
                break;
            }
            _tokens.push(token);
        }
        _tokens
    };

    let mut parser = Parser::new(Lexer::new(source_code));
    let ast = parser.parse_program();

    let mut symbol_table = SymbolTable::new();
    semantic::analyze(&ast, &mut symbol_table);

    println!("{:#?}", ast);
    println!("{:#?}", symbol_table);
}