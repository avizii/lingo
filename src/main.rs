mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Lingo programming language!");
    println!("Feel free to type in commands");

    repl::start();
}
