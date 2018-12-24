#[macro_use]
mod macros;
mod join;

fn main() {
    let args = vec!["file_name".to_owned(), "main.rs".to_owned(), "join.rs".to_owned()]; 
    join::umain(args); 
}
