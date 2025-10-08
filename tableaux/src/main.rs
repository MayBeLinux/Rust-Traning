use std::io; // ajout de la bibliothèques standard io pour gérer les entrée : sortie.

fn main() {
   let month = ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", "Juillet", "Aout", "Septembre", "Octobre", "Novembre", "Décembre"]; // Ecriture d'une table de 12 index ( Les mois de l'années).

   println!("{:?}" , month); // Utilisation de :? pour afficher le tableau.

   // {} → affiche une valeur en utilisant le trait Display.
   // → C’est pour les types “jolis” destinés à l’utilisateur (nombres, chaînes, char, bool, etc.).

   // {:?} → affiche une valeur en utilisant le trait Debug.
   // → C’est pour du “debugging”, une représentation brute et technique, souvent pas super jolie, mais très complète.

   //--------------------------------------------------------------------------------------------------------------------------------

   let a: [i32; 5] = [1, 2, 3, 4, 5]; // Création d'un tableau avec le type i32 (qui est le type par défaut dans Rust) est juste après la taille du tableau donc [types; taille] = [i64; 12] / Puis après l'affectation des valeurs.
   println!("Tableaux A : {:?}" , a); // Ecriture du tableau dans le retour de commande utilisateur avec ":?




   //------------------------------------------------------------------------------------------------------------------------------------
   let a = [3; 5]; // Cela va créer une table ressemblant a ceci [3, 3, 3, 3, 3]
   // [valeurs ajouté ; nombre fois dans la table]
   println!("Tableaux A : {:?}" , a); // Ecriture du tableau dans le retour de commande utilisateur avec ":?

   // Un tableau est un bloc de mémoire de taille fixe et connue, pouvant être alloué sur la pile. L'indexation permet d'accéder aux éléments d'un tableau, comme suit :

   let a = [1, 2, 3, 4, 5];

   let first = a[0];
   let second = a[1];
   let third = a[2];

   println!(" The first {}" , first);
   println!("The second {}" , second);
   println!("The second {}" , third);

   
   
   //--------------------------------------------------------------------------------------------------------------------------------------------------
   // Essaie de voir comment réagis le compilateur quand l'utilisateur rentre une valeur qui n'existe pas dans le tableau des valeurs.
   let a = [1, 2, 3, 4, 5];

   println!("Please insert an index values.");

   let mut index = String::new();

   io::stdin()
   .read_line(&mut index)
   .expect("Failed to read line");

   let index: usize = index
   .trim()
   .parse()
   .expect("Index entered was not a number");

   let element = a[index];

   println!("The value of the element at index {index} is: {element}");
}

