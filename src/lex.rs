use regex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenTy {
    Whitespace,
    Ident,
    Number,
    LParen,
    RParen,
    Comma,
    Operator,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub ty: TokenTy,
}

struct Matcher {
    re: regex::Regex,
    ty: TokenTy,
    keep: bool,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let matchers = build_matchers();

    let mut tokens = vec![];
    let mut idx = 0;
    while idx < input.len() {
        let mut found_match = false;
        for matcher in &matchers {
            let match_res = matcher.re.find(&input[idx..]);
            match match_res {
                Some(x) => {
                    let tok = Token {
                        text: String::from(x.as_str()),
                        ty: matcher.ty,
                    };
                    idx += tok.text.len();
                    if matcher.keep {
                        tokens.push(tok);
                    }
                    found_match = true;
                    break;
                }
                _ => continue,
            }
        }
        if !found_match {
            return Err(format!("Could not parse token at idx: {}", idx));
        };
    }
    Ok(tokens)
}

#[allow(clippy::trivial_regex)]
fn build_matchers() -> Vec<Matcher> {
    return vec![
        Matcher {
            re: regex::Regex::new(r"^[\s\n]+").unwrap(),
            ty: TokenTy::Whitespace,
            keep: false,
        },
        Matcher {
            re: regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap(),
            ty: TokenTy::Ident,
            keep: true,
        },
        Matcher {
            re: regex::Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap(),
            ty: TokenTy::Number,
            keep: true,
        },
        Matcher {
            re: regex::Regex::new(r"^\(").unwrap(),
            ty: TokenTy::LParen,
            keep: true,
        },
        Matcher {
            re: regex::Regex::new(r"^\)").unwrap(),
            ty: TokenTy::RParen,
            keep: true,
        },
        Matcher {
            re: regex::Regex::new(r"^,").unwrap(),
            ty: TokenTy::Comma,
            keep: true,
        },
        Matcher {
            re: regex::Regex::new(r"^[+\-\*/]").unwrap(),
            ty: TokenTy::Operator,
            keep: true,
        },
    ];
}
