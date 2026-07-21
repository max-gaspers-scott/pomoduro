//! this is a pomodoro timer for intense work
//! no relaxed lofi
//!
//! to install:
//! cargo install intense-pomodor
//!
//! to run:
//! intense-pomodor

use rodio::{Decoder, MixerDeviceSink, source::Source};

use std::char::CharTryFromError;
use std::ffi::os_str::Display;
use std::fs::OpenOptions;
use std::io::Write;
use time::OffsetDateTime; // Make sure to bring in format_description

use std::fmt::format;
use std::fs::{File, Metadata};
use std::process::Command;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("\x1B[41mThis has a red background!\x1B[0m");

    println!("\\e[41m\\e[97m Red Background \\e[0m");
    spaceing();
    println!("use this commad to record your work  ");
    println!("asciinema record 7-9_6pm.cast ");
    let task = get_input("what are you going to work on");

    write("task", &task);

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
    let start_str = format!("{:?}", start);
    let now_local: OffsetDateTime = OffsetDateTime::now_utc();
    write("start-time", &now_local.to_string());
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
        let display_minuts = seconts_left / 60;
        let display_seconts = seconts_left % 60;
        // print!("minuts: ");
        // print_color(&to_binary_str(display_minuts));
        // print!(", seconts: ");
        // print_color(&to_binary_str(display_seconts));
        // println!("");
        //
        println!(
            "minuts: {}, seconts: {}",
            to_binary_str(display_minuts),
            to_binary_str(display_seconts)
        );
        // println!("                              ___________");
        println!("                                6 318 421");
        println!("                                4 26     ");
        println!();
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(_) => {
                println!("\nStopped early by user!");
                println!("worked for: {display_minuts}:{display_seconts}");
                break;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Just timed out, continue standard loop execution
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                // The input thread disconnected/panicked, break
                println!("worked for: {display_minuts}:{display_seconts}");
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

            println!("worked for: {display_minuts}:{display_seconts}");
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
    let learn = get_input("what did you learn");

    write("learned", &learn);
}

fn get_input(msg: &str) -> String {
    println!("{msg}");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(e) => {
            println!("error getting input: {e}");
            get_input(msg)
        }
    }
}

fn write(meta_str: &str, text: &str) {
    let text = text.trim();
    let text = format!("{}: {}\n", meta_str, text);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("learned.txt")
        .unwrap();

    file.write_all(text.as_bytes()).unwrap();
    file.flush().unwrap();
}

fn spaceing() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("learned.txt")
        .unwrap();

    file.write_all("\n\n".as_bytes()).unwrap();
    file.flush().unwrap();
}

fn to_binary_str(mut num: i32) -> String {
    let mut displayibl = format!("{:0>9b}", num);
    displayibl.insert_str(3, "-");
    displayibl.insert_str(7, "-");
    displayibl
}

fn print_color(input: &str) {
    let mut is_odd = false;
    for ch in input.chars() {
        if ch.eq(&'-') {
            print!("-");
            continue;
        }
        if is_odd {
            print!("{ch}");
        } else {
            print!("\x1B[21m{ch}\x1B[0m")
        }
        is_odd = !is_odd;
    }
}
