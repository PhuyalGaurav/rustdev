fn main() {
    let spaces: &str = "   ";
    {
        let spaces: usize = spaces.len();
        println!("The value of spaces is: {}", spaces);
    }
    println!("The value of spaces is: {}", spaces);
}
