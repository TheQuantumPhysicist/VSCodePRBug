fn do_stuff() -> String {
    let s1 = "Look at me doing some stuff... ";
    let s2 = "and... doing even more stuff";
    String::new() + s1 + s2
}

fn main() {
    println!("Hello, world!");
    println!("{}", do_stuff());
}
