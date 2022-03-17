use std::error::Error;
use std::io;
use std::process;
use csv;

type Record = (u32, String, String); // define type "Record" for a single patient record
type Records = Vec<Record>; // define type "Records" for all the patient records

#[derive(Debug)]
struct DataSummary { // define type "DataSummary" for aggregated patient records
    treatment_pos: u32,
    treatment_neg: u32,
    placebo_pos: u32,
    placebo_neg: u32,
}

fn main() {

    if let Ok(records) = get() {

        let records = records;

        let mut summary = DataSummary {treatment_pos:0, treatment_neg:0, placebo_pos:0, placebo_neg:0};
        for patient in records.iter() {
            if patient.1 == "Treatment" {
                if patient.2 == "Yes" {
                    summary.treatment_pos += 1;
                } else {
                    summary.treatment_neg += 1;
                }
            } else {
                if patient.2 == "Yes" {
                    summary.placebo_pos += 1; 
                } else {
                    summary.placebo_neg += 1;
                }
            } 
        }
        println!("RESULT: {:?}", summary)

    } else {
        println!("ERROR: failed to get records");
        process::exit(1);
    } 
}

fn get() -> Result<Records, Box<dyn Error>> { // returns type "Result" with two enums?

    let mut records: Records = Vec::new(); // initialize Vector to store
    let mut rdr = csv::Reader::from_reader(io::stdin()); // intialize reader

    for result in rdr.deserialize() { // iterator with serde's deserialize instead of records
        let record: Record = result?; // deserialize into type "Patient", ?
        records.push(record);
    }
    Ok(records)
}