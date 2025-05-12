pub mod lexer_pack;

use lexer_pack::lexer::Lexer;
fn main() {
    let input = "Hello".to_string();
    let lex = Lexer::new(input);
    println!("Hello, world!");

}
