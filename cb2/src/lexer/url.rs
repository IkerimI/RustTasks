#![allow(unused_imports)]
use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};


/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // Capture link definitions

    // # [regex("<a[^>]*href=\"[^>]*\"[^>]*>")]
    // LinkUrl,

    // # [regex(">.*</a>")]
    // LinkText,

    #[regex(r#"href=[^"]*"[^<]*<"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    // Ignore all characters that do not belong to a link definition
    # [regex(".*", logos::skip)]
    Ignored,

    // Catch any error
    # [error]
    Error,
}


/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(_lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    lex = _lex;

    let (url, text) = ("wda", "daev");
    

    (LinkUrl(url.to_string()), LinkText(text.to_string()))


    // <a ... href="URL" ... >LINKTEXT</a>

}