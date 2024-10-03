mod lexer;

fn main() {
    let test = "ssss".to_string();
    lexer::lexer::lex(&test);
    println!("Hello, world!");
}
