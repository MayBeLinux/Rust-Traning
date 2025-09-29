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
    let sum = 5 + 10;

    // soustraction
    let soustraction = 5 - 10;

    // multiplication
    let multiplication = 5 x 10;

    // division
    let division = 5 / 10;

    // reste
    let reste = 5 % 10;


    //----------------------------------------------------------------------------------------------------------
    // Les booléen 

    let t = true;

    let f: bool = false; // Le type booléen prend 1 octet en mémoire !
    
    

    //-----------------------------------------------------------------------------------------------------------
    // Les tuples 

    let tuples: (i32, f64, u8) = (500, 6.4, 1);
    
    }



// Scope est la définition de la portée d'une variable. Une variable est accessible uniquement dans le scope où elle a été définie et dans les scopes imbriqués à l'intérieur de ce scope.
// Un scope est délimité par des accolades {}.
// Une variable définie dans un scope intérieur masque une variable du même nom définie dans un scope extérieur.
// Une variable définie dans un scope extérieur est accessible dans un scope intérieur.