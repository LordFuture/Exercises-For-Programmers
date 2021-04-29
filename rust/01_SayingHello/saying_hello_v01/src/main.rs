use std::io;

pub(crate) fn main() {
    let mut question = String::from("What is your name?");
    trim_newline(&mut question);

    max_point();

    println!("{}", question);
    let mut my_name = String::new();

    io::stdin()
    .read_line(&mut my_name)
    .expect("Failed to read line");

    trim_newline(&mut my_name);

    println!("Hello {}, nice to meet you!", my_name);
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn max_point() -> i32 {
    const MAX_POINT: i32 = 100;
    MAX_POINT
}