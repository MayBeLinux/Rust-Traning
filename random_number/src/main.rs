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

    let secret_number = rand::thread_rng().gen_range(1..=100); // Génère un nombre aléatoire entre 1 et 100 inclus.

    //println!("The secret number is {}" , secret_number); // Affiche le nombre secret (pour le débogage).
    loop {
    println!("Please input your guess");

    let mut guess = String::new(); // Crée une nouvelle variable mutable nommée guess de type String.

     io::stdin()
    .read_line(&mut guess) // Lit une ligne de l'entrée standard et la stocke dans la variable guess.
    .expect("Failed to read your custom line , Error 1"); // Gère les erreurs de lecture de l'entrée standard.

    let guess: u32 = match guess.trim().parse() { // Parse est une méthode qui convertit une chaîne de caratères en un autre type.
        // Match est une expression qui permet de faire des correspondances de motifs.
        // Match compare la valeur de guess avec les motifs définis dans les branches suivantes.
        // Trim est une méthode qui supprime les espaces blancs au début et à la fin d'une chaîne de caractères.
    Ok(num) => num, // Si la conversion réussit, stocke la valeur dans guess. // Num est une variable temporaire qui stocke la valeur convertie.
    Err(_) => continue, // continue permet de recommencer la boucle si la conversion échoue.
    // Le _ est un caractère générique qui permet d'ignorer la valeur de l'erreur.
    // => est un opérateur qui permet de faire correspondre une valeur avec un motif.
    }; // ; permet de terminer une instruction.
    println!("You guessed : {}", guess); // Affiche la valeur de la variable guess. // {} est un espace réservé pour la valeur de guess qui est inscrite juste après la virugule.

    match guess.cmp(&secret_number) {  // Compare la valeur de guess avec celle de secret_number.
        Ordering::Less => println!("Too small !"), // Si guess est inférieur à secret_number alors affiche "Too small !"
        Ordering::Greater => {
            println!("Tu renvoie une valeurs bien trop élévée que celle inscrite dans les valeurs de random");
            println!("Too big !"); // Si guess est supérieur à secret_number alors affiche "Too big !"
        }

        Ordering::Equal => {
            println!("You win !"); // Si guess est égal à secret_number alors affiche "You win !"
            break; // Break qui permet de sortir de la boucle quand l'utilisateur a trouvé le numéro correct !
        }
        



        // Ordering:: est une énumération qui a trois variantes : Less, Greater et Equal.
        // cmp est une méthode qui compare deux valeurs et renvoie une valeur de type Ordering.
        // La valeur de retour peut être utilisée pour déterminer si une valeur est inférieure, supérieure ou égale à une autre valeur.

        // le faites d'écrire Ordering:: Less, Ordering::Greater et Ordering::Equal est une façon de faire référence aux variantes de l'énumération Ordering.
        // Ordering comporte trois variantes : Less, Greater et Equal.

      }
    }

}


// Informations utiles, je jamais oublier " ; " à la fin des instructions ! 