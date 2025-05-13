#[cfg(test)]
pub mod lexer_tests {
    use super::super::lexer::Lexer;
    use super::super::lex_tokens::LextToken;

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
        let tokens = lexer.analize();
    }

    #[test]
    fn test_multiple_characters_input() {
        let mut lexer = Lexer::new("...".to_string());
        let tokens = lexer.analize();
        assert_eq!(tokens.len(), 3, "Should be three tokns");
        assert_eq!(tokens[0], LextToken::get_token('.', 0), "Token Reading Error");
        assert_eq!(tokens[1], LextToken::get_token('.', 1), "Token Reading Error for 1");
    }
}