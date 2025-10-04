fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() retourne la longueur de la chaîne
    (s, length) // retourne la chaîne et sa longueur
}

// s1 n'est plus valide ici, car sa valeur a été déplacée dans calculate_length

