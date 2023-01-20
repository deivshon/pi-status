use std::fs;
use std::error::Error;

fn u64_from_file(path: String) -> Result<u64, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let num = file_content.replace("\n", "").parse::<u64>()?;

    return Ok(num);
}
