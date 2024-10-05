#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, PartialEq)]
struct Lexer<'a> {
    current: Option<Token>,
    code: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.current;
        if let Some(lexed) = Self::lex(self.code) {
            self.current = Some(lexed.token);
            self.code = lexed.rest;
        } else {
            self.current = None;
            self.code = &"";
        };
        token
    }
}

impl<'a> Lexer<'a>  {
    pub fn new(code: &'a str) -> Self {
        if let Some(lexed) = Self::lex(code) {
            let current = Some(lexed.token);
            let code = lexed.rest;
            Lexer { current, code }
        } else {
            Lexer { current: None, code: &"" }
        }
    }

    fn lex(code: &str) -> Option<Lexed> {
        let functions = [
            Self::l_paren, 
            Self::r_paren,
            Self::l_brace, 
            Self::r_brace,
            Self::plus,
            Self::minus,
            Self::asterisk,
            Self::slash, 
            Self::number];
        functions.iter().find_map(|f| {
            let code = code.trim_start();
            f(code)
        })
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
    
    fn l_paren(code: &str) -> Option<Lexed> {
        Self::char(code, '(', Token::LParen)
    }
    
    fn r_paren(code: &str) -> Option<Lexed> {
        Self::char(code, ')', Token::RParen)
    }

    fn l_brace(code: &str) -> Option<Lexed> {
        Self::char(code, '[', Token::LBrace)
    }
    
    fn r_brace(code: &str) -> Option<Lexed> {
        Self::char(code, ']', Token::RBrace)
    }
    
    fn plus(code: &str) -> Option<Lexed> {
        Self::char(code, '+', Token::Plus)
    }
    
    fn minus(code: &str) -> Option<Lexed> {
        Self::char(code, '-', Token::Minus)
    }
    
    fn asterisk(code: &str) -> Option<Lexed> {
        Self::char(code, '*', Token::Asterisk)
    }
    
    fn slash(code: &str) -> Option<Lexed> {
        Self::char(code, '/', Token::Slash)
    }
    
    fn char(code: &str, target: char, token: Token) -> Option<Lexed> {
        let mut chars = code.chars();
        let next = chars.next();
        match next {
            Some(c) if c == target => Some(Lexed::new(token, &chars.as_str())),
            _ => None
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paren() {
        let test = "()";
        let expect = Some(Lexed::new(Token::LParen, &")"));
        assert_eq!(Lexer::l_paren(&test), expect);

        let test = "))";
        let expect = None;
        assert_eq!(Lexer::l_paren(&test), expect);

        let test = "))";
        let expect = Some(Lexed::new(Token::RParen, &")"));
        assert_eq!(Lexer::r_paren(&test), expect);

        let test = "()";
        let expect = None;
        assert_eq!(Lexer::r_paren(&test), expect);
    }

    #[test]
    fn test_bracket() {
        let test = "[]";
        let expect = Some(Lexed::new(Token::LBrace, &"]"));
        assert_eq!(Lexer::l_brace(&test), expect);

        let test = "]]";
        let expect = None;
        assert_eq!(Lexer::l_brace(&test), expect);

        let test = "]]";
        let expect = Some(Lexed::new(Token::RBrace, &"]"));
        assert_eq!(Lexer::r_brace(&test), expect);

        let test = "[]]";
        let expect = None;
        assert_eq!(Lexer::r_brace(&test), expect);
    }

    #[test]
    fn test_number() {
        let test = "123c";
        let expect = Some(Lexed::new(Token::Number(123), &"c"));
        assert_eq!(Lexer::number(&test), expect);

        let test = "123";
        let expect = Some(Lexed::new(Token::Number(123), &""));
        assert_eq!(Lexer::number(&test), expect);

        let test = "+123";
        let expect = None;
        assert_eq!(Lexer::number(&test), expect);
    }

    #[test]
    fn test_operator() {
        let test = "+";
        let expect = Some(Lexed::new(Token::Plus, &""));
        assert_eq!(Lexer::plus(&test), expect);

        let test = "+ 1 2";
        let expect = Some(Lexed::new(Token::Plus, &" 1 2"));
        assert_eq!(Lexer::plus(&test), expect);
        
        let test = "1+2";
        let expect = None;
        assert_eq!(Lexer::plus(&test), expect);

        let test = "-";
        let expect = Some(Lexed::new(Token::Minus, &""));
        assert_eq!(Lexer::minus(&test), expect);

        let test = "- 1 2";
        let expect = Some(Lexed::new(Token::Minus, &" 1 2"));
        assert_eq!(Lexer::minus(&test), expect);

        let test = "*";
        let expect = Some(Lexed::new(Token::Asterisk, &""));
        assert_eq!(Lexer::asterisk(&test), expect);

        let test = "* 1 2";
        let expect = Some(Lexed::new(Token::Asterisk, &" 1 2"));
        assert_eq!(Lexer::asterisk(&test), expect);

        let test = "/";
        let expect = Some(Lexed::new(Token::Slash, &""));
        assert_eq!(Lexer::slash(&test), expect);

        let test = "/ 1 2";
        let expect = Some(Lexed::new(Token::Slash, &" 1 2"));
        assert_eq!(Lexer::slash(&test), expect);
    }

    #[test]
    fn test_lex() {
        let test = "123c";
        let expect = Some(Lexed::new(Token::Number(123), &"c"));
        assert_eq!(Lexer::lex(&test), expect);

        let test = "+ 1 2";
        let expect = Some(Lexed::new(Token::Plus, &" 1 2"));
        assert_eq!(Lexer::lex(&test), expect);
        
        let test = "1+2";
        let expect = Some(Lexed::new(Token::Number(1), &"+2"));
        assert_eq!(Lexer::lex(&test), expect);

        let test = "))";
        let expect = Some(Lexed::new(Token::RParen, &")"));
        assert_eq!(Lexer::r_paren(&test), expect);

        let test = "~";
        let expect = None;
        assert_eq!(Lexer::r_paren(&test), expect);
    }

    #[test]
    fn test_lexer() {
        let code = "(+12)";
        let lexer = Lexer::new(&code);
        let mut lexer = lexer.peekable();
        assert_eq!(lexer.peek(), Some(&Token::LParen));
        assert_eq!(lexer.next(), Some(Token::LParen));
        assert_eq!(lexer.peek(), Some(&Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.peek(), Some(&Token::Number(12)));
        assert_eq!(lexer.next(), Some(Token::Number(12)));
        assert_eq!(lexer.peek(), Some(&Token::RParen));
        assert_eq!(lexer.next(), Some(Token::RParen));
        assert_eq!(lexer.next(), None);

        let code = "( + 1 2 )";
        let lexer = Lexer::new(&code);
        let mut lexer = lexer.peekable();
        assert_eq!(lexer.peek(), Some(&Token::LParen));
        assert_eq!(lexer.next(), Some(Token::LParen));
        assert_eq!(lexer.peek(), Some(&Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.peek(), Some(&Token::Number(1)));
        assert_eq!(lexer.next(), Some(Token::Number(1)));
        assert_eq!(lexer.peek(), Some(&Token::Number(2)));
        assert_eq!(lexer.next(), Some(Token::Number(2)));
        assert_eq!(lexer.peek(), Some(&Token::RParen));
        assert_eq!(lexer.next(), Some(Token::RParen));
        assert_eq!(lexer.next(), None);
    }
}