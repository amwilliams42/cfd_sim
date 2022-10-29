use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn test() -> String{
    String::from("Hello from rust!")
}
