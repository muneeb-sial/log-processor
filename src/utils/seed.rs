use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, Write},
};

use rand::RngExt;
use rayon::prelude::*;
use crate::utils::enums::*;
use crate::utils::stucts::*;

const FILE_COUNT: usize = 10;
const TARGET_SIZE: u64 = 500 * 1024 * 1024; // 500 MiB

pub fn generate_log_files() {
    println!("Generating {FILE_COUNT} log files...");

    create_dir_all("./docs").expect("Failed to create docs directory");

    (1..=FILE_COUNT).into_par_iter().for_each(|file_number| {
        let path = format!("./docs/log-{file_number}.txt");

        println!("Creating {path} (~500 MB)...");

        let file = File::create(&path).expect("Failed to create log file");

        let mut writer = BufWriter::with_capacity(1024 * 1024, file);
        let mut bytes_written = 0u64;

        while bytes_written < TARGET_SIZE {
            let log = generate_log();

            writer
                .write_all(log.as_bytes())
                .expect("Failed to write log");

            bytes_written += log.len() as u64;
        }

        writer.flush().expect("Failed to flush file");

        println!(
            "Created {path}: {:.2} MB",
            bytes_written as f64 / 1024.0 / 1024.0
        );
    });

    println!("All files created.");
}


fn generate_log() -> String {
    let mut rng = rand::rng();

    let level = match rng.random_range(0..4) {
        0 => LogLevel::Info,
        1 => LogLevel::Warn,
        2 => LogLevel::Error,
        _ => LogLevel::Debug,
    };

    let log = Log {
        timestamp: rng.random_range(1000..9999).to_string(),
        level,
        user_id: format!("usr_{}", rng.random_range(1000..9999)),
        message: "Request processed".to_string(),
        duration_ms: rng.random_range(1..1000),
    };

    format!(
        "{} | {:?} | user_id={} | message=\"{}\" | processing_time={}ms\n",
        log.timestamp, log.level, log.user_id, log.message, log.duration_ms
    )
}
