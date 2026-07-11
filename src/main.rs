use rodio::{Decoder, MixerDeviceSink, source::Source};

use std::fs::File;
use std::process::Command;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    println!("use this commad to record your work  ");
    println!("asciinema record 7-9_6pm.cast ");
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
        .parse::<i32>()
        .expect("that was not a valid u64 number")
        * 60;

    let start = SystemTime::now();
    let (tx, rx) = channel::<()>();
    let join_handel = std::thread::spawn(move || {
        let mut input = String::new();
        // This blocks the background thread until the user presses Enter
        let _ = std::io::stdin().read_line(&mut input);
        // Send a signal back to the main thread
        let _ = tx.send(());
    });

    let mut duration: i32 = 0;
    loop {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        let now = SystemTime::now();
        duration = now
            .duration_since(start)
            .expect("error getting duration_since")
            .as_secs()
            .try_into()
            .unwrap();

        let seconts_left = seconts - duration;
        let display_minust = seconts_left / 60;
        let display_seconts = seconts_left % 60;
        println!("minuts: {}, seconts: {}", display_minust, display_seconts);

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {
                println!("\nStopped early by user!");
                println!("worked for: {display_minust}:{display_seconts}");
                break;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Just timed out, continue standard loop execution
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                // The input thread disconnected/panicked, break
                println!("worked for: {display_minust}:{display_seconts}");
                break;
            }
        }
        if seconts_left == 0 {
            let handle =
                rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
            let player = rodio::Player::connect_new(&handle.mixer());
            // Load a sound from a file, using a path relative to Cargo.toml }
            let file = File::open("bell.mp3").unwrap();
            // Decode that sound file into a source
            let source = Decoder::try_from(file).unwrap();
            // Play the sound directly on the device
            handle.mixer().add(source);
            // does not work
            // Command::new("wall").args(["Work time done"]);

            // The sound plays in a separate audio thread,
            // so we need to keep the main thread hile it's playing.
            std::thread::sleep(std::time::Duration::from_secs(3));

            println!("worked for: {display_minust}:{display_seconts}");
        }
        if seconts_left % 60 * 5 == 0 && seconts_left != 0 {
            let handle =
                rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
            let player = rodio::Player::connect_new(&handle.mixer());
            let file = File::open("scream.mp3").unwrap();
            let source = Decoder::try_from(file).unwrap();
            handle.mixer().add(source);
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }
    // rx
    println!("what did you learn");
    let mut learn = String::new();

    learn = learn.trim().to_string();
    match std::io::stdin().read_line(&mut learn) {
        Ok(_) => {
            println!("learned: {learn}");
        }
        Err(e) => {
            println!("error: {e}")
        }
    }

    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("learned.txt")
        .unwrap();

    file.write_all(learn.as_bytes()).unwrap();
    file.flush().unwrap();
}
