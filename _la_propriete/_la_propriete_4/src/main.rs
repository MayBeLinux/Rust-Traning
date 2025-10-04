fn main() {
    let reference_to_nothing = dangle(); // on essaye de récupérer une référence à une variables qui n'existes plus.

}
fn dangle() -> &String {
    let s = String::from("hello"); // s est crée ici et est valide jusqu'à la fin de la fonction dangle

    &s // s est hors de portée ici car la variables s est détruite à la fin de la fonction dangle(). 
}