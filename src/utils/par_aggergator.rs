use std::{fs, io};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::utils::aggregator::aggregator;

pub fn execute() -> io::Result<()> {
    let start = std::time::Instant::now();

    let files: Vec<_> = fs::read_dir("./docs")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    files.par_iter().for_each(|path| {
        let p = path.to_str().unwrap();
        println!("[PAR] reading file `{}`", p);
        if let Err(e) = aggregator(p) {
            eprintln!("Failed: {:?}: {}", path, e);
        }
    });

    let elapsed = start.elapsed().as_secs_f64();

    println!("Total time: {:.3}s", elapsed);

    Ok(())
}
