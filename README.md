# reflex
A simple `flex`-like lexing/tokenizing library written in Rust

Workflow:

- Define a `Token` enum with all possible tokens
- Create a `Lexer` and add rules
- Call your `Lexer`'s `tokenize` method with a string to tokenize

Example code for tokenizing a simple calculator language:

```rust
use reflex::Lexer;

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(f64),
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPow,
}

fn main() {
    let mut program = String::new();
    std::io::stdin().read_line(&mut program).unwrap();

    let mut lexer = Lexer::new();
    lexer.rule(r"-?[0-9]*\.?[0-9]+", |mat| Token::Number(mat.as_str().parse().unwrap())).unwrap()
         .rule_simple(r"\+", Token::OpAdd).unwrap()
         .rule_simple(r"-", Token::OpSub).unwrap()
         .rule_simple(r"\*", Token::OpMul).unwrap()
         .rule_simple(r"/", Token::OpDiv).unwrap()
         .rule_simple(r"\^", Token::OpPow).unwrap()
         .rule_noop(r"(?s)\s").unwrap();

    for token in lexer.tokenize(&program) {
        println!("{:?}", token.unwrap());
    }
}
```
