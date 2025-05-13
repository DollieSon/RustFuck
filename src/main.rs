pub mod lexer_pack;

use lexer_pack::lexer::Lexer;
fn main() {
    let input = "....".to_string();
    let mut lex = Lexer::new(input);
    lex.analize();
    println!("Hello, world!");

}
