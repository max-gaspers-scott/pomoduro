use std::collections::btree_map::Range;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let mut minuts = String::new();
    match std::io::stdin().read_line(&mut minuts) {
        Ok(_) => {}
        Err(e) => {
            println!("error: {e}")
        }
    }
    let seconts = minuts
        .trim()
        .parse::<u64>()
        .expect("that was not a valid u64 number")
        * 60;

    let start = SystemTime::now();
    let mut duration = 0;
    while duration < seconts {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        let now = SystemTime::now();
        duration = now
            .duration_since(start)
            .expect("error getting duration_since")
            .as_secs();

        println!("seconts: {:?}", seconts - duration);

        thread::sleep(Duration::from_secs(1));
    }
    // play::play("sound.mp3").unwrap();
}
