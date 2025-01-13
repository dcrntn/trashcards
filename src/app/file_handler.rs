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
