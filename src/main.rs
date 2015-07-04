extern crate rustless;
extern crate iron;
extern crate hyper;
extern crate valico;
extern crate rustc_serialize;
extern crate radix_trie;

use radix_trie::Trie;
use rustless::prelude::*;
use std::io::Read;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use rustc_serialize::json;
use valico::json_dsl;
use iron::Iron;
use rustless::{
    Application, Api
};
use rustless::server::header;

const MIN_RELEVANCE : u8 = 1;

fn main() {
    let words = get_words();
    let not_relevants = get_not_relevants();
    let api = Api::build(|api| {

        api.post("keywords", |endpoint| {


            endpoint.params(|params| {
                params.req_typed("text", json_dsl::string());
            });

            endpoint.handle(move |mut client, params| {

                client.set_header(header::ContentType::json());

                let mut current = String::with_capacity(512);

                let text = params.find("text").unwrap().to_string();
                let mut chars = text.trim_matches('"').chars();

                match chars.next() {
                    Some(c) => current.push(c),
                    None => return client.text("{}".to_string())
                }
                
                let decomposed = chars.fold(
                    Vec::new(),
                    |mut v, c| {
                        // if 'current' can't be extended and is already a word
                        // by itself
                        if is_unique_match(&words, &current) {
                            v.push(current.clone());
                            current.clear();
                            current.push(c);
                            return v;
                        }
                        
                        // if curren't word can be extended to a valid word
                        if word_can_get_any_longer(&words, &current) {
                            // TODO we should fallback to the last valid word we've
                            // seen instead of just "pushing" 'current'

                            // but the current+1 word  can't be extended
                            let mut next = current.clone();
                            next.push(c);
                            if !word_can_get_any_longer(&words, &next) {
                                v.push(current.clone());
                                current.clear();
                            }

                            current.push(c);
                            return v;
                        }

                        let mut current_char = String::new();
                        current_char.push(c);

                        // if there's a least a word starting by the value
                        // of 'current character'
                        if word_can_get_any_longer(&words, &current_char) {
                            v.push(current.clone());
                            current.clear()
                        }

                        current.push(c);
                        v
                    }
                ); 

                let words_count : HashMap<String, u8> = decomposed.iter().fold(
                    HashMap::new(),
                    |mut m, c| {
                        if !not_relevants.contains(c) {
                            *m.entry(c.clone()).or_insert(0) += 1;
                        }
                        m
                    }
                );

                let relevants : HashMap<&String, &u8> = words_count
                    .iter().
                    filter(|&(&_, &v)| v > MIN_RELEVANCE).collect();

                client.text(json::encode(&relevants).unwrap())
            })
        });

    });

    let app = Application::new(api);
    match Iron::new(app).http("0.0.0.0:4000") {
        Ok(_) => println!("listening on port 4000"),
        Err(_) => println!("failed to open port 4000, is it already taken?")
    }
}

fn is_unique_match(words: &Trie<String, ()>, value: &String) -> bool {
    words.get_node(value).and_then(|node| Some(node.is_leaf())).unwrap_or(false)
}

fn word_can_get_any_longer(words: &Trie<String, ()>, value: &String) -> bool {
    words.get_descendant(value).is_some()
}
fn get_words() -> Trie<String, ()>  {

    let mut s = String::new();
    let _  = File::open("data/words.json").unwrap().read_to_string(&mut s);

    let words : Vec<String> = json::decode(&s).unwrap();

    let mut x = Trie::new();
    for word in words.iter() {
        x.insert(word.clone(), ());
    } 
    x
}

fn get_not_relevants() -> HashSet<String>  {

    let mut s = String::new();
    let _  = File::open("data/not_relevants.json").unwrap().read_to_string(&mut s);

    json::decode(&s).unwrap()
}
