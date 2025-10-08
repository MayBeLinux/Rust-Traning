// EXERCICE 1 : Slices

fn main() { // Fonction principale
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // On crée un tableau de 10 éléments
    let slice = &a[0..3]; // On crée une slice qui contient les éléments de l'index 0 à 3
    println!(" Nous somme dans la fonction main {:?}", slice); // Affiche la slice
    let sum = print_slice(&a); // On passe une référence à la slice
}

fn print_slice(sliced : &[i32] ) { // On utilise &[i32] pour indiquer que c'est une slice de i32
    let lenght = sliced.len(); // On utilise la méthode len() pour obtenir la longueur de la slice
    println!("Nous somme dans la fonction print slice {}", lenght); // Affiche la longueur de la slice
    
}

// Les slices sont des références vers une partie d'un tableau ou d'une collection.
// ELles permettent de travailler avec des segments de données sans avoir à copier les données elles-mêmes.
// Elles sont utiles pour manipuler des sous-ensembles de données de manière efficace et sûre.

// &str : C'est une slice de chaîne de caractères.
// &[T] : C'est une slice générique qui peut contenir des éléments de type
// &mut [T] : C'est une slice mutable qui permet de modifier les éléments qu'elle contient.
// &[u8] : C'est une slice d'octets, souvent utilisée pour manip
// &[i32] : C'est une slice d'entiers de 32 bits.
// &[f64] : C'est une slice de nombres à virgule flottante de
// &[bool] : C'est une slice de valeurs booléennes (true/false).
// &[char] : C'est une slice de caractères Unicode.
// &[String] : C'est une slice de chaînes de caractères allouées dynamiquement
// &[Vec<T>] : C'est une slice de vecteurs, où chaque élément est un vecteur de type T.
// &[Option<T>] : C'est une slice d'options, où chaque élément peut être Some(T) ou None.
// &[Result<T, E>] : C'est une slice de résultats, où chaque élément peut
