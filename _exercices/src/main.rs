use::std::io; // importation de la bibliothèque standard d'entrée/sortie pour permettre la lecture des entrées utilisateur.

fn main() {
    // fonctions main pour l'appelle vers les autres fonctions.
    println!("Entrez une température :");
    let mut input = String::new(); // création d'une nouvelle chaîne de caractères mutable pour stocker l'entrée de l'utilisateur.
    io::stdin().read_line(&mut input).expect("échec de la lecture de l'entrée"); // io::stdin() permet de lire l'entrée standard (clavier).
                                                                                   // read_line(&mut input) lit une ligne de texte et la stocke dans la variable input. &mut indique que input est passé par référence mutable.
                                                                                   // expect("échec de la lecture de l'entrée") gère les erreurs potentielles lors de la lecture de l'entrée.
    let temperature: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide"); // trim() supprime les espaces blancs autour de l'entrée.
                                                                                           // parse() convertit la chaîne de caractères en un nombre à virgule flottante (f64).
                                                                                           // expect("Veuillez entrer un nombre valide") gère les erreurs potentielles lors de la conversion.

    let fahrenheit = celcius_to_fahrenheit(temperature); // appel de la fonction de conversion de Celsius à Fahrenheit.
    let celcius = fahrenheit_to_celcius(temperature); // appel de la fonction de conversion de Fahrenheit à Celsius.
    fib(); // appel de la fonction pour afficher les 10 premiers nombres de Fibonacci.
}


fn celcius_to_fahrenheit(celsius: f64) { // ici entre parenthèses c'est un paramètre qui va être passé à la fonction.
                                                // le -> f64 indique que le type de retour de la fonction sera un f64 (nombre à virgule flottante à 64 bits).

let result = (celsius * 9.0 / 5.0) + 32.0; // formule de conversion de Celsius à Fahrenheit             
println!("Celcius en Fahrenheit donne {}°F", result); // affichage du résultat en Fahrenheit.                  

}


fn fahrenheit_to_celcius(fahrenheit: f64) { // ici entre parenthèses c'est un paramètre qui va être passé à la fonction.
                                                   // le -> f64 indique que le type de retour de la fonction sera un f64 (nombre à virgule flottante à 64 bits).
let result = (fahrenheit - 32.0) * 5.0 / 9.0; // formule de conversion de Fahrenheit à Celsius.
println!("{}°C", result); // affichage du résultat en Celsius.
}


fn fib() {

    let mut a = 0; // initialisation des deux premières valeurs de la suite de Fibonacci.
    let mut b = 1; // initialisation des deux premières valeurs de la suite de Fibonacci.
    for _ in 0..10 { // le _ est utilisé pour indiquer que la variable de boucle n'est pas utilisée dans le corps de la boucle.
        let temp = a; // temp est une variable temporaire pour stocker la valeur actuelle de a.
        a = b; // mise à jour de a avec la valeur actuelle de b.
        b = temp + b; // mise à jour de b avec la somme de l'ancienne valeur de a (stockée dans temp) et de b.
        println!("{}", a); // affichage de la valeur actuelle de a.
    }
}