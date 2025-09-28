# 🦀 Rust Training Grounds

Bienvenue dans **ton dojo Rust**. Ici, on affûte la syntaxe comme une lame, on dompte le borrow checker comme un dragon capricieux, et on apprend à aimer les `Result<T, E>` plus que le café du matin.  

---

## 🚀 Objectif du dépôt
Ce dépôt rassemble :  
- Des notes de formation pour progresser pas à pas en Rust  
- Des exemples de code réutilisables au quotidien  
- Des rappels rapides pour les commandes et concepts essentiels  
- Des ressources pour ne jamais se perdre dans les méandres du système de types  

---

## 📒 Les bases Cargo
```Cargo
# Créer un projet binaire
cargo new mon-projet
```
```Cargo
# Créer une bibliothèque
cargo new --lib ma-lib
```
```Cargo
# Compiler et exécuter
cargo run
```
```Cargo
# Vérifier sans exécuter
cargo check
```
```Cargo
# Ajouter une dépendance depuis crates.io
cargo add serde
```
```Cargo
# Mettre à jour les dépendances
cargo update
```
---

## Petits Rappels
// Immuable par défaut
let x = 42;

// Mutable
let mut y = 0;

// Boucle classique
for i in 0..5 {
    println!("i = {}", i);
}

// Gestion d'erreurs façon "samouraï zen"
fn div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division par zéro 🌀".into())
    } else {
        Ok(a / b)
    }
}

// Option : quand une valeur peut exister... ou pas
fn premier(vec: &Vec<i32>) -> Option<i32> {
    vec.get(0).copied()
}


## CheetSheet Express 

| Besoin                | Solution en Rust                           |
| --------------------- | ------------------------------------------ |
| Valeur immuable       | `let x = 5;`                               |
| Valeur mutable        | `let mut x = 5;`                           |
| Fonction              | `fn ma_fonction(x: i32) -> i32 { x + 1 }`  |
| Struct                | `struct Point { x: i32, y: i32 }`          |
| Enum                  | `enum Couleur { Rouge, Vert, Bleu }`       |
| Pattern matching      | `match option { Some(v) => v, None => 0 }` |
| Erreur gérée          | `Result<T, E>`                             |
| Smart pointer partagé | `Rc<T>`                                    |
| Concurrence sûre      | `Arc<Mutex<T>>`                            |



## Trucs & pièges pièges fréquents

### Ownership & Borrowing

- Une variable possède sa donnée → quand elle sort du scope, la donnée est libérée.
- Une donnée ne peut avoir qu’un seul propriétaire.
- Les références (&) permettent d’emprunter sans prendre la possession.

### Règles d’emprunt

- Tu peux avoir plusieurs références immuables (&) en même temps.
- Tu ne peux avoir qu’une seule référence mutable (&mut) à la fois.
- Pas de mélange : si tu as une référence mutable active, tu ne peux pas avoir une référence immuable en même temps.

###Lifetimes

- Les lifetimes ('a) permettent d’exprimer la durée de vie d’une référence.
- Le compilateur t’aide à éviter les “dangling references” (références vers des données déjà libérées).


### Exemple simple : 
```Rust
fn plus_long<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 🔗 Ressources sacrées

📖 Le Rust Book (la bible vivante du langage) : https://doc.rust-lang.org/book/
📚 Rust By Example : https://doc.rust-lang.org/rust-by-example/
📦 The Cargo Book : https://doc.rust-lang.org/cargo/
🦀 Rustlings (petits exercices pratiques) : https://github.com/rust-lang/rustlings






