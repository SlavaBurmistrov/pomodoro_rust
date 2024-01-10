use std::{thread, time, io};
use rodio::{OutputStream, Sink};
use chrono::Local;
use csv::Writer;

fn play_sound(file_path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add your sound file path here (e.g., "path/to/sound.mp3")

    let file = std::fs::File::open(file_path).unwrap();
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

fn log_session_start(session_type: &str, writer: &mut csv::Writer<std::fs::File>) {
    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{} session started at: {}", session_type, current_time);

    writer.write_record(&[session_type, &formatted_time]).unwrap();
    writer.flush().unwrap();
}

fn pomodoro_timer(writer: &mut csv::Writer<std::fs::File>) {
    let work_duration = 25; // 25 minutes for work
    let short_break_duration = 5; // 5 minutes for short break
    let long_break_duration = 15; // 15 minutes for long break
    let cycles_before_long_break = 4; // Number of work cycles before a long break

    let mut total_cycles = 0;

    loop {
        total_cycles += 1;
        play_sound("src/oof.wav ");
        log_session_start("Work", writer);
        println!("Cycle {}: Work for {} minutes", total_cycles, work_duration);
        thread::sleep(time::Duration::from_secs(work_duration * 60)); // Convert minutes to seconds

        if total_cycles % cycles_before_long_break == 0 {
            play_sound("src/ow.wav ");
            log_session_start("Work", writer);
            println!("Take a long break for {} minutes", long_break_duration);
            thread::sleep(time::Duration::from_secs(long_break_duration * 60)); // Convert minutes to seconds
        } else {
            play_sound("src/ow.wav ");
            log_session_start("Work", writer);
            println!("Take a short break for {} minutes", short_break_duration);
            thread::sleep(time::Duration::from_secs(short_break_duration * 60)); // Convert minutes to seconds
        }
    }
}

fn main() {
    let file_path = "src/test.csv ";
    let file = std::fs::File::create(file_path).expect("Error creating file");
    let mut writer = csv::Writer::from_writer(file);

    // Write header to the CSV file
    writer.write_record(&["Session Type", "Start Time"]).unwrap();
    writer.flush().unwrap();

    pomodoro_timer(&mut writer);
}