use crate::ast::{Expr, Operator};
use crate::lex::{lex, Token, TokenTy};

pub struct Ctx {
    tokens: Vec<Token>,
    idx: usize,
}

pub fn parse(code: &str) -> Result<Expr, String> {
    let tokens = lex(code)?;
    let mut ctx = Ctx { tokens, idx: 0 };
    parse_bin_op_or_expr(&mut ctx)
}

fn peek(ctx: &mut Ctx) -> Token {
    if ctx.idx >= ctx.tokens.len() {
        return Token {
            text: String::from(""),
            ty: TokenTy::EOF,
        };
    }
    ctx.tokens[ctx.idx].clone()
}

fn consume(ctx: &mut Ctx) -> Token {
    let tok = peek(ctx);
    ctx.idx += 1;
    tok
}

// TODO all these should be Result
fn expect(ctx: &mut Ctx, ty: TokenTy) -> Result<Token, String> {
    let tok = consume(ctx);
    if tok.ty != ty {
        return Err(format!("expected: {:?}, found {:?}", ty, tok.ty));
    }
    Ok(tok)
}

fn parse_bin_op_or_expr(ctx: &mut Ctx) -> Result<Expr, String> {
    // here we parse an expression, then look ahead to see if there's a operator after, and keep consuming expressions and operators until we've got em alloc
    // for now we'll just stuff all operators into the left
    // a+b+c+d -> (((a+b)+c)+d)
    // TODO: do precidence and associativity as a post processing step at the end of this fn.

    let mut expr = parse_expr(ctx)?;
    while peek(ctx).ty == TokenTy::Operator {
        let op_token = consume(ctx);
        let op = match op_token.text.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => return Err(format!("unsupported operator: {}", op_token.text)),
        };
        let a = Box::new(expr);
        let b = Box::new(parse_expr(ctx)?);
        expr = Expr::BinOp { op, a, b }
    }
    Ok(expr)
}

fn parse_expr(ctx: &mut Ctx) -> Result<Expr, String> {
    match peek(ctx).ty {
        TokenTy::Ident => parse_call(ctx),
        TokenTy::Number => parse_num(ctx),
        TokenTy::LParen => parse_paren_expr(ctx),
        t => Err(format!("unexpected token: {:?}", t)),
    }
}

fn parse_paren_expr(ctx: &mut Ctx) -> Result<Expr, String> {
    expect(ctx, TokenTy::LParen)?;
    let expr = parse_bin_op_or_expr(ctx);
    expect(ctx, TokenTy::RParen)?;

    expr
}

fn parse_call(ctx: &mut Ctx) -> Result<Expr, String> {
    let tok = expect(ctx, TokenTy::Ident)?;
    expect(ctx, TokenTy::LParen)?;
    let args = parse_args(ctx)?;
    expect(ctx, TokenTy::RParen)?;

    Ok(Expr::Call {
        target: tok.text,
        args,
    })
}

fn parse_num(ctx: &mut Ctx) -> Result<Expr, String> {
    let tok = expect(ctx, TokenTy::Number)?;
    Ok(Expr::Num {
        val: tok.text.parse::<f64>().unwrap(),
    })
}

fn parse_args(ctx: &mut Ctx) -> Result<Vec<Expr>, String> {
    let mut args = vec![];

    while peek(ctx).ty != TokenTy::RParen {
        let arg = parse_bin_op_or_expr(ctx)?;
        args.push(arg);

        if peek(ctx).ty == TokenTy::Comma {
            consume(ctx);
        }
    }

    Ok(args)
}
