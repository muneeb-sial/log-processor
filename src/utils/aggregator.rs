use std::collections::HashMap;
use std::io::{self, BufRead, Read as _};
use std::{fs::File, io::BufReader};

// const _file_path : &str = "./docs/log-1.txt";

pub fn aggregator(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);
    let mut line = String::with_capacity(256);

    let mut user_map: HashMap<u32, i32> = HashMap::new();
    let mut log_levels = LogLevelStruct {
        debug: 0,
        error: 0,
        warn: 0,
        info: 0,
    };

    let start = std::time::Instant::now();
    let mut count = 0u64;
    loop {
        line.clear();

        let bytes = reader.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        let val = line.trim_end();
        formate_str_to_log(&val, &mut log_levels, &mut user_map).unwrap();
        count += 1; // 13,243,956
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("===========================================================================");
    println!("file: `{}` ", file_path);
    println!("Processed: {} logs", count);
    println!("Time: {:.3}s", elapsed);
    println!("Throughput: {:.0} logs/sec", count as f64 / elapsed);
    println!("Errors {}", log_levels.error);
    println!("debug {}", log_levels.debug);
    println!("info {}", log_levels.info);
    println!("warn {}", log_levels.warn);
    if let Some((user, count)) = user_map.iter().max_by_key(|(_, count)| *count) {
        println!("User: {}", user);
        println!("Logs: {}", count);
    }
    println!("===========================================================================");
    Ok(())
}

struct LogLevelStruct {
    info: i64,
    warn: i64,
    error: i64,
    debug: i64,
}

fn formate_str_to_log<'a>(
    val: &'a str,
    log_levels: &mut LogLevelStruct,
    user_map: &mut HashMap<u32, i32>,
) -> io::Result<()> {
    let mut log_str = val.split(" | ");
    let _timestamp = log_str.next().unwrap();

    let level = log_str.next().unwrap();
    let user = log_str.next().unwrap();

    match level {
        "Debug" => log_levels.debug += 1,
        "Info" => log_levels.info += 1,
        "Warn" => log_levels.warn += 1,
        "Error" => log_levels.error += 1,
        _ => log_levels.error += 1,
    };
    let user_id: u32 = user.strip_prefix("user_id=usr_").unwrap().parse().unwrap();
    *user_map.entry(user_id).or_insert(0) += 1;
    Ok(())
}
