use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let target = match env::args().nth(1) {
        None => return Err("please specify the target path".into()),
        Some(t) => t,
    };

    let abspath = to_absolute::to_absolute_from_current_dir(&target)?;
    println!("{}", abspath.display());

    Ok(())
}
