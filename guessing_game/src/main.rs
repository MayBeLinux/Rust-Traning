use std::io;  // Bibliothèque standard pour les entrées/sorties

fn main() {  // Le FN déclare une fonction nommée main, le point d'entrée du programme.
    println!("mynumber the number !");  // Macro pour afficher du texte à la console  // "!" indique que c'est une macro, pas une fonction ordinaire.

    println!("Please input your mynumber.");  // Même chose que ci-dessus

    let mut mynumber = String::new(); //Crée une nouvelle variable nommé mynumber de type String (chaîne de caratères) et mutable (mut) qui veut dire que sa valeur peut changer
   
    // le let est utilisé pour déclarer des variables en Rust. Par défaut, les variables sont immuables, mais en utilisant le mot-clé mut, on peut rendre une variable mutable.
    // Let instructions pour crée une variable.
    // let apples = 5; Variables immuables
    // let mut bananas = 5; variables mutables
   
    //Immutable signifie que la valeur de la variable ne peut pas être changée après sa création. Mutable signifie que la valeur de la variable peut être changée.
    
    // Autres types de variables : i32 (entier signé 32 bits), f64 (nombre à virgule flottante 64 bits), bool (booléen, true ou false), char (caractère Unicode)


    io::stdin() // Appelle la fonction stdin de la bibliothèque io pour obtenir une poignée vers l'entrée standard (stdin)
    // IO est l'abréviation de Input/Output (Entrée/Sortie)
    // Stdin est une fonction qui permet de lire les entrées de l'utilisateur depuis la console. 

    .read_line(&mut mynumber) // Prend l'entrée standard (stdin) et lit une ligne de texte, en stockant le résultat dans la variable mynumber. Le & indique que nous passons une référence à la variable mynumber, plutôt que la valeur elle-même.
    .expect("Failed to read line");  // Permet de récupérer une erreur si jamais la lecture de la ligne échoue, il existe deux types OK et Err en Rust

    println!("You mynumbered : {mynumber}");  // Affiche la valeur de la variable mynumber en utilisant l'interpolation de chaîne (le {mynumber} est remplacé par la valeur actuelle de la variable mynumber)
    // {} est utilisé pour l'interpolation de chaîne en Rust, permettant d'insérer la valeur d'une variable ou d'une expression dans une chaîne de caractères.

}


// lien vers le cours du Rust Book : https://doc.rust-lang.org/book/ch02-00-mynumbering-game-tutorial.html#storing-values-with-variables
