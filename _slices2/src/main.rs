fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &a[0..3];
    println!(" Nous somme dans la fonction main {:?}", slice);
    let sum = print_slice(&a);
}


fn print_slice(sliced : &[i32] ) {
    let lenght = sliced.len();
    println!("Nous somme dans la fonction print slice {}", lenght);
    
}