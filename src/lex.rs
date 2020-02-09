use regex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
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
  pub ty: TokenType,
}

struct Matcher {
  re: regex::Regex,
  ty: TokenType,
  keep: bool,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
  let matchers: Vec<Matcher> = vec![
    Matcher {
      re: regex::Regex::new(r"^[\s\n]+").unwrap(),
      ty: TokenType::Whitespace,
      keep: false,
    },
    Matcher {
      re: regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap(),
      ty: TokenType::Ident,
      keep: true,
    },
    Matcher {
      re: regex::Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap(),
      ty: TokenType::Number,
      keep: true,
    },
    Matcher {
      re: regex::Regex::new(r"^\(").unwrap(),
      ty: TokenType::LParen,
      keep: true,
    },
    Matcher {
      re: regex::Regex::new(r"^\)").unwrap(),
      ty: TokenType::RParen,
      keep: true,
    },
    Matcher {
      re: regex::Regex::new(r"^,").unwrap(),
      ty: TokenType::Comma,
      keep: true,
    },
    Matcher {
      re: regex::Regex::new(r"^[+\-\*/]").unwrap(),
      ty: TokenType::Operator,
      keep: true,
    },
  ];

  let mut tokens = Vec::new();
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
  return Ok(tokens);
}
