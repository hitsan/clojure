#[derive(Debug, PartialEq)]
enum Token {
    LBrace,
    RBrace,
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, PartialEq)]
struct Lexed<'a> {
    token: Token,
    rest: &'a str,
}
impl<'a> Lexed<'a> {
    pub fn new(token: Token, rest: &'a str) -> Self {
        Lexed{token, rest}
    }
}

fn number(code: &str) -> Option<Lexed> {
    let mut chars = code.chars();
    let index = chars.position(|c| !c.is_numeric()).unwrap_or(code.len());
    if index == 0 { return None }
    let num = &code[..index];
    let rest = &code[index..];
    let num = num.parse::<i32>();
    match num {
        Ok(n) => Some(Lexed::new(Token::Number(n), rest)),
        Err(_) => None
    }
}

fn l_brace(code: &str) -> Option<Lexed> {
    char(code, '(', Token::LBrace)
}

fn r_brace(code: &str) -> Option<Lexed> {
    char(code, ')', Token::RBrace)
}

fn plus(code: &str) -> Option<Lexed> {
    char(code, '+', Token::Plus)
}

fn minus(code: &str) -> Option<Lexed> {
    char(code, '-', Token::Minus)
}

fn asterisk(code: &str) -> Option<Lexed> {
    char(code, '*', Token::Asterisk)
}

fn slash(code: &str) -> Option<Lexed> {
    char(code, '/', Token::Slash)
}

fn char(code: &str, target: char, token: Token) -> Option<Lexed> {
    let mut chars = code.chars();
    let next = chars.next();
    match next {
        Some(target) => Some(Lexed::new(token, &chars.as_str())),
        _ => None
    }
}

pub fn lex(code: &str) {
    println!("{}", code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l_brace() {
        let test = "()";
        let expect = Some(Lexed::new(Token::LBrace, &")"));
        assert_eq!(l_brace(&test), expect);
    }

    #[test]
    fn test_r_brace() {
        let test = "))";
        let expect = Some(Lexed::new(Token::RBrace, &")"));
        assert_eq!(r_brace(&test), expect);
    }

    #[test]
    fn test_number() {
        let test = "123c";
        let expect = Some(Lexed::new(Token::Number(123), &"c"));
        assert_eq!(number(&test), expect);

        let test = "123";
        let expect = Some(Lexed::new(Token::Number(123), &""));
        assert_eq!(number(&test), expect);

        let test = "+123";
        let expect = None;
        assert_eq!(number(&test), expect);
    }

    #[test]
    fn test_operator() {
        let test = "+";
        let expect = Some(Lexed::new(Token::Plus, &""));
        assert_eq!(plus(&test), expect);

        let test = "+ 1 2";
        let expect = Some(Lexed::new(Token::Plus, &" 1 2"));
        assert_eq!(plus(&test), expect);
        
        let test = "-";
        let expect = Some(Lexed::new(Token::Minus, &""));
        assert_eq!(minus(&test), expect);

        let test = "- 1 2";
        let expect = Some(Lexed::new(Token::Minus, &" 1 2"));
        assert_eq!(minus(&test), expect);

        let test = "*";
        let expect = Some(Lexed::new(Token::Asterisk, &""));
        assert_eq!(asterisk(&test), expect);

        let test = "* 1 2";
        let expect = Some(Lexed::new(Token::Asterisk, &" 1 2"));
        assert_eq!(asterisk(&test), expect);

        let test = "/";
        let expect = Some(Lexed::new(Token::Slash, &""));
        assert_eq!(slash(&test), expect);

        let test = "/ 1 2";
        let expect = Some(Lexed::new(Token::Slash, &" 1 2"));
        assert_eq!(slash(&test), expect);
    }
}