use std::mem;

struct Exemple {
    a: u8,
    b: i32,
}

fn main() {
    println!("taille = {}", mem::size_of::<Exemple>());
    println!("alignement = {}", mem::align_of::<Exemple>());

    let e = String::from("Salut");
    let f = 65;

    test();
    test2();
    test3(e);
    test4(f);
    test3(e); // e n'est plus valide ici car elle a été déplacéze dans la fonction test3 (move semantics). A partir du moment ou une variable est déplacée, elle n'est plus valide dans le scope d'origine. car on a appellé la fonction test3 qui a pris la propriété de e et la envoyé dans la propriété de number_integer, est quand le println est temriné on sort du scope car derriere j'ai des accolades, donc la fonction drop() est appellée et la mémoire est libérée.
    test4(f); // f est toujours valide ici car les types primitifs comme i32 implémentent le trait Copy, donc ils sont copiés de manière superficielle (shallow copy) et non déplacés (move semantics).
}


fn test() {

    let a = String::from("Hello");
    let b = a.clone(); // utilisation de la méthode clone() pour faire une copie profonde de a dans b..
    println!("a = {a}, b = {b}"); // a et b sont valides car on a utilisé la méthode clone() qui permet de copier non pas de manière superficielle mais en profondeur (deep copy).
}


fn test2() {
   let a = 12;
   let b = a;
   println!("a = {}, b = {}", a, b);
}

fn test3(number_integer: String) {
    println!("la valeur {number_integer} est bien recue"); // e n'est plus valide ici car elle a été déplacée dans la fonction test3 (move semantics). A partir du moment ou une variable est déplacée, elle n'est plus valide dans le scope d'origine. car on a appellé la fonction test3 qui a pris la propriété de e et la envoyé dans la propriété de number_integer, est quand le println est temriné on sort du scope car derriere j'ai des accolades, donc la fonction drop() est appellée et la mémoire est libérée.
}

fn test4(numbers_integer: i32) {
    println!("la valeur {numbers_integer} est bien recue"); // f est toujours valide ici car les types primitifs comme i32 implémentent le trait Copy, donc ils sont copiés de manière superficielle (shallow copy) et non déplacés (move semantics).
}

/*
Résumé sur la gestion de la propriété en Rust :

- Propriété (Ownership) : En Rust, chaque valeur a un propriétaire unique. Quand la propriété est transférée (move), l'ancienne variable n'est plus valide.
- Move : Transfert de la propriété d'une variable à une autre (ex : passer une String à une fonction).
- Copy : Les types simples (comme i32, u8) implémentent le trait Copy, donc ils sont copiés lors d'une affectation ou d'un passage en fonction.
- Clone : Permet de faire une copie profonde (deep copy) d'une valeur, utile pour les types complexes comme String.
- Scope : Quand une variable sort de son scope, la fonction drop() est appelée automatiquement pour libérer la mémoire.
- Fonction : Les fonctions test, test2, test3, test4 illustrent comment la propriété est transférée ou copiée selon le type de la variable.

Ce code montre comment Rust gère la mémoire et la propriété pour garantir la sécurité et éviter les erreurs courantes comme les doubles libérations ou les accès invalides.
*/
