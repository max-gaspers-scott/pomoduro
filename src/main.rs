use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::collections::btree_map::Range;
use std::ffi::os_str::Display;
use std::fs::File;
use std::thread;

use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    println!("how may munuts do you want to work");
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

        let seconts_left = seconts - duration;
        let display_minust = seconts_left / 60;
        let display_seconts = seconts_left % 60;
        println!("seconts: {:?}", seconts_left);
        println!("minuts: {}, seconts: {}", display_minust, display_seconts);

        thread::sleep(Duration::from_secs(1));
    }

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    // Load a sound from a file, using a path relative to Cargo.toml }
    let file = File::open("scream.mp3").unwrap();
    // Decode that sound file into a source
    let source = Decoder::try_from(file).unwrap();
    // Play the sound directly on the device
    handle.mixer().add(source);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
