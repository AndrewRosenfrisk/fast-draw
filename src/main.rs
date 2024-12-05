use std::{io::stdin, thread, time::Duration};

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    'main: loop {
        let sleep_time = (rng.gen_range(20..=50) / 10) as f64;

        println!("It's high noon...");

        thread::sleep(Duration::from_secs_f64(sleep_time));

        println!("DRAW!!!");

        let test = std::time::Instant::now();

        get_input();

        let elapsed = test.elapsed().as_secs_f64();

        if elapsed < 0.01 {
            println!("You drew before \"DRAW\" appeared. You lose!");
        }
        if elapsed > 0.3 {
            println!("You took {:.4} seconds to draw. Too slow!", elapsed);
        } else {
            println!("You took {:.4} seconds to draw.", elapsed);
            println!("You are the fastest draw in the west. You win!!!!");
        }

        println!("[Q] to quit. Enter to continue.");
        let quit_prompt = get_input();
        if quit_prompt.trim().to_uppercase() == "Q" {
            println!("Thanks for playing!");
            break 'main;
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading input");
    input
}
