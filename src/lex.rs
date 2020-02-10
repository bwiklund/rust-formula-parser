#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenTy {
    // Whitespace,
    Ident,
    Number,
    LParen,
    RParen,
    Comma,
    Operator,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub text: &'a str,
    pub ty: TokenTy,
}

// A token contains a slice of the input text. As such we can
// declare that the token's lifetime will not outlive that of
// the input text - this is the behavior of the lifetime `'a`
pub fn lex<'a>(input: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut tokens = vec![];
    let mut char_it = input.chars().enumerate().peekable();
    loop {
        match char_it.next() {
            Some((idx, ch)) => match ch {
                // whitespace
                ' ' | '\n' => continue,

                // ident
                'a'..='z' | 'A'..='Z' => {
                    let mut end_idx = idx;
                    loop {
                        match char_it.peek() {
                            Some((inner_idx, ch)) => match ch {
                                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                    char_it.next();
                                }
                                _ => {
                                    end_idx = *inner_idx - 1;
                                    break;
                                }
                            },
                            None => break,
                        }
                    }

                    tokens.push(Token {
                        ty: TokenTy::Ident,
                        text: &input[idx..end_idx + 1],
                    });
                }

                // number // TODO decimal point and tail
                '0'..='9' => {
                    let mut end_idx = idx;
                    loop {
                        match char_it.peek() {
                            Some((inner_idx, ch)) => match ch {
                                '0'..='9' => {
                                    char_it.next();
                                }
                                _ => {
                                    end_idx = *inner_idx - 1;
                                    break;
                                }
                            },
                            None => break,
                        }
                    }

                    tokens.push(Token {
                        ty: TokenTy::Number,
                        text: &input[idx..end_idx + 1],
                    });
                }

                '(' => tokens.push(Token {
                    ty: TokenTy::LParen,
                    text: &input[idx..idx + 1],
                }),

                ')' => tokens.push(Token {
                    ty: TokenTy::RParen,
                    text: &input[idx..idx + 1],
                }),

                ',' => tokens.push(Token {
                    ty: TokenTy::Comma,
                    text: &input[idx..idx + 1],
                }),

                '+' | '-' | '*' | '/' => tokens.push(Token {
                    ty: TokenTy::Operator,
                    text: &input[idx..idx + 1],
                }),

                _ => return Err(format!("Could not parse token at idx: {}", idx)),
            },

            None => {
                break;
            }
        }
    }
    Ok(tokens)
}
