pub mod lexer_pack;
pub mod parser_pack;
pub mod eval_pack;
use lexer_pack::lexer::Lexer;
use parser_pack::parser::{Evaluateable, Parser};

fn main() {
    let input = ".[++].".to_string();
    let mut lex = Lexer::new(input);
    let lex_res = lex.analize();
    println!("lexing done : {}",lex_res.len());
    println!("{:?}",lex_res);
    let mut parser = Parser::new(lex_res);
    let parser_res = parser.evaluate();
    parser_res.print_self();
    println!("Hello, world!");

}
