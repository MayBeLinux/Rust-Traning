use std::cmp::Ordering; // Bibliothèques standard pour les comparaisons
use std::io; // Bibliothèque standard pour les entrées/sorties

use rand::Rng; // Bibliothèque pour la génération de nombres aléatoires
// Rng définit les méthodes implémentées par les génrateurs de nombres aléatoires. 
// Pourquoi ::Rng ? Parce que Rng est un trait, et les traits sont une sorte d'interface qui définit des fonctionnalités particulières qu'un type doit implémenter si le type veut implémenter ce trait.

// Explication d'un trait : https://doc.rust-lang.org/book/ch10-02-traits.html
// Un trait est une collection de méthodes définies pour un type particulier. 
// type doit implémenter un trait pour pouvoir utiliser ses méthodes.
// Un type est une structure de données définie par l'utilisateur qui regroupe des données et des comportements associés.

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Génère un nombre aléatoire entre 1 et 100 inclus

    println!("The secret number is {}" , secret_number); // Affiche le nombre secret (pour le débogage)

    println!("Please input your guess");

    let mut guess = String::new(); // Crée une nouvelle variable mutable nommée guess de type String

     io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read your custom line , Error 1");

    let guess: u32 = guess.trim().parse().expect("Please type a number !"); // Convertit la chaîne de caratères en un nombre entier non signé de 32 bits (u32)
    
    println!("You guessed : {}", guess); // Affiche la valeur de la variable guess

    match guess.cmp(&secret_number) {  // Compare la valeur de guess avec celle de secret_number 
        Ordering::Less => println!("Too small !"), // Si guess est inférieur à secret_number alors affiche "Too small !"
        Ordering::Greater => println!("Too big !"), // Si guess est supérieur à secret_number alors affiche "Too big !"
        Ordering::Equal => println!("You win !"), // Si guess est égal à secret_number alors affiche "You win !"



        // Ordering:: est une énumération qui a trois variantes : Less, Greater et Equal.
        // cmp est une méthode qui compare deux valeurs et renvoie une valeur de type Ordering.
        // La valeur de retour peut être utilisée pour déterminer si une valeur est inférieure, supérieure ou égale à une autre valeur.

        // le faites d'écrire Ordering:: Less, Ordering::Greater et Ordering::Equal est une façon de faire référence aux variantes de l'énumération Ordering.
        // Ordering comporte trois variantes : Less, Greater et Equal.

    }

}


// Informations utiles, je jamais oublier " ; " à la fin des instructions ! 

