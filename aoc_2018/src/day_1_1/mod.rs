use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};



fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let buf = BufReader::new(io);
    buf.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


pub fn calculate_frequency(filename: &str) -> Result<i64, Error> {
    let file = try!(File::open(filename));
    let frequencies = read(file);
    let result: i64 = frequencies.unwrap().iter().fold(0, |sum, x| sum + x);
    println!("End frequency: {}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate_frequency("test_3").unwrap(), 3);
        assert_eq!(calculate_frequency("test_0").unwrap(), 0);
        assert_eq!(calculate_frequency("test_-6").unwrap(), -6);
    }

}