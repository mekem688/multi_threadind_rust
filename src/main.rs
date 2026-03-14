
use std::thread;
use std::sync::Arc;

fn main() {
    /*  println!("Hello, world!");
    let t = thread::spawn(||{
        println!("thread en cours...");
    });
    t.join().unwrap();
    println!("Fin du programme ");
    let compteur = Mutex::new(0);
    {
        let mut valeur = compteur.lock().unwrap();
        *valeur +=1 ;
    }
    let mut handles = vec![];
    println!("compteur= {:?}",compteur);
    let nombre = Arc::new(10);
    for _ in 0..5 {
        let n = Arc::clone(&nombre);
        let handle =
        thread::spawn(move || {
            println!("{}",n);
        });
        handles.push(handle);
    }
    for handle in handles {
            handle.join().unwrap();
        } */
      // Import du module thread de la bibliothèque standard
// Il permet de créer et gérer des threads
use std::thread;

// Import de Arc (Atomic Reference Counting)
// Arc permet de partager une donnée entre plusieurs threads en toute sécurité

    // Création d'une valeur partagée entre plusieurs threads
    // Arc permet d'avoir plusieurs propriétaires de la même donnée
    let nombre = Arc::new(10);

    // On clone la référence Arc
    // Attention : cela ne copie PAS la valeur 10
    // Cela augmente simplement le compteur de références
    let a = Arc::clone(&nombre);

    // Création d'un nouveau thread
    // thread::spawn lance un thread qui exécute la closure
    let handle =
    thread::spawn(move || {

        // "move" déplace la propriété de la variable "a"
        // dans le thread pour qu'il puisse l'utiliser
        println!("travail en cours... resultat : {}", a);

    });

    // join() permet d'attendre que le thread se termine
    // Sans join(), le programme pourrait se terminer avant la fin du thread
    handle.join().unwrap();

    // Le thread principal peut toujours accéder à "nombre"
    // car Arc permet plusieurs références vers la même donnée
    println!("{}", nombre);

}
    
    
    
