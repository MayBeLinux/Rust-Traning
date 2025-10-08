fn main() {
    println!("Hello, world!");
    another_functions(12 , "salut")
}

fn another_functions(x: i32 , y: &str) {

    println!("Le chiffre de puis fonction main est {x} est {y}")

}
