// Bring in serialization traits (add serde_json in Cargo.toml)
// In Cargo.toml, add: serde = { version = “1.0”, features = [”derive”] }, serde_json = “1.0”

use serde::{Deserialize, Serialize};
// Define the macro. We can allow one or multiple expressions.
#[macro_export]
macro_rules! json_macro {
    // Match one or more comma-separated expressions
    ( $($val:expr),+ $(,)? ) => {
        $(
            // Only print when compiling in debug mode
            if cfg!(debug_assertions) {
                // Serialize to pretty JSON and print.
                // .unwrap() will panic if serialization fails (we could use expect or better error handling in real code).
                println!("{}", serde_json::to_string_pretty(&$val).unwrap());
            }
        )*
    };
}



#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    store_name: String,
    items: Vec<Item>,
}

fn main() {
    let inventory = Inventory {
        store_name: "Potion Shop".to_string(),
        items: vec![
            Item {
                name: "Health Potion".into(),
                price: 10.0,
            },
            Item {
                name: "Mana Potion".into(),
                price: 12.5,
            },
        ],
    };

    // Use our debug-printing macro
    json_macro!(inventory);
}
