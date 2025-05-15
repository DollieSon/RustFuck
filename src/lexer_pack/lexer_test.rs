#[cfg(test)]
pub mod lexer_tests {
    use super::super::lexer::Lexer;
    use super::super::lex_tokens::{LextToken,TokenType};
    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("".to_string());
        let tokens = lexer.analize();
        assert!(tokens.is_empty(), "Expected no tokens for empty input");
    }

    #[test]
    #[should_panic]
    fn test_single_character_input() {
        let mut lexer = Lexer::new("a".to_string());
        let _ = lexer.analize();
    }

    #[test]
    fn test_multiple_characters_input() {
        let mut lexer = Lexer::new("<>+-.,[]".to_string());
        let tokens = lexer.analize();
        assert_eq!(tokens.len(), 8, "Should be 8");
        let mut iter = tokens.iter();
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::MOVE_L, line_num:0});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::MOVE_R, line_num:1});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::INCREMENT, line_num:2});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::DECREMENT, line_num:3});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::OUTPUT, line_num:4});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::INPUT, line_num:5});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::BRACKET_O, line_num:6});
        assert_eq!(*iter.next().unwrap(),LextToken{token: TokenType::BRAKET_C, line_num:7});
    }
}