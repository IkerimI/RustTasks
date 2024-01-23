use rust_bison_skeleton::{process_bison_file, BisonErr};
use std::path::Path;

fn main() {
    match process_bison_file(Path::new("src/parser/minako_syntax.y")) {
        Ok(_) => {}
        Err(BisonErr { message, .. }) => {
            eprintln!("Bison error:\n{}\nexiting with 1", message);
            std::process::exit(1);
        }
    }
    println!("cargo:rerun-if-changed=src/parser/minako_syntax.y");
}
