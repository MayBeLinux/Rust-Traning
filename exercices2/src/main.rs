use std::io;

struct User {
    name: String,
    age: u8,
    sexe: String,
}

fn main() {
    println!("Entrer un nombre svp");

    let mut nombre = String::new();
    let afficher = io::stdin().read_line(&mut nombre).expect("Echec impossible de lire le texte");

    println!("taille de {} afficher {} ", afficher , nombre); // nombre est un type String

    choisir(&nombre.trim()); //trim() pour enlever le \n ou toutes autres espaces que l'on ne souhaite pas dans nôtre String
}

fn choisir(nombre: &str) {
    // Première approche: vérifier si c'est un nombre valide
    let result = match nombre.parse::<i32>() {
        Ok(n) => match n {
            1 => String::from("Vous avez selectionné le nombre 1"),
            2 => String::from("Vous avez selectionné le nombre 2"),
            3 => String::from("Vous avez selectionné le nombre 3"),
            19 => String::from("Vous avez selectionné le nombre 19"),
            _ => String::from("Nombre valide mais non reconnu"),
        },
        Err(_) => String::from("Ceci n'est pas une entrée valide"),
    };
    affichage_du_texte(&result);


    println!("Entre le nom de l'utilisateur : ");
    let mut variable_string = String::new();
    io::stdin().read_line(&mut variable_string).expect("Impossible lire la donnée rentrer par l'utilisateur");
    
    println!("Entrer l'age de l'utilisateur : ");
    let mut variable_age_string = String::new();
    io::stdin().read_line(&mut variable_age_string).expect("Impossible de lire la donnée rentrer par l'utilisateur");
    let variable_age = variable_age_string.trim().parse::<u8>().expect("Age invalide");

    println!("Entrer le sexe de l'utilisateur");
    let mut variable_sexe = String::new();
    io::stdin().read_line(&mut variable_sexe).expect("Impossible de lire la donnée rentrer par l'utilisateur");

    let user = users_informations(variable_string, variable_age, variable_sexe);
    affiche_le_texte(user.name, user.age , user.sexe);
}


fn affichage_du_texte(texte: &str) {
    println!("Voici vôtre texte : {}", texte);
    println!("Voici vôtre texte : {}", texte);

        // Fin de la fonction      
}



fn users_informations(name: String, age: u8, sexe: String) -> User {
    let user = User {
        name,    // Raccourci: équivalent à name: name
        age,     // Raccourci: équivalent à age: age  
        sexe     // Raccourci: équivalent à sexe: sexe
    };

    user
}


fn affiche_le_texte(nom:String, ages:u8, orientation:String) {
    println!("Le nom de la personne est : {nom}");
    println!("Il a : {ages}");
    println!("Son orientation est {orientation}");
} // Fin du scope pour les informations envoyé dans la fonction acutelle, car utilisé dans la fonction println!

/*
=== ANALYSE OWNERSHIP & BORROWING DE CE CODE ===

1. STRUCT USER :
   - struct User { name: String, age: u8, sexe: String }
   - OWNERSHIP: La struct POSSÈDE les données String (name, sexe)
   - age (u8) implémente Copy, donc pas de move nécessaire

2. FONCTION MAIN() :
   - let mut nombre = String::new() → main() POSSÈDE "nombre"
   - &nombre.trim() → BORROWING (emprunt immutable) vers choisir()
   - variable_string, variable_age_string, variable_sexe → main() les POSSÈDE
   - users_informations(variable_string, variable_age, variable_sexe) → MOVE !
     * variable_string est DÉPLACÉE vers users_informations (plus accessible dans main)
     * variable_sexe est DÉPLACÉE vers users_informations (plus accessible dans main)
     * variable_age est COPIÉE (u8 implémente Copy)

3. FONCTION CHOISIR() :
   - fn choisir(nombre: &str) → BORROWING (reçoit une référence)
   - Pas de move car on emprunte seulement
   - affichage_du_texte(&result) → BORROWING de result

4. FONCTION USERS_INFORMATIONS() :
   - fn users_informations(name: String, age: u8, sexe: String) → User
   - OWNERSHIP: Reçoit la PROPRIÉTÉ de name et sexe
   - RETOURNE: Transfère la propriété du User créé vers l'appelant (main)
   - Après return, name et sexe sont "consumés" par le User retourné

5. FONCTION AFFICHE_LE_TEXTE() :
   - fn affiche_le_texte(nom:String, ages:u8, orientation:String)
   - OWNERSHIP: Reçoit la PROPRIÉTÉ de nom et orientation
   - Une fois la fonction terminée, nom et orientation sont DROPPÉS (libérés de la mémoire)
   - ages est copié (u8 implémente Copy)

6. FLUX OWNERSHIP DANS CE PROGRAMME :
   main() crée variables
      ↓ MOVE
   users_informations() reçoit ownership
      ↓ MOVE (dans User)
   main() récupère ownership du User
      ↓ DESTRUCTURE + MOVE  
   affiche_le_texte() reçoit ownership
      ↓ FIN DE SCOPE
   Variables DROPPÉES automatiquement

7. POINTS IMPORTANTS :
   - String = ownership (déplacement nécessaire)
   - &str = borrowing (pas de déplacement)  
   - u8 = Copy (duplication automatique)
   - Une fois qu'une variable est MOVE, elle n'est plus utilisable
   - Rust empêche les use-after-move à la compilation

8. POURQUOI ÇA MARCHE :
   - user.name, user.age, user.sexe → DESTRUCTURE le User
   - Chaque champ est MOVE individuellement vers affiche_le_texte()
   - Pas de double-free ou memory leak possible !

9. ALTERNATIVE AVEC BORROWING :
   - Si on voulait garder user après, on ferait :
   - affiche_le_texte(&user.name, user.age, &user.sexe);
   - Ou mieux : affiche_le_texte(&user); et changer la signature

10. RÈGLES OWNERSHIP RESPECTÉES :
    ✅ Chaque valeur a UN SEUL propriétaire
    ✅ Quand le propriétaire sort de scope, la valeur est drop
    ✅ Pas de dangling pointers possibles
    ✅ Memory safety garantie à la compilation !
*/