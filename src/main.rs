fn do_stuff() -> String {
    let s1 = "Look at me doing some stuff... ";
    String::new() + s1
}

fn main() {
    println!("Hello, world!");
    println!("{}", do_stuff());
}
