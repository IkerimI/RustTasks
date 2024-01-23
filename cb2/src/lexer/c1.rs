use logos::Logos;

#[derive(Logos, Debug, PartialEq)]

pub enum C1Token {
    
    # [regex(r"[ \t\n\f]+", logos::skip)] // Skip whitespaces, tabs and linebreaks
    Skip,
    
    // Schlüsselwörter
    # [token("bool")]
    KwBoolean,

    # [token("do")]
    KwDo,

    # [token("else")]
    KwElse,

    # [token("float")]
    KwFloat,

    # [token("for")]
    KwFor,

    # [token("int")]
    KwInt,

    # [token("if")]
    KwIf,

    # [token("printf")]
    KwPrintf,

    # [token("return")]
    KwReturn,

    # [token("void")]
    KwVoid,

    # [token("while")]
    KwWhile,


    // Operatoren
    # [token("+")]
    Plus,

    # [token("-")]
    Minus,

    # [token("*")]
    Asterisk,

    # [token("/")]
    Slash,

    # [token("=")]
    Assign,

    # [token("==")]
    Eq,

    # [token("!=")]
    Neq,

    # [token("<")]
    Lss,

    # [token(">")]
    Grt,

    # [token("<=")]
    Leq,

    # [token(">=")]
    Geq,

    # [token("&&")]
    And,

    # [token("||")]
    Or,


    // Sonstige Token
    # [token(",")]
    Comma,

    # [token(";")]
    Semicolon,

    # [token("(")]
    LParen,

    # [token(")")]
    RParen,

    # [token("{")]
    LBrace,

    # [token("}")]
    RBrace,


    // Termvariablen
    # [regex("[0-9]+")]
    ConstInt,

    # [regex("[0-9]*\\.[0-9]+([eE][-+]?[0-9]+)?|[0-9]+[eE][-+]?[0-9]+")]
    ConstFloat,

    # [token("true")]
    # [token("false")]
    ConstBoolean,

    # [regex(r#""[^"\n]*""#)]
    ConstString,

    # [regex("[A-Za-z]+[A-Za-z0-9_]*")]
    Id,

    // Comments
    # [regex(r#"/\*[^*/]*\*/"#, logos::skip)]
    CComment,  

    # [regex("//[^\n]*\n?", logos::skip)]
    CppComment,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
