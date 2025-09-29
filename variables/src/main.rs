fn main() {
    //let x = 5;
    //println!("The value of x is : {}", x); // La variable x est immuable par défaut. 

    //x = 6;
    //println!("The value of x is : {}" , x); // L'erreur crée ici est que la variables x est immuable par défaut. Pour rendre une variable mutable, il faut utiliser le mot-clé mut.

    let mut y = 5;
    println!("The value of y is : {}", y); // La variable y est mutable grâce au mot-clé mut.

    y = 6;
    println!("The value of y is : {}" , y); // La variable y est mutable grâce au mot-clé mut, donc la valeur actuelle est 6 et non plus 5.

    let z = 5;

    let z = z + 1; // Ici, on crée une nouvelle variable z qui masque l'ancienne variable z. C'est ce qu'on apelle le shadowing.

    {
        let z = z * 2; // Ici, on crée une nouvelle variable z qui masque l'ancienne variable z.
        println!("The value of z in the inner scope is : {}", z); // La valeur de z dans ce scope est 12 (6 * 2).
    }

    println!("The value of z is : {}", z); // La valeur de z dans ce scope est 6 (5 + 1). // La variable z est redevenue 6 car on est sorti du scope intérieur qui déclaré la variable z à 12. 



    //----------------------------------------------------------------------------------------------------------
    // Variables à virgules flottantes
    let w = 2.0; //F64
    let e: f32 = 3.0; //F32


    // ---------------------------------------------------------------------------------------------------------
    // Différents calcul mathématiques de base !
    
    //addition 
    let _sum = 5 + 10;

    // soustraction
    let _soustraction = 5 - 10;

    // multiplication
    let _multiplication = 5 * 10;

    // division
    let _division = 5 / 10;

    // reste
    let _reste = 5 % 10;


    //----------------------------------------------------------------------------------------------------------
    // Les booléen 

    let _t = true;

    let _f: bool = false; // Le type booléen prend 1 octet en mémoire !
    
    

    //-----------------------------------------------------------------------------------------------------------
    // Les tuples 

    let _tuples: (i32, f64, u8) = (500, 6.4, 1); // Ceci assigne les types (a gauche) au valeurs inscrites (a droite) comme par exemple i32 = 500 comme f64 = 6.4 


    // Nous pouvons donc utiliser une méthodes que l'on appelle la Déstructuration pour récupérer les valeurs individuellement dans chaques variables distinctes.
    // Déstructuration 1
    let new_tuples = (500, 6.4, 1);  // La je viens d'assigner que x prend 500 / y prend 6.4 / z prend 1

    {
    // Nous pouvons aussi accéder a chaque variables en utilisant leur index dans le tuples comme exemple ci dessous/
    println!("Tuple X {}" , new_tuples.0); // Va afficher 500
    println!("Tuple Y {}" , new_tuples.1); // Va afficher 6.4
    println!("Tuple Z {}" , new_tuples.2); // Va afficher 'z'
    }
}



// Scope est la définition de la portée d'une variable. Une variable est accessible uniquement dans le scope où elle a été définie et dans les scopes imbriqués à l'intérieur de ce scope.
// Un scope est délimité par des accolades {}.
// Une variable définie dans un scope intérieur masque une variable du même nom définie dans un scope extérieur.
// Une variable définie dans un scope extérieur est accessible dans un scope intérieur.