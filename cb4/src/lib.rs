pub mod parser;

pub use parser::lexer::C1Lexer;
pub use parser::loc::Loc;
pub use parser::minako_syntax::{token_name, Parser};
pub use parser::token::Token;
pub use parser::value::Value;

#[cfg(test)]
fn parse(input: &'static str) -> Vec<String> {
    let lexer = C1Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.debug = false;
    parser.do_parse()
}

#[test]
fn test_valid() {
    let errors = parse("int main() {}");
    assert!(errors.is_empty());
}

#[test]
fn test_invalid() {
    let errors = parse("int main( {}");
    assert!(!errors.is_empty());
}

#[test]
fn parse_function() {
    let errors = parse(
        r"bool not(bool b) {
	if (b == true) return false;
	else return true;
}",
    );
    assert!(errors.is_empty());
}

#[test]
fn parse_demorgan() {
    let errors = parse(
        r#"
/* Ueberprueft deMorgans Law */
// Die erwartete Ausgabe steht am Ende

bool not(bool b) {
	if (b == true) return false;
	else return true;
}

bool morgan11(bool a, bool b) {
	return not(a || b);
}
bool morgan12(bool a, bool b) {
	return not(a) && not(b);
}

bool morgan21(bool a, bool b) {
	return not(a && b);
}
bool morgan22(bool a, bool b) {
	return not(a) || not(b);
}

void main() {
	bool a = true;
	bool b = true;
	int i=0;

	printf( "deMorgan\n" );

	do {
		do {

			printf(i);

			printf(morgan11(a,b));
			printf(morgan12(a,b));
			printf(morgan21(a,b));
			printf(morgan22(a,b));

			i = i+1;
			b = not(b);
		} while (b != true);
		a = not(a);
	} while(a != true);

}

/* Erwartete Ausgabe:
Process finished with exit code 0
*/
"#,
    );
    assert!(errors.is_empty());
}
