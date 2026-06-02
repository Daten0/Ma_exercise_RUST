// 🧠 Your Practice Blueprint

// To lock in this concept before moving on to the Ownership model, open your local workspace or Rust Playground and build a tiny system matching these specifications:

//     Create an enum called CacheState with three variants:

//         Empty

//         Expired

//         Hit(u32) (contains a data size value)

//     Write a function that takes CacheState and uses a match expression to return a string action description.

//     If it's a Hit, use a Match Guard to check if the data size is larger than 1024. If it is, return "Flush heavy payload". If it's a hit but smaller than 1024, return "Read payload directly".

//     In main, write an if let statement that checks only if a CacheState variable is Expired, printing a warning if it is.
#[derive(PartialEq)]
enum CacheState {
    Empty,
    Expired,
    Hit(u32),
}

fn cache_state(cache: CacheState) -> String {
    match cache {
        // Using a clean Match Guard condition
        CacheState::Hit(num) if num > 1024 => "Flush heavy payload".to_string(),
        CacheState::Hit(_) => "Read payload directly".to_string(),
        
        CacheState::Empty => "It's an empty cache".to_string(),
        CacheState::Expired => "Expired".to_string(),
    }
}

fn main() {
    // 1. Let's test the match function with a heavy payload
    let heavy_cache = CacheState::Hit(2048);
    // println!("Action required: {}", cache_state(heavy_cache));

    // 2. Let's test your if let target by setting it to Expired
    let current_state = CacheState::Expired;

    // Correct pattern matching layout:
    if let CacheState::Expired = current_state {
        println!("This is a Warning: Cache state is currently Expired!");
    }

    let empty_cache = CacheState::Empty;
    // println!("This is the empty state : {}", cache_state(empty_cache));

    if heavy_cache == CacheState::Hit(1024) {
        println!("The warning : {}",cache_state(heavy_cache))
    } else if current_state == CacheState::Expired {
        println!("The warning : {}", cache_state(current_state))
    } else if current_state == CacheState::Empty {
        println!("The warning : {}", cache_state(current_state))
    }
}
