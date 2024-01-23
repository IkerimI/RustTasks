use crate::parser::loc::Loc;
use crate::parser::token::Token;
use logos::{Lexer, Logos, Span};

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum C1Token {
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    /// =
    Assign,

    #[token("==")]
    /// ==
    Equal,

    #[token("!=")]
    /// !=
    NotEqual,

    #[token("<")]
    /// <
    Less,

    #[token(">")]
    /// >
    Greater,

    #[token("<=")]
    /// <=
    LessEqual,

    #[token(">=")]
    /// >=
    GreaterEqual,

    #[token("&&")]
    /// &&
    And,

    #[token("||")]
    /// ||
    Or,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    /// (
    LeftParenthesis,

    #[token(")")]
    /// )
    RightParenthesis,

    #[token("{")]
    /// {
    LeftBrace,

    #[token("}")]
    /// }
    RightBrace,

    #[regex("[0-9]+")]
    ConstInt,

    #[regex(r"(\d+\.\d+)|(\.\d+([eE]([-+])?\d+)?)|(\d+[eE]([-+])?\d+)")]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex("\"[^\n\"]*\"")]
    ConstString,

    #[regex("[a-zA-Z]+[0-9a-zA-Z]*")]
    Identifier,

    #[regex(r"/\*[^\*/]*\*/", logos::skip)]
    CComment,

    #[regex("//[^\n]*(\n)?", logos::skip)]
    CPPComment,

    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,

    #[regex(r"[\n]")]
    Linebreak,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

#[derive(Debug)]
#[allow(non_upper_case_globals)]
pub struct C1Lexer {
    logos_lexer: Lexer<'static, C1Token>,
    logos_line_number: usize,
    current_token: Option<TokenData>,
    peek_token: Option<TokenData>,
    span: Span,
}

impl C1Lexer {
    /// Initialize a new C1Lexer for the given string slice
    pub fn new(text: &'static str) -> C1Lexer {
        let mut lexer = C1Lexer {
            logos_lexer: C1Token::lexer(text),
            logos_line_number: 1,
            current_token: None,
            peek_token: None,
            span: Span::default(),
        };
        lexer.current_token = lexer.next_token();
        lexer.span = lexer.logos_lexer.span();
        lexer.peek_token = lexer.next_token();
        lexer
    }

    /**
    Take the next C1Token and convert it into a crate::parser::Token that can be interpreted by the parser.
    This method consumes (eats) the token that it returns.
    */
    pub(crate) fn yylex(&mut self) -> Token {
        match self.current_token() {
            None => Token {
                token_type: Self::YYEOF,
                loc: Loc::from_span(self.span()),
            },
            Some(token) => {
                let token_type: i32 = match token {
                    C1Token::KwBoolean => Self::KW_BOOLEAN,
                    C1Token::KwDo => Self::KW_DO,
                    C1Token::KwElse => Self::KW_ELSE,
                    C1Token::KwFloat => Self::KW_FLOAT,
                    C1Token::KwFor => Self::KW_FOR,
                    C1Token::KwIf => Self::KW_IF,
                    C1Token::KwInt => Self::KW_INT,
                    C1Token::KwPrintf => Self::KW_PRINTF,
                    C1Token::KwReturn => Self::KW_RETURN,
                    C1Token::KwVoid => Self::KW_VOID,
                    C1Token::KwWhile => Self::KW_WHILE,
                    C1Token::Equal => Self::EQ,
                    C1Token::NotEqual => Self::NEQ,
                    C1Token::Less => Self::LSS,
                    C1Token::Greater => Self::GRT,
                    C1Token::LessEqual => Self::LEQ,
                    C1Token::GreaterEqual => Self::GEQ,
                    C1Token::And => Self::AND,
                    C1Token::Or => Self::OR,
                    C1Token::ConstInt => Self::CONST_INT,
                    C1Token::ConstFloat => Self::CONST_FLOAT,
                    C1Token::ConstBoolean => Self::CONST_BOOLEAN,
                    C1Token::ConstString => Self::CONST_STRING,
                    C1Token::Identifier => Self::ID,
                    C1Token::CComment
                    | C1Token::CPPComment
                    | C1Token::Whitespace
                    | C1Token::Linebreak
                    | C1Token::Error => Self::YYerror,
                    C1Token::Plus => '+' as i32,
                    C1Token::Minus => '-' as i32,
                    C1Token::Asterisk => '*' as i32,
                    C1Token::Slash => '/' as i32,
                    C1Token::Assign => '=' as i32,
                    C1Token::Comma => ',' as i32,
                    C1Token::Semicolon => ';' as i32,
                    C1Token::LeftParenthesis => '(' as i32,
                    C1Token::RightParenthesis => ')' as i32,
                    C1Token::LeftBrace => '{' as i32,
                    C1Token::RightBrace => '}' as i32,
                };
                let token = Token {
                    token_type,
                    loc: Loc::from_span(self.span()),
                };
                self.eat();
                token
            }
        }
    }

    /// Return the C1Token variant of the current token without consuming it.
    /// ```
    /// use cb_4::parser::{C1Lexer,C1Token};
    /// let lexer = C1Lexer::new("current next");
    ///
    /// assert_eq!(lexer.current_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.current_text(), Some("current"));
    ///
    /// assert_eq!(lexer.current_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.current_text(), Some("current"));
    /// ```
    pub fn current_token(&self) -> Option<C1Token> {
        self.current_token.token_type()
    }

    /// Return the C1Token variant of the next token without consuming it.
    ///```
    /// use cb_4::parser::{C1Lexer,C1Token};
    /// let lexer = C1Lexer::new("current next");
    ///
    /// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    ///
    /// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    /// ```
    pub fn peek_token(&self) -> Option<C1Token> {
        self.peek_token.token_type()
    }

    /// Return the text of the current token
    pub fn current_text(&self) -> Option<&str> {
        self.current_token.text()
    }

    /// Return the text of the next token
    pub fn peek_text(&self) -> Option<&str> {
        self.peek_token.text()
    }

    /// Return the line number where the current token is located
    pub fn current_line_number(&self) -> Option<usize> {
        self.current_token.line_number()
    }

    /// Return the line number where the next token is located
    pub fn peek_line_number(&self) -> Option<usize> {
        self.peek_token.line_number()
    }

    /// Return the span (aka. range) of characters where the current token is located
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Drop the current token and retrieve the next token in the text.
    /// ```
    /// use cb_4::C1Lexer;
    /// let mut lexer = C1Lexer::new("current next last");
    ///
    /// assert_eq!(lexer.current_text(), Some("current"));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), Some("next"));
    /// assert_eq!(lexer.peek_text(), Some("last"));
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), Some("last"));
    /// assert_eq!(lexer.peek_text(), None);
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), None);
    /// assert_eq!(lexer.peek_text(), None);
    /// ```
    pub fn eat(&mut self) {
        self.current_token = self.peek_token.take();
        self.span = self.logos_lexer.span();
        self.peek_token = self.next_token();
    }

    /// Private method for reading the next token from the logos::Lexer and extracting the required data
    /// from it
    fn next_token(&mut self) -> Option<TokenData> {
        // Retrieve the next token from the internal lexer
        if let Some(c1_token) = self.logos_lexer.next() {
            match c1_token {
                C1Token::Linebreak => {
                    // If the token is a linebreak, increase the line number and get the next token
                    self.logos_line_number += 1;
                    self.next_token()
                }
                _ => Some(TokenData {
                    // If the token is not a linebreak, initialize and return a TokenData instance
                    token_type: c1_token,
                    token_text: self.logos_lexer.slice(),
                    token_line: self.logos_line_number,
                }),
            }
        } else {
            None
        }
    }
}

/// Hidden struct for capsuling the data associated with a token.
#[derive(Debug)]
struct TokenData {
    token_type: C1Token,
    token_text: &'static str,
    token_line: usize,
}

/// Hidden trait that makes it possible to implemented the required getter functionality directly for
/// Option<TokenData>.
trait TokenDataProvider {
    /// Return the type of the token, aka. its C1Token variant.
    fn token_type(&self) -> Option<C1Token>;
    /// Return the text of the token
    fn text(&self) -> Option<&str>;
    /// Return the line number of the token
    fn line_number(&self) -> Option<usize>;
}

impl TokenDataProvider for Option<TokenData> {
    fn token_type(&self) -> Option<C1Token> {
        self.as_ref().map(|data| data.token_type)
    }

    fn text(&self) -> Option<&'static str> {
        self.as_ref().map(|data| data.token_text)
    }

    fn line_number(&self) -> Option<usize> {
        self.as_ref().map(|data| data.token_line)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::C1Lexer;
    use crate::parser::C1Token;

    #[test]
    fn lines_are_counted() {
        let mut lexer1 = C1Lexer::new("Hello\nTest");
        assert_eq!(lexer1.current_line_number(), Some(1));
        assert_eq!(lexer1.peek_line_number(), Some(2));
        lexer1.eat();
        assert_eq!(lexer1.current_line_number(), Some(2));
        assert_eq!(lexer1.peek_line_number(), None);
        lexer1.eat();
        assert_eq!(lexer1.current_line_number(), None);
        assert_eq!(lexer1.peek_line_number(), None);
    }

    #[test]
    fn line_count_is_reset() {
        {
            let mut lexer1 = C1Lexer::new("Hello\nTest\nbla\nfoo");
            lexer1.eat();
            lexer1.eat();
            assert_eq!(lexer1.current_line_number(), Some(3));
            assert_eq!(lexer1.peek_line_number(), Some(4));
        }
        let lexer2 = C1Lexer::new("bool foo()");
        assert_eq!(lexer2.current_line_number(), Some(1));
        assert_eq!(lexer2.peek_line_number(), Some(1));
    }

    #[test]
    fn float_recognition() {
        let lexer = C1Lexer::new("1.2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.000");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new(".2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e4");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e+4");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e-10");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2E-10");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("33E+2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));
    }
}
