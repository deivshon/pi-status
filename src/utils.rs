use std::fs;

fn u64_from_file(path: String) -> Result<u64, String> {
    match fs::read_to_string(path) {
        Ok(s) => {
            match s.replace("\n", "").parse::<u64>() {
                Ok(v) => return Ok(v),
                Err(e) => return Err(e.to_string())
            }
        },
        Err(e) => return Err(e.to_string())
    }
}
