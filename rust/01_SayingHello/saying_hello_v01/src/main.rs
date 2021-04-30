use std::io;
use std::io::Write;

pub(crate) fn main() {
    //--------------------------------------------------------------------------
    // println! adds a newline character to the end of string so that the input 
    // cursor is on next line. Using print! solves the issue but stdout is 
    // frequently line-buffered by default so it may be necessary to use 
    // io::stdout().flush() to ensure the output is emitted immediately.
    //--------------------------------------------------------------------------
    print!("What is your name? ");
    io::stdout().flush().unwrap();
    let mut my_name = String::new();

    io::stdin()
    .read_line(&mut my_name)
    .expect("Failed to read line");
    trim_newline(&mut my_name);

    println!("Hello {}, nice to meet you!", my_name);
}
//------------------------------------------------------------------------------
// Removes newline characters if they are found at the end of strings
//------------------------------------------------------------------------------
fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}