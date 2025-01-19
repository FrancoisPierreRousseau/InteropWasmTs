use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, XmlHttpRequest};

// Ts: wasm-pack build   --target web --out-dir ..\ProjetTypescript\pkg (wasm_bindgen)
// C# :cargo build (no_mangle)

#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn fetch_data() {
    let request = Rc::new(RefCell::new(XmlHttpRequest::new().unwrap()));
    request
        .borrow_mut()
        .open("GET", "https://jsonplaceholder.typicode.com/posts")
        .unwrap();

    let request_clone = request.clone();
    let onload_callback = Closure::wrap(Box::new(move || {
        let request_borrowed = request_clone.borrow();
        match request_borrowed.status() {
            Ok(status) => {
                let response_text = request_borrowed
                    .response_text()
                    .unwrap_or_else(|_| Some("No response".to_string()));
                console::log_1(&format!("Response: {:?}", response_text).into());
            }
            Err(err) => {
                console::log_1(&format!("Failed to get status: {:?}", err).into());
            }
        }
    }) as Box<dyn Fn()>);

    request
        .borrow_mut()
        .set_onload(Some(onload_callback.as_ref().unchecked_ref()));
    request.borrow_mut().send().unwrap();
    onload_callback.forget();
}

#[wasm_bindgen]
pub fn call_csharp_code() -> String {
    // Charger la bibliothèque dynamique (DLL)
    // let lib = Library::new("C:\\Users\\franc\\source\\repos\\TsC#Intterop\\BibliothequeCallByRust\\bin\\Release\\net8.0\\win-x64\\publish\\BibliothequeCallByRust.dll")
    //   .expect("Impossible de charger la bibliothèque");

    // Définir le type de la fonction exportée
    // let add_numbers: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
    //    lib.get(b"add_numbers").expect("Impossible de trouver la fonction 'add_numbers'");

    // Appeler la fonction exportée
    // let result = add_numbers(5, 7);

    // Retourner le résultat sous forme de chaîne
    format!("Le résultat de l'addition est : {}", "hello from rust")
}
