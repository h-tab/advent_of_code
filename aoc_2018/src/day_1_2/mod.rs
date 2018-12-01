use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashSet;


fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let buf = BufReader::new(io);
    buf.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


pub fn find_duplicate_frequency(filename: &str) -> Result<i64, Error> {
    let file = try!(File::open(filename));
    let frequencies = read(file).unwrap();
    let mut seen_frequencies: HashSet<i64> = HashSet::new();
    let mut current_frequency: i64 = 0;
    let mut current_index: usize = 0;
    while !seen_frequencies.contains(&current_frequency) {
        seen_frequencies.insert(current_frequency);
        current_frequency += frequencies[current_index];
        if current_index < frequencies.len() - 1 {
            current_index += 1;
        } else {
            current_index = 0;
        }
    }
    println!("Frequency seen twice: {}", current_frequency);
    Ok(current_frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_frequency() {
        assert_eq!(find_duplicate_frequency("test_0").unwrap(), 0);
        assert_eq!(find_duplicate_frequency("test_5").unwrap(), 5);
        assert_eq!(find_duplicate_frequency("test_10").unwrap(), 10);
        assert_eq!(find_duplicate_frequency("test_14").unwrap(), 14);
    }

}