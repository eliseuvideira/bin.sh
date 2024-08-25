use std::error::Error;
use std::io::{self, BufRead};
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let rows = transpose_csv(reader)?;

    for row in rows {
        println!("{}", row);
    }

    Ok(())
}

fn transpose_csv<R: BufRead>(reader: R) -> Result<Vec<String>, Box<dyn Error>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(reader);
    let mut rows = Vec::new();

    for result in csv_reader.records() {
        let record = result?;
        for field in record.iter() {
            rows.push(field.to_string());
        }
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_transpose_single_line() {
        let input = "1,2,3,4\n";
        let cursor = Cursor::new(input);

        let rows = transpose_csv(cursor).unwrap();
        assert_eq!(rows, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn test_transpose_multiple_lines() {
        let input = "1,2,3,4\n5,6,7,8\n";
        let cursor = Cursor::new(input);

        let rows = transpose_csv(cursor).unwrap();
        assert_eq!(rows, vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }

    #[test]
    fn test_transpose_empty_input() {
        let input = "";
        let cursor = Cursor::new(input);

        let rows = transpose_csv(cursor).unwrap();
        assert_eq!(rows, Vec::<String>::new());
    }

    #[test]
    fn test_transpose_complex_input() {
        let input = "00000000-0000-0000-0000-000000000000,,\"{apple,banana,orange}\",\"{\"\"id\"\": 1, \"\"is_active\"\": false, \"\"created_at\"\": \"\"2024-01-01 12:00:00.000000+00\"\"}\",f,*****00000000";
        let cursor = Cursor::new(input);

        let rows = transpose_csv(cursor).unwrap();
        assert_eq!(
            rows,
            vec![
                "00000000-0000-0000-0000-000000000000",
                "",
                "{apple,banana,orange}",
                "{\"id\": 1, \"is_active\": false, \"created_at\": \"2024-01-01 12:00:00.000000+00\"}",
                "f",
                "*****00000000"
            ]
        );
    }
}
