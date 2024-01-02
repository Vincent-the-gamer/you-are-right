use serde_json::{from_str, Value, json};
use rand::Rng;

const MEMES: &str = include_str!("memes.json");

pub fn get_random_meme() -> String {
   let memes_list: Vec<Value> = from_str(MEMES).expect("can't read json.");
   let mut random = rand::thread_rng();

   // get random number from 0 to memes_list.len()
   let index = random.gen_range(0..memes_list.len());
   let result = memes_list[index].as_str().unwrap();

   result.to_string()
}

pub fn get_meme_json() -> Value {
    json!({ 
        "content": get_random_meme()
    })
}