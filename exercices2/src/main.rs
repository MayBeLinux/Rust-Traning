use std::io;

fn main() {
    println!("Entrer un nombre svp");

    let mut nombre = String::new();
    let afficher = io::stdin().read_line(&mut nombre).expect("Echec impossible de lire le texte");

    println!("taille de {} afficher {} ", afficher , nombre); // nombre est un type String

    choisir(&nombre.trim()); //trim() pour enlever le \n ou toutes autres espaces que l'on ne souhaite pas dans nôtre String
}

fn choisir(nombre: &str) {
    let result = match nombre {
        "1" => String::from("Vous avez selectionné le nombre 1"),
        "2" => String::from("Vous avez selectionné le nombre 2"),
        "3" => String::from("Vous avez selectionné le nombre 3"),
        "19" => String::from("Vous avez selectionné le nombre 19"),
        _ => String::from("Vous n'avez pas selectionné un nombre valide"),
    };
    affichage_du_texte(&result);
}


fn affichage_du_texte(texte: &str) {
    
        println!("Voici vôtre texte : {}", texte);

        // Fin de la fonction  
        // Reprise Demain le 17/10/2025                 
    }