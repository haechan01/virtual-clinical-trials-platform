#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial_data {

    // use std::error::Error; // handle errors
    use std::io; // input output
    use csv; // parse csv
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ClinicalTrialData {

        // data
        records: Vec<(u32, String, String)>, // Vec[(1, "Treatment", "Positive"), (2, "Placebo", "Negative")]
        data_summary: ink_storage::Mapping<String, u32>, // {'Treatment Positive': 3, 'Placebo Negative': 358}

        // study characteristics
        p_value: u32,
        statistical_test: String,
    }

    impl ClinicalTrialData {

        // creates a new clinical_trial_data smart contract initialized to the given values
        #[ink(constructor)] 
        pub fn new(custom_p_value: u32, custom_statistical_test: String) -> Self {
            Self { records: Vec::new(),
                   data_summary: Mapping::new(),
                   p_value: custom_p_value,
                   statistical_test: custom_statistical_test}
        }

        // creates a new clinical_trial_data smart contract initialized to default values
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { records: Vec::new(),
                   data_summary: Mapping::new(),
                   p_value: 5,
                   statistical_test: String::from("t-test")}
        }

        // uploads records from csv file (access: authorized data collectors, i.e. doctors, nurses)
        #[ink(message)]
        pub fn upload_raw(&mut self) {

            let mut rdr = csv::Reader::from_reader(io::stdin()); // intialize csv reader
            for result in rdr.deserialize() { // iterate with serde's deserialize

                match result {
                    Ok(result) => {
                        let record: (u32, String, String) = result;
                        self.records.push(record)
                    }
                    Err(e) => return (),
                };
            }
        }

        // downloads raw csv records (access: owner)
        #[ink(message)]
        pub fn download_raw(&mut self) {}

        // uploads preprocessed csv records (access: owner)
        #[ink(message)]
        pub fn upload_preprocessed(&mut self) {}

        // runs statistical test on data summary 
        #[ink(message)]
        pub fn run_stat_test(&mut self) {}

        // aggregates records to data summary (access: owner)
        #[ink(message)]
        pub fn aggregate_data(&mut self) {

            let treatment_pos = 0;
            let treatment_neg = 0;
            let placebo_pos = 0;
            let placebo_neg = 0;

            for patient in self.records.iter() {

                if patient.1 == "Treatment" {
                    if patient.2 == "Yes" {
                        treatment_pos += 1;
                    } else {
                        treatment_neg += 1;
                    }
                } else {
                    if patient.2 == "Yes" {
                        placebo_pos += 1; 
                    } else {
                        placebo_neg += 1;
                    }
                } 
            }

            self.data_summary.insert(String::from("Treatment Positive"), &treatment_pos);
            self.data_summary.insert(String::from("Treatment Negative"), &treatment_neg);
            self.data_summary.insert(String::from("Placebo Positive"), &placebo_pos);
            self.data_summary.insert(String::from("Placebo Negative"), &placebo_neg);
        }
    }

    #[cfg(test)]
    mod tests {
        
        use super::*; // imports all definitions from the outer scope
        use ink_lang as ink; // imports `ink_lang` so we can use `#[ink::test]`

        #[ink::test]
        fn test() {
            // pass
        }
    }
}
