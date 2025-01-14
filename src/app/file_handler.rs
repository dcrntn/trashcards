use csv::{ReaderBuilder, StringRecord};
use rand::seq::SliceRandom; // For random sampling
use std::error::Error;
use std::fs::File;

pub fn read_and_process_csv_with_headers(
    file_path: &str,
    n: usize,
) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(file_path)?;

    // Create a CSV reader with semicolon delimiter
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Specify that the CSV has headers
        .delimiter(b';')   // Set delimiter to semicolon
        .from_reader(file);

    // Read headers and convert to Vec<String>
    let headers = rdr
        .headers()? // Retrieve headers
        .iter()
        .map(|header| header.to_string())
        .collect::<Vec<String>>();


    let mut records = Vec::new(); // Vec to hold the rows

    // Iterate through each row in the CSV and store it
    for result in rdr.records() {
        let record: StringRecord = result?; // Get the record for the current row
        let row = record.iter().map(|field| field.to_string()).collect::<Vec<String>>(); // Convert to Vec<String>
        records.push(row); // Add row to the records vector
    }

    // Shuffle the rows and take the first N random rows
    let mut rng = rand::thread_rng();
    let random_rows = records
        .choose_multiple(&mut rng, n)
        .cloned()
        .collect::<Vec<Vec<String>>>();

    Ok((headers, random_rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_and_process_csv_with_headers_success() {
        // Create a temporary CSV file with headers and data
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "Question;Answer\nWhat is Rust?;A systems programming language.\nWhat is Cargo?;Rust's package manager."
        )
        .unwrap();

        // Call the function
        let result = read_and_process_csv_with_headers(temp_file.path().to_str().unwrap(), 1);

        // Verify the result
        assert!(result.is_ok());
        let (headers, rows) = result.unwrap();

        // Check headers
        assert_eq!(headers, vec!["Question", "Answer"]);

        // Check rows
        assert_eq!(rows.len(), 1); // Only one random row should be returned
        assert_eq!(rows[0].len(), 2); // Each row should have two columns
    }

    #[test]
    fn test_read_and_process_csv_with_headers_file_not_found() {
        // Call the function with an invalid file path
        let result = read_and_process_csv_with_headers("invalid_path.csv", 1);

        // Verify that the function returns an error
        assert!(result.is_err());
    }

    #[test]
    fn test_read_and_process_csv_with_headers_empty_csv() {
        // Create an empty temporary CSV file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Question;Answer").unwrap(); // Only headers, no data

        // Call the function
        let result = read_and_process_csv_with_headers(temp_file.path().to_str().unwrap(), 1);

        // Verify the result
        assert!(result.is_ok());
        let (headers, rows) = result.unwrap();

        // Check headers
        assert_eq!(headers, vec!["Question", "Answer"]);

        // Check rows
        assert_eq!(rows.len(), 0); // No rows should be returned
    }

    #[test]
    fn test_read_and_process_csv_with_headers_n_larger_than_rows() {
        // Create a temporary CSV file with a small number of rows
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "Question;Answer\nWhat is Rust?;A systems programming language."
        )
        .unwrap();

        // Call the function with n larger than the number of rows
        let result = read_and_process_csv_with_headers(temp_file.path().to_str().unwrap(), 5);

        // Verify the result
        assert!(result.is_ok());
        let (headers, rows) = result.unwrap();

        // Check headers
        assert_eq!(headers, vec!["Question", "Answer"]);

        // Check rows
        assert_eq!(rows.len(), 1); // Only one row exists in the CSV
    }

    #[test]
    fn test_read_and_process_csv_with_headers_semicolon_delimiter() {
        // Create a temporary CSV file with semicolon as the delimiter
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "Question;Answer\nWhat is Rust?;A systems programming language.\nWhat is Cargo?;Rust's package manager."
        )
        .unwrap();

        // Call the function
        let result = read_and_process_csv_with_headers(temp_file.path().to_str().unwrap(), 2);

        // Verify the result
        assert!(result.is_ok());
        let (headers, rows) = result.unwrap();

        // Check headers
        assert_eq!(headers, vec!["Question", "Answer"]);

        // Check rows
        assert_eq!(rows.len(), 2); // Two rows should be returned
    }
}
