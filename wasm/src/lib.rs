use get_meme::get_meme::get_random_meme;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn meme() -> String {
    get_random_meme()
}