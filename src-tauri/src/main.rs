// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use serde_json::Value;
use std::fs;
use std::path::Path;
use strsim::normalized_levenshtein;

static mut VERSES: Vec<String> = Vec::new();

#[tauri::command]
fn get_answer(answer: &str, memory_verse: &str) -> (String, String, bool) {
    let percent_correct: u32 = (normalized_levenshtein(answer, memory_verse) * 100.0) as u32;
    let correct_answer: (String, bool) = match percent_correct {
        100 => ("Perfect! That's the correct answer.".to_string(), true),
        90..=99 => (
            "Very good! You almost have this verse memorized!".to_string(),
            false,
        ),
        80..=89 => (
            "Good! You're on the right track to memorizing this verse.".to_string(),
            false,
        ),
        70..=79 => ("Not bad! Keep trying and you'll get it!".to_string(), false),
        60..=69 => (
            "Not quite. Try again and you'll get it next time.".to_string(),
            false,
        ),
        _ => ("Sorry, that's not correct. Keep trying.".to_string(), false),
    };
    (
        format!("{}%", percent_correct),
        format!("{}", correct_answer.0),
        correct_answer.1,
    )
}

#[tauri::command]
fn get_new_question() -> String {
    let mut rng = rand::thread_rng();
    unsafe {
        let random_index = rng.gen_range(0, VERSES.len());
        VERSES[random_index].clone()
    }
}

#[tauri::command]
fn setup_verses(handle: tauri::AppHandle) {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("aruljohn-Bible-kjv")
        .expect("failed to resolve resource");

    for entry in fs::read_dir(&resource_path).expect("Something went wrong reading the directory") {
        match entry {
            Err(e) => println!("{}", e),
            Ok(file) => {
                if let Some(path) = file.path().extension() {
                    // Check if it's a JSON file by checking its extension
                    if path == "json" {
                        read_json(&file.path())
                    } else {
                        println!("{} is not a JSON file", file.path().display());
                    }
                }
            }
        };
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_answer,
            get_new_question,
            setup_verses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn read_json(path: &Path) {
    let file = std::fs::File::open(&path).expect("Unable to open JSON file.");
    let reader = std::io::BufReader::new(file);

    // Read JSON from file
    let value: Value = serde_json::from_reader(reader).expect("Unable to parse JSON");
    let chaps: Value = value
        .get("chapters")
        .expect("Unable to find chapters in JSON")
        .to_owned();
    if let Some(i) = chaps.as_array() {
        for n in 0..i.len() {
            let vers = i[n].get("verses").unwrap();
            if let Some(j) = vers.as_array() {
                for k in j {
                    let v = rem_first_and_last(k.get("text").unwrap().to_string());
                    unsafe {
                        VERSES.push(v);
                    }
                }
            }
        }
    }
}

fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
