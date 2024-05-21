use std::io::{self, BufReader, BufRead};
use std::fs::File;

pub fn get_content(source: String) -> Result<Vec<String>, io::Error> {
    let mut lines: Vec<String> = vec![];
    let mut buf = String::new();
    if source == "-" {
        let mut reader = BufReader::new(io::stdin());
        loop {
            let result = reader.read_line(&mut buf)?;
            if result == 0 {
                break;
            }
            lines.push(buf.strip_suffix("\n").unwrap().to_string());
            buf.clear();
        }
    } else {
        let f = File::open(source)?;
        let mut reader = BufReader::new(f);
        loop {
            let result = reader.read_line(&mut buf)?;
            if result == 0 {
                break;
            }
            lines.push(buf.strip_suffix("\n").unwrap().to_string());
            buf.clear();
    
        }
    }
    Ok(lines)
}