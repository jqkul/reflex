extern crate reflex;

use reflex::Lexer;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    Number(f64),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
    ParenL,
    ParenR,
}

#[test]
fn calculator() -> Result<(), regex::Error> {
    let program = "(3.45 + 3) * (-7 - .8) ^ (1.3 / -.5)".to_string();
    let expected = vec![
        Token::ParenL, Token::Number(3.45), Token::OpAdd, Token::Number(3.0), Token::ParenR,
        Token::OpMul, Token::ParenL, Token::Number(-7.0), Token::OpSub, Token::Number(0.8), Token::ParenR,
        Token::OpPow, Token::ParenL, Token::Number(1.3), Token::OpDiv, Token::Number(-0.5), Token::ParenR
    ];

    let mut lexer = Lexer::new();
    lexer.rule(r"-?[0-9]*\.?[0-9]+", |mat| Token::Number(mat.as_str().parse().unwrap()))?
         .rule_simple(r"\+", Token::OpAdd)?
         .rule_simple(r"-", Token::OpSub)?
         .rule_simple(r"\*", Token::OpMul)?
         .rule_simple(r"/", Token::OpDiv)?
         .rule_simple(r"\^", Token::OpPow)?
         .rule_simple(r"\(", Token::ParenL)?
         .rule_simple(r"\)", Token::ParenR)?
         .rule_noop(r"(?s)\s+")?;
    
    let mut i = 0;
    for token in lexer.tokenize(&program) {
        assert_eq!(token.unwrap(), expected[i]);
        i += 1;
    }

    Ok(())
}
