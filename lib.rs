#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial_data {

    use ink_prelude::string::String; 
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use statrs::distribution::Hypergeometric;
    use statrs::distribution::DiscreteCDF;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ClinicalTrialData {

        // data
        raw_records: Vec<(u64, String, String)>, // Vec[(1, "Treatment", "Positive"), (2, "Placebo", "Negative"), ...]
        preprocessed_records: Vec<(u64, String, String)>,
        data_summary: Mapping<String, u64>, // {'Treatment Positive': 3, 'Treatment Negative': 345, 'Placebo Positive': 10, 'Placebo Negative': 358}

        // study characteristics
        p_value: u64, // i.e. ink doesn't allow for float, use significant figure multiplier method
        stat_test: String, // i.e. fishers_exact_test        
    }

    impl ClinicalTrialData {

        // creates a new ClinicalTrialData contract initialized to the given values
        #[ink(constructor)] 
        pub fn new(custom_p_value: u64, custom_stat_test: String) -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_value = custom_p_value;
                contract.stat_test = custom_stat_test;
            })
        }

        // creates a new ClinicalTrialData contract initialized to default values
        #[ink(constructor)]
        pub fn default() -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_value = 5;
                contract.stat_test = String::from("fishers_exact_test");
            })
        }

        // gets the p-value of the ClinicalTrialData contract
        #[ink(message)]
        pub fn get_p_value(&self) -> u64 {
            ink_env::debug_println!("p-value for this clinical trial is: {}", self.p_value);
            self.p_value
        }

        // gets the statistical test of the ClinicalTrialData contract
        #[ink(message)]
        pub fn get_stat_test(&self) -> String {
            ink_env::debug_println!("statistical test for this clinical trial is {}", self.stat_test);
            self.stat_test.clone()
        }

        // uploads records to contract storage from frontend (access: authorized data collectors, i.e. doctors, nurses)
        #[ink(message)]
        pub fn upload_raw(&mut self, records: Vec<(u64, String, String)>) {
            for record in records {
                self.raw_records.push(record);
            }
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn download_raw(&self) -> Vec<(u64, String, String)>{
            self.raw_records.clone()
        }

        // uploads preprocessed record to contract storage from frontend and returns stat test results (access: owner)
        #[ink(message)]
        pub fn upload_preprocessed(&mut self, records: Vec<(u64, String, String)>) -> bool {
            for record in records {
                self.preprocessed_records.push(record);
            }
            self.aggregate_data();
            self.run_stat_test()
        }

        // aggregates preprocessed records to data summary (access: owner)
        fn aggregate_data(&mut self) {

            // 1. initiazlie variables
            let mut treatment_pos: u64 = 0;
            let mut treatment_neg: u64 = 0;
            let mut placebo_pos: u64 = 0;
            let mut placebo_neg: u64 = 0;

            // 2. iterate through preprocessed records
            for patient in self.preprocessed_records.iter() {

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

            // 3. insert into Mapping into contract storage
            self.data_summary.insert(String::from("Treatment Positive"), &treatment_pos);
            self.data_summary.insert(String::from("Treatment Negative"), &treatment_neg);
            self.data_summary.insert(String::from("Placebo Positive"), &placebo_pos);
            self.data_summary.insert(String::from("Placebo Negative"), &placebo_neg);
        }

        // runs statistical test on data summary 
        fn run_stat_test(&self) -> bool {

            // 1. read self.data_summary
            let treatment_pos = self.data_summary.get(String::from("Treatment Positive")).unwrap();
            let treatment_neg = self.data_summary.get(String::from("Treatment Negative")).unwrap();
            let placebo_pos = self.data_summary.get(String::from("Treatment Positive")).unwrap();
            let placebo_neg = self.data_summary.get(String::from("Treatment Negative")).unwrap();
            
            // 2. get hypergeomtric parameters
            let population  = treatment_pos + treatment_neg + placebo_pos + placebo_neg;
            let cured = treatment_pos + placebo_neg;
            let treatment = treatment_pos + treatment_neg;
            let observed = treatment_pos;

            // 3. calculate using significant figure multiplier technique since floats are not allowed on ink
            let scalar: u64 = 1000; // significant figure multiplier
            let scaled_p: u64 = self.p_value * (scalar/100); // since self.p_value = 5 is already scaled by 100 from 0.05
            let scaled_left_cdf: u64 = (Hypergeometric::new(population, cured, treatment).unwrap().cdf(observed-1) * scalar as f64) as u64;
            let scaled_right_cdf: u64 = scalar - scaled_left_cdf;
            
            // 4. compare p with p-value
            if scaled_right_cdf < scaled_p {
                return true
            }
            return false
        }
    }

    #[cfg(test)]
    mod tests {
        
        use super::*; // imports all definitions from the outer scope
        use ink_lang as ink; // imports `ink_lang` so we can use `#[ink::test]`

        #[ink::test]
        fn default_init_works() {
            let research = ClinicalTrialData::default();
            assert!(research.get_p_value() == 5 && research.get_stat_test() == String::from("fishers_exact_test"));

        }

        #[ink::test]
        fn new_init_works() {
            let research = ClinicalTrialData::new(2, String::from("t_test"));
            assert!(research.get_p_value() == 2 && research.get_stat_test() == String::from("t_test"));
        }
    }
}
