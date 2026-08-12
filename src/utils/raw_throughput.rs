use std::{fs::File, io::{self, BufRead, BufReader}};

pub fn execute() -> io::Result<()> {
    let file = File::open("./docs/log-1.txt")?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);

    let start = std::time::Instant::now();

    let mut count = 0u64;
    let mut line = String::with_capacity(256);

    loop {
        line.clear();

        let bytes = reader.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        count += 1;
    }

    let elapsed = start.elapsed().as_secs_f64();

    println!("Processed: {} logs", count);
    println!("Time: {:.3}s", elapsed);
    println!("Throughput: {:.0} logs/sec", count as f64 / elapsed);

    Ok(())
}
