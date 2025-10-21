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
    println!("Le nom de la personne est {}", user.name);
    println!("L'âge de la personne est {}", user.age);
    println!("Le sexe de la personne est {}", user.sexe);
}


fn affichage_du_texte(texte: &str) {
    println!("Voici vôtre texte : {}", texte);
    println!("Voici vôtre texte : {}", texte);

        // Fin de la fonction      
}


fn calculer(a: i32) -> i32 {
    a * 2
    
}

fn users_informations(name: String, age: u8, sexe: String) -> User {
    let user = User {
        name: name,
        age: age,
        sexe: sexe
    };

    user

}