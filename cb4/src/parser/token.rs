use crate::parser::loc::Loc;
use crate::parser::minako_syntax;

/// A token that is emitted by a lexer and consumed by a parser
#[derive(Clone)]
pub struct Token {
    /// Type of the token (i.e. tNUM, tPLUS or YYEOF)
    pub token_type: i32,

    /// Location of the token (i.e. range in source code that it refers to)
    pub loc: Loc,
}

use std::fmt;

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_> /*'*/) -> fmt::Result {
        f.write_str(&format!(
            "[{}, {}...{}]",
            Self::token_name(self.token_type),
            self.loc.begin,
            self.loc.end
        ))
    }
}

impl Token {
    fn token_name(token_type: i32) -> String {
        if token_type > 256 {
            String::from(minako_syntax::token_name(token_type))
        } else {
            String::from(char::from_u32(token_type as u32).unwrap())
        }
    }
}
