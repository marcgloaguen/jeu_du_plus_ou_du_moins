use std::io;

fn main() {
    println!("Devinez le nombre !");

    println!("Veuillez entrer un nombre.");

    let mut supposition = String::new(); // mut = mutable

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}
