
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
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
       let nombre = Arc::new(10);

       let a = Arc::clone(&nombre);

       let handle = 
       thread::spawn(move || {
        println!("travaill en cour... resultat : {}", a);

       });

       handle.join().unwrap();

       println!("{}",nombre);

      
    }
    
    
