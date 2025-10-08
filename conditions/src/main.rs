fn main() {

    let number = 3;
    // Nous venons de crée une condition "if" 
    if number % 4 == 0 {
        println!("Ce chiffre est divisible par 4");
    } else if number % 2 == 0 {
        println!("Ce chiffre est divisible par 2");
    } else if number % 3 == 0 {
        println!("Ce chiffre est divisible par 3");
    } else {
        println!("Rien n'est divisible");
    }

    new();
}

fn new() {

    let condition = true; // Nous venons de crée une condition "if"
    let number = if condition { 5 } else { "six" }; // Ici la condition ne fonctionnera pas car les types ne correspondent pas 5 = i32 et six = &str // &str veut dire "string slice" et char veut dire "caractère"

    println!("The value of number is: {number}"); // Le number est de type i32
}

