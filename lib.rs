#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial_data {

    // use std::error::Error;
    use std::io;
    use csv;
    use ink_storage::Mapping;
    use ink_prelude::string::String; 
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ClinicalTrialData {

        // data
        raw_records: Vec<(u32, String, String)>, // Vec[(1, "Treatment", "Positive"), (2, "Placebo", "Negative")]
        preprocessed_records: Vec<(u32, String, String)>, // Vec[(1, "Treatment", "Positive"), (2, "Placebo", "Negative")]
        data_summary: Mapping<String, u32>, // {'Treatment Positive': 3, 'Placebo Negative': 358}

        // study characteristics
        p_value: u32, // i.e. ink doesn't allow for float
        stat_test: String, // i.e. fishers_exact_test        
    }

    impl ClinicalTrialData {

        // creates a new clinical_trial_data smart contract initialized to the given values
        #[ink(constructor)] 
        pub fn new(custom_p_value: u32, custom_stat_test: String) -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_value = custom_p_value;
                contract.stat_test = custom_stat_test;
            })
        }

        // creates a new clinical_trial_data smart contract initialized to default values
        #[ink(constructor)]
        pub fn default() -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_value = 5;
                contract.stat_test = String::from("fishers_exact_test");
            })
        }

        #[ink(message)]
        pub fn get_p_value(&self) -> u32 {
            ink_env::debug_println!("p-value for this clinical trial is: {}", self.p_value);
            self.p_value
        }

        #[ink(message)]
        pub fn get_stat_test(&self) -> String {
            ink_env::debug_println!("statistical test for this clinical trial is {}", self.stat_test);
            self.stat_test.clone()
        }

        // uploads records from csv file (access: authorized data collectors, i.e. doctors, nurses)
        #[ink(message)]
        pub fn upload_raw(&mut self) {

            let mut rdr = csv::Reader::from_reader(io::stdin()); // intialize csv reader
            for result in rdr.deserialize() { // iterate with serde's deserialize

                match result {
                    Ok(result) => {
                        let raw_record: (u32, String, String) = result;
                        self.raw_records.push(raw_record)
                    }
                    Err(e) => println!("Failed to upload CSV", e),
                };
            }
        }

        // downloads raw csv records (access: owner)
        #[ink(message)]
        pub fn download_raw(&mut self) {
            // 1. pull all entries in self.raw_records
            // 2. convert into csv
            // 3. return csv
        }

        // uploads preprocessed csv records (access: owner)
        #[ink(message)]
        pub fn upload_preprocessed(&mut self) {
            // 1. reads csv
            // 2. iterate 
            // 3. push all entries into self.preprocessed_records
            // 4. aggregate data to self.data_summary, i.e. call aggregate_data()
            // 5. run statistical test, i.e. call run_stat_test()
            // 6. return statistical test results.
        }

        // runs statistical test on data summary 
        fn run_stat_test(&mut self) {
            // 1. reads self.data_summary
            // 2. calculates results
            // 3. return results
        }

        // aggregates preprocessed records to data summary (access: owner)
        fn aggregate_data(&mut self) {
            // 1. read self.preprocessed_records
            // 2. iterate
            // 3. count into self.data_summary

            let treatment_pos = 0;
            let treatment_neg = 0;
            let placebo_pos = 0;
            let placebo_neg = 0;

            for patient in self.preprocessed_   records.iter() {

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
        fn default_init() {
            let research = ClinicalTrialData::default();
            assert!(research.get_p_value() == 5 && research.get_stat_test() == String::from("fishers_exact_test"));

        }

        #[ink::test]
        fn new_init() {
            let research = ClinicalTrialData::new(2, String::from("t_test"));
            assert!(research.get_p_value() == 2 && research.get_stat_test() == String::from("t_test"));
        }
    }
}
