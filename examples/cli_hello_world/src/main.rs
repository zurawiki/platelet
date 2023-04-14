fn main() {
    foo();
    bar();
}

fn foo() {
    let s = "Hello, world!".to_string();
    println!(s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn bar() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}
