fn main() {
    let text = String::from("hello world");
    let word = first_word(&text); // on passe la référence de la variables text en faisant un appelle vers la fonction first_word
    println!("{word}");
}

fn first_word(s: &String) -> usize { // ici la fonction prend une référence à une string en entrée // la sortie de la fonction est un entier usize est un type non signé 
    let bytes = s.as_bytes(); // on convertit la string en un tableau d'octets qui va ressembler a ceci : [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]

    for (i, &byte) in bytes.iter().enumerate() { // on itère sur chaque octet du tableau avec son index.
        if byte == b' ' { // on vérifie si l'octet est un espace (le caractère espace en ASCII est représenté par le nombre 32)
            return i; // si on trouve un espace, on retourne la portion de la string jusqu'à cet espace
    }
}

    s.len()
}

// le principe de la fonction first_word est de trouver la position du premier espace dans une string et de retourner cette position en tant qu'entier. Si aucun espace n'est trouvé, la fonctino retourne la longueur totale de la string.

// 1. on initialise une variable bytes qui contient la représentation en octets de la string s.
//2. on utilise une boucle for pour itérer sur chaque octet du tableau bytes avec son index i.
//3. on vérifie si l'octet actuel est un espace en comparant avec.
//4. si on trouve un espace, on retourne l'index i, qui représente la position du premier espace dans la string.
//5. si la boucle se termine sans trouver d'espace, on retourne la longueur totale de la string s en utilisant s.len() 

// s.len() est une méthodes qui retourne la longueur de la string s en nombre de caractères.
