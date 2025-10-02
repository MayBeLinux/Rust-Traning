fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 2;
  }

  index(); // appel de la fonction index.
  inversement(); // appel de la fonction inversement.
}

fn index() {

    let mut a = [1, 2, 3, 4, 5,]; // Ceci crée un tableau de 5 éléments dans la variable a qui est mut (modifiable).
    let mut index = 0; // cette variable index est permet de parcourir le tableau.

    while index < 5 { // tant que index est inférieur à 5 (la taille du tableau).
        println!("The value is : {}", a[index]); // on affiche la valeur de l'élément du tableau à la position index.

        index += 1; // on incrémente index de 1 pour passer à l'élément suivant du tableau.
    }
}

fn inversement() {
    for number in (1..4).rev() { // .rev() permet d'inverser l'ordre de la plage 1 à 4 (exclu).
    // .rev() s'appelle une méthode (function) qui est appelée sur l'objet (1..4) qui est une plage.
    // une méthode en Rust s'appelle avec un point (.) suivi du nom de la méthode et des parenthèses ().
        println!("{number}");

    }
    println!("List off");
}