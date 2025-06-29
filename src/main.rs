use std::io;
use std::cmp::Ordering;
use rand;

fn main() {
    let nombre_secret = rand::random_range(0..=100);

    println!("Le nombre secret est : {}", nombre_secret);

    println!("Devinez le nombre !");

    loop {
        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecteur de l'entrée de l'utilisateur.");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        println!("Votre nombre : {}", supposition);

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("Le nombre secret est plus grand."),
            Ordering::Greater => println!("Le nombre secret est plus petit"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            },
        }
        println!("Veuillez saisir un nombre.");
    }    
}