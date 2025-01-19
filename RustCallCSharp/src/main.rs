use libloading::{Library, Symbol};

// Utilise BibliothequeCallByRust
// cargo build 
// cargo run
fn main() {
    unsafe {
        let lib = Library::new("C:\\Users\\franc\\source\\repos\\TsCSharpIntterop\\BibliothequeCallByRust\\bin\\Release\\net8.0\\win-x64\\publish\\BibliothequeCallByRust.dll").expect("Impossible de charger la bibliothèque");
        
        // Définir le type de la fonction exportée
        let add_numbers: Symbol<unsafe extern fn(i32, i32) -> i32> =
        lib.get(b"add_numbers").expect("Impossible de trouver la fonction 'add_numbers'");

        // Appeler la fonction exportée
         let result = add_numbers(5, 7);
         println!("Le résultat de l'addition est : {}", result);
        }
}
