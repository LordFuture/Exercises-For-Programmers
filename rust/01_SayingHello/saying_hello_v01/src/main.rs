//------------------------------------------------------------------------------
// Practising using Rust, following the book "Exercises for Programmers 57: 
// Challenges to Develop Your Coding Skills" by Brian P. Hogan
//------------------------------------------------------------------------------
// 1) Saying Hello v 01
// The “Hello, World” program is the first program you learn to write in many 
// languages, but it doesn’t involve any input. So create a program that prompts 
// for your name and prints a greeting using your name.
//------------------------------------------------------------------------------
// *Example Output*
// `What is your name? Brian`
// `Hello, Brian, nice to meet you!`
//------------------------------------------------------------------------------
// *Constraint*
// *Keep the input, string concatenation, and output separate.
//------------------------------------------------------------------------------
use std::io;
use std::io::Write;

pub(crate) fn main() {
    //--------------------------------------------------------------------------
    // println! adds a newline character to the end of string so that the input 
    // cursor is on next line. Using print! solves the issue but stdout is 
    // frequently line-buffered by default so it may be necessary to use 
    // io::stdout().flush() to ensure the output is emitted immediately.
    //--------------------------------------------------------------------------
    let mut my_name = String::new();
    
    get_name(&mut my_name);

    //let reply = create_greeting(&my_name);

    print_greeting(&create_greeting(&my_name));
}
//------------------------------------------------------------------------------
// Removes newline characters if they are found at the end of strings
//------------------------------------------------------------------------------
fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn get_name(s: &mut String) {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    io::stdin()
    .read_line(s)
    .expect("Failed to read line");
    trim_newline(s);
}

fn create_greeting(s: &String) -> String {
    return format!("Hello {}, nice to meet you!", s);
}

fn print_greeting(s: &String) {
    println!("{}", s);
}