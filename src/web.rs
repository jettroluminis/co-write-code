use warp::Filter;
use tokio::sync::Mutex;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
struct GameState {
    secret_number: u32,
    output: Arc<Mutex<String>>,
}

#[tokio::main]
pub async fn main() {
    let game_state = GameState {
        secret_number: rand::thread_rng().gen_range(1..=100),
        output: Arc::new(Mutex::new(String::new())),
    };

    let game_state_filter = warp::any().map(move || game_state.clone());

    let index_route = warp::path::end()
        .and(warp::fs::file("static/index.html"));

    let guess_route = warp::path("guess")
        .and(warp::post())
        .and(warp::body::form())
        .and(game_state_filter.clone())
        .and_then(handle_guess);

    let routes = index_route.or(guess_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_guess(form: std::collections::HashMap<String, String>, game_state: GameState) -> Result<impl warp::Reply, warp::Rejection> {
    let guess: u32 = match form.get("guess").and_then(|g| g.parse().ok()) {
        Some(num) => num,
        None => return Ok(warp::reply::html("Invalid input")),
    };

    let mut output = game_state.output.lock().await;

    output.push_str(&format!("You guessed: {}\n", guess));

    match guess.cmp(&game_state.secret_number) {
        std::cmp::Ordering::Less => output.push_str("Too small!\n"),
        std::cmp::Ordering::Greater => output.push_str("Too big!\n"),
        std::cmp::Ordering::Equal => {
            output.push_str("You win!\n");
            *output = String::new();
        }
    }

    Ok(warp::reply::html(&output.clone()))
}
