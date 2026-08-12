use std::io;

mod utils;

fn main() -> io::Result<()> {
    utils::par_aggergator::execute()?;
    Ok(())
}
