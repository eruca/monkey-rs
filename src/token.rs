#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // 标识符+字面量
    IDENT,
    INT,

    // 运算符
    ASSIGN,
    PLUS,
    EQ,
    NotEq,

    // 分隔符
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // 关键字
    FUNCTION,
    LET,
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new<Ch: ToString>(typ: TokenType, ch: Ch) -> Self {
        Self {
            typ,
            literal: ch.to_string(),
        }
    }
}

pub fn lookup_ident<S: AsRef<str>>(ident: S) -> Option<TokenType> {
    match ident.as_ref() {
        "fn" => Some(TokenType::FUNCTION),
        "let" => Some(TokenType::LET),
        _ => None,
    }
}
