use tokio;
use serde_json;
use crate::models::AppState;
use std::collections::HashMap;
use std::io::Error;
use rand::Rng;

pub async fn write_to_file(state: &AppState) -> std::io::Result<()> {
    let json_content = serde_json::to_string_pretty(state)?;
    tokio::fs::write("data.json", json_content).await?;
    Ok(())
}

pub async fn load_data_from_file() -> std::io::Result<AppState> {
    // Check if file exists
    if tokio::fs::metadata("data.json").await.is_ok() {
        let content = tokio::fs::read_to_string("data.json").await?;
        let state: AppState = serde_json::from_str(&content)?;
        println!("Loaded existing data from data.json");
        Ok(state)
    } else {
        // Create new empty state if file doesn't exist
        println!("No data.json found, creating new file with empty state");
        let empty_state = AppState {
            users: HashMap::new(),
            orders: HashMap::new(),
            items: HashMap::new(),
        };

    // write the json file into the newly created file
    let empty_state_json = match  serde_json::to_string_pretty(&empty_state) {
        Ok(json) => json,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to serialize: {}", e)
            ));
        }
    };
    tokio::fs::write("data.json", empty_state_json).await?;

    println!("Created data.json successfully");
    Ok(empty_state)
    }
}

pub fn generate_random_array() -> Vec<i32> {
    let mut rng = rand::rng();
    let size = rng.random_range(5..=50);
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        let value = rng.random_range(-100..=100);
        vec.push(value);
    }
    vec
}

// custom bubble sort algorithm function
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut swap_count = 0;
    for i in 0..n {
        for j in 0..(n-i-1) {
            if arr[j] > arr[j + 1] {
                let temp_holder = arr[j]; // temporary holder for the item at the j index, since i32 implements the copy trait
                arr[j] = arr[j + 1]; // item at 1 index after, since it's higher gets swapped to the smaller one beside it
                arr[j + 1] = temp_holder;

                swap_count += 1
            }
        }
    }

    println!("Number of swaps performed: {}", swap_count);
}