# 🧵 Multi-threading en Rust

Ce projet montre comment utiliser **les threads en Rust** pour exécuter des tâches en parallèle et partager des données de manière sécurisée avec **Arc (Atomic Reference Counting)**.  

Il est conçu comme un **exemple pédagogique** pour comprendre les concepts suivants :

- Threads
- Partage sécurisé de données
- Synchronisation entre threads

---

## 📌 Concepts clés

1. **Thread** : unité d’exécution indépendante pour exécuter plusieurs tâches simultanément.  
2. **Arc** : permet de partager une donnée entre plusieurs threads de manière sûre.  
3. **Join** : attendre la fin d’un thread avant de continuer le programme principal.

---

## 💻 Exemple de code

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    // Création d'une valeur partagée entre plusieurs threads
    let nombre = Arc::new(10);

    // On clone la référence Arc pour l’envoyer dans un thread
    let a = Arc::clone(&nombre);

    // Création d'un nouveau thread
    let handle = thread::spawn(move || {
        println!("Travail en cours... résultat : {}", a);
    });

    // Attendre la fin du thread
    handle.join().unwrap();

    // La valeur originale est toujours accessible dans le thread principal
    println!("{}", nombre);
}
