#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial {

    use std::error::Error; // handle errors
    use std::io; // input output
    use std::process; // kill process
    use csv; // parse csv

    // contract state

    #[ink(storage)]
    pub struct ClinicalTrial {
        records: Vec<Record>,
        data_summary: DataSummary,
    }

    // custom types

    type Record = (u32, String, String); // single patient record

    type Records = Vec<Record>; // all patient records

    #[derive(Debug)]
    pub struct DataSummary { // aggregated patient records
        treatment_pos: u32,
        treatment_neg: u32,
        placebo_pos: u32,
        placebo_neg: u32,
    }

    impl ClinicalTrial {

        // contract constructor

        #[ink(constructor)] 
        pub fn new() -> Self {
            // empty patient records and empty data summary
            Self { records: Vec::new(), data_summary: DataSummary {treatment_pos:0, treatment_neg:0, placebo_pos:0, placebo_neg:0}}
        }

        // contract methods

        #[ink(message)]
        // populates records from csv file
        pub fn get(&mut self) {

            let mut records: Records = Vec::new(); // initialize Vector to store
            let mut rdr = csv::Reader::from_reader(io::stdin()); // intialize reader

            for result in rdr.deserialize() { // iterate with serde's deserialize

                match result {
                    Ok(result) => {
                        let record: Record = result;
                        self.records.push(record)
                    }
                    Err(e) => return (),
                };
            }
        }

        #[ink(message)]
        // aggregates records to data summary
        pub fn aggregate(&mut self) {

            for patient in self.records.iter() {

                if patient.1 == "Treatment" {
                    if patient.2 == "Yes" {
                        self.data_summary.treatment_pos += 1;
                    } else {
                        self.data_summary.treatment_neg += 1;
                    }
                } else {
                    if patient.2 == "Yes" {
                        self.data_summary.placebo_pos += 1; 
                    } else {
                        self.data_summary.placebo_neg += 1;
                    }
                } 
            }
        }
    }

    // tests

    #[cfg(test)]
    mod tests {
        
        use super::*; // imports all definitions from the outer scope
        use ink_lang as ink; // imports `ink_lang` so we can use `#[ink::test]`

        #[ink::test]
        // test with data.csv
        fn test_data() {
            let mut clinical_trial = ClinicalTrial::new();
            clinical_trial.aggregate();
            assert_eq!(clinical_trial.data_summary, DataSummary { treatment_pos: 3, treatment_neg: 385, placebo_pos: 28, placebo_neg: 358 });
        }
    }
}
