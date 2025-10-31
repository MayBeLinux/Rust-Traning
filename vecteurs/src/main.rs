

#[derive(Debug)]

struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

fn main() {
    let rect1 = Rectangle {
        largeur: 32,
        hauteur: 32,
    };

    println!("Le carré fait {}" , calc_carre(&rect1));


}

fn calc_carre(cote: &Rectangle) -> u32 { // Il faut passer en paramètre une référence de la struct et non la variables car la fonction ne possède pas la propriétée de la variable
    cote.largeur * cote.hauteur

}