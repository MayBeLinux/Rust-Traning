fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1: {s1}, s3: {s3}");
    println!("s1: {s1}, s3: {s3}");
} // fin de main, s3 est libéré ici car il sort du scope. s2 n'est pas libéré car sa propriété a été déplacée dans s3. s1 est libéré ici aussi car il sort du scope.



fn gives_ownership() -> String {

    let some_string = String::from("yours"); // some_string est créé et prend possession de la chaîne "yours"
    some_string // some_string est retourné et sa propriété est déplacée vers le scope appelant (main)
}


fn takes_and_gives_back(a_string: String) -> String { // a_string prend possession de la chaîne passée en argument

    a_string // a_string est retourné et sa propriété est déplacée vers le scope appelant (main)

}
/*
Résumé détaillé du code :

Ce programme illustre la notion de propriété (ownership) en Rust à travers trois fonctions principales :

1. gives_ownership() -> String :
    - Crée une nouvelle String "yours".
    - Retourne cette String, transférant ainsi sa propriété au scope appelant (ici, main).

2. takes_and_gives_back(a_string: String) -> String :
    - Prend une String en paramètre, ce qui transfère la propriété de cette String à la fonction.
    - Retourne la String, transférant à nouveau la propriété au scope appelant.

3. main() :
    - Reçoit la propriété d'une String via gives_ownership() (s1).
    - Crée une nouvelle String "hello" (s2).
    - Passe s2 à takes_and_gives_back(), ce qui transfère la propriété de s2 à la fonction, puis la récupère dans s3.
    - Affiche s1 et s3.
    - À la fin de main, s1 et s3 sont libérés (drop), car ils sortent du scope. s2 n'est pas libéré explicitement car sa propriété a été déplacée dans s3.

Ce code montre comment la propriété des données est transférée entre fonctions en Rust, évitant ainsi les problèmes de double libération ou de références invalides.
*/
