use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn append(file_path: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::options().append(true).create(true).open(file_path)?;

    writeln!(&mut file, "{contents}\n")?;
    Ok(())
}
