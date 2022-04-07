#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial_data {

    use ink_prelude::string::String; 
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec; 

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ClinicalTrialData {

        // data
        raw_records: Vec<(String, String, String)>, // Vec[("1", "Treatment", "Positive"), ("2", "Placebo", "Negative"), ...]
        preprocessed_records: Vec<(String, String, String)>,
        data_summary: Mapping<String, u128>, // {'Treatment Positive': 3, 'Treatment Negative': 385, 'Placebo Positive': 28, 'Placebo Negative': 358}

        // study characteristics
        p_thresh: u128, // i.e. ink doesn't allow for float, use significant figure multiplier method
        stat_test: String, // i.e. fishers_exact_test
        result: bool, // true for significant result, i.e. < p-value, false for insignificant, i.e. > p-value
        p_value: Vec<u128> // resulting p   
    }

    impl ClinicalTrialData {

        // creates a new ClinicalTrialData contract initialized to the given values (done on polkadot/subtrate UI)
        #[ink(constructor)] 
        pub fn new(custom_p_thresh: u128, custom_stat_test: String) -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_thresh = custom_p_thresh;
                contract.stat_test = custom_stat_test;
            })
        }

        // creates a new ClinicalTrialData contract initialized to default values (done on polkadot/subtrate UI)
        #[ink(constructor)]
        pub fn default() -> Self {

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.p_thresh = 5;
                contract.stat_test = String::from("fishers_exact_test");
            })
        }

        // sets the p-value of the ClinicalTrialData contract
        #[ink(message)]
        pub fn set_p_thresh(&mut self, p: u128) {
            self.p_thresh = p;
        }

        // gets the p-value of the ClinicalTrialData contract
        #[ink(message)]
        pub fn set_stat_test(&mut self, stat_test: String) {
            self.stat_test = stat_test;
        }

        // gets the p-value of the ClinicalTrialData contract
        #[ink(message)]
        pub fn get_p_thresh(&self) -> u128 {
            self.p_thresh
        }

        // gets the statistical test of the ClinicalTrialData contract
        #[ink(message)]
        pub fn get_stat_test(&self) -> String {
            self.stat_test.clone()
        }

        // uploads records to contract storage from frontend with a whole array
        #[ink(message)]
        pub fn upload_all_raw(&mut self, records: Vec<(String, String, String)>) {
            for record in records {
                self.raw_records.push(record);
            }
        }

        // uploads records to contract storage from frontend one-by-one
        #[ink(message)]
        pub fn upload_one_raw(&mut self, patient_id: String, group: String, outcome: String) {

            let record: (String, String, String) = (patient_id, group, outcome);
            self.raw_records.push(record);
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn download_raw(&self) -> Vec<(String, String, String)>{
            self.raw_records.clone()
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn download_preprocessed(&self) -> Vec<(String, String, String)>{
            self.preprocessed_records.clone()
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn clear_raw(&mut self) {
            self.raw_records.clear();
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn clear_preprocessed(&mut self) {
            self.preprocessed_records.clear();
        }

        // uploads preprocessed record to contract storage from frontend and returns stat test results 
        #[ink(message)]
        pub fn upload_all_preprocessed(&mut self, records: Vec<(String, String, String)>) {
            for record in records {
                self.preprocessed_records.push(record);
            }
            self.aggregate_data();
            self.run_stat_test();
        }

        // uploads preprocessed record to contract storage from frontend and returns stat test results 
        #[ink(message)]
        pub fn upload_one_preprocessed(&mut self, patient_id: String, group: String, outcome: String) {
            let record: (String, String, String) = (patient_id, group, outcome);
            self.preprocessed_records.push(record);
        }

        // runs test after upload_one_proprocessed is done
        #[ink(message)]
        pub fn run_on_preprocessed(&mut self) {
            self.aggregate_data();
            self.run_stat_test();
        }

        // gets result; true = significant, otherwise false
        #[ink(message)]
        pub fn get_result(&self) -> bool {
            self.result
        }

        // gets p-value result
        #[ink(message)]
        pub fn get_p_value(&self) -> Vec<u128> {
            self.p_value.clone()
        }

        // aggregates preprocessed records to data summary (access: owner)
        pub fn aggregate_data(&mut self) {

            // 1. initiazlie variables
            let mut treatment_pos: u128 = 0;
            let mut treatment_neg: u128 = 0;
            let mut placebo_pos: u128 = 0;
            let mut placebo_neg: u128 = 0;

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

        // calculates factorial iteratively
        pub fn factorial(&self, num: u128) -> u128 {
            (1..=num).fold(1, |acc, v| acc * v)
        }
        
        // calculates fisher's exact test formulaically
        pub fn binomial(&self, val1: u128, val2: u128) -> u128{
            self.factorial(val1) / (self.factorial(val2) * self.factorial(&val1 - &val2))
        }

        // calculates p-value using hypergeometric distribution in fisher's exact test
        pub fn hypergeom_cdf(&self, population: u128, cured: u128, treatment: u128, mut observed: u128) -> u128 {
            let mut hypergeom_sum: u128 = 0;
            while observed <= treatment{
                hypergeom_sum += self.binomial(treatment, observed) * self.binomial(population-treatment, cured - observed);
                observed += 1;
            }
            hypergeom_sum
        }

        // runs statistical test on data summary 
        pub fn run_stat_test(&mut self) {

            // 1. read self.data_summary
            let treatment_pos = self.data_summary.get(String::from("Treatment Positive")).unwrap();
            let treatment_neg = self.data_summary.get(String::from("Treatment Negative")).unwrap();
            let placebo_pos = self.data_summary.get(String::from("Treatment Positive")).unwrap();
            let placebo_neg = self.data_summary.get(String::from("Treatment Negative")).unwrap();
            
            // 2. get hypergeomtric parameters
            let population  = treatment_pos + treatment_neg + placebo_pos + placebo_neg;
            let cured = treatment_pos + placebo_neg;
            let treatment = treatment_pos + treatment_neg;
            let observed = treatment_neg; // neg instead of pos because we consider the left tail in our sample

            // significant figure multiplier
            let scalar: u128 = 100; // significant figure multiplier
            let scaled_p: u128 = self.p_thresh * self.binomial(population, treatment); // since self.p_thresh = 5 is already scaled by 100 from 0.05
            let scaled_right_cdf: u128 = self.hypergeom_cdf(population, cured, treatment, observed)*scalar;

            // 4. compare p-value with p-thresh
            self.p_value = vec![self.hypergeom_cdf(population, cured, treatment, observed), self.binomial(population, treatment)];
            if scaled_right_cdf < scaled_p {
                self.result = true;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        
        use super::*; // imports all definitions from the outer scope
        use ink_lang as ink; // imports `ink_lang` so we can use `#[ink::test]`
        use ink_prelude::string::ToString;

        #[ink::test]
        fn init_works() {
            let research = ClinicalTrialData::default();
            assert!(research.get_p_thresh() == 5 && research.get_stat_test() == String::from("fishers_exact_test"));

            let research = ClinicalTrialData::new(2, String::from("t_test"));
            assert!(research.get_p_thresh() == 2 && research.get_stat_test() == String::from("t_test"));
        }

        #[ink::test]
        fn upload_all_works() {
            
            let sample: Vec<(String, String, String)> = vec![
                ("1", "Treatment", "Yes"), ("2", "Treatment", "Yes"), ("3", "Treatment", "Yes"), 
                ("4", "Treatment", "No"), ("5", "Treatment", "No"), ("6", "Treatment", "No"), 
                ("7", "Treatment", "No"), ("8", "Treatment", "No"), ("9", "Treatment", "No"), 
                ("10", "Treatment", "No"),("111", "Treatment", "No"), ("112", "Treatment", "No"), 
                ("113", "Treatment", "No"), ("114", "Treatment", "No"), ("115", "Treatment", "No"),
                ("431", "Placebo", "No"), ("432", "Placebo", "No"), ("433", "Placebo", "No"), 
                ("434", "Placebo", "No"), ("435", "Placebo", "No"), ("436", "Placebo", "No"), 
                ("437", "Placebo", "No"), ("438", "Placebo", "No"), ("439", "Placebo", "No"), 
                ("440", "Placebo", "No")]
                    .iter()
                    .map(|x| (x.0.to_string(), x.1.to_string(), x.2.to_string()))
                    .collect::<Vec<(String, String, String)>>();
            
            // initialize default contract with p = 0.05 and fisher's exact test
            let mut research = ClinicalTrialData::default();

            // test raw records upload
            research.upload_all_raw(sample.clone());
            assert!(research.raw_records == sample);

            // test raw records download
            let download = research.download_raw();
            assert!(download == research.raw_records);

            // clear raw records
            research.clear_raw();
            assert!(research.raw_records.len() == 0);

            // test preprocessed records upload 
            research.upload_all_preprocessed(sample.clone());
            assert!(research.preprocessed_records == sample);

            // test data aggregation
            assert!(research.data_summary.get(&String::from("Treatment Positive")).unwrap() == 3);
            assert!(research.data_summary.get(&String::from("Treatment Negative")).unwrap() == 12);
            assert!(research.data_summary.get(&String::from("Placebo Positive")).unwrap() == 0);
            assert!(research.data_summary.get(&String::from("Placebo Negative")).unwrap() == 10);
            
            // test statistical test
            ink_env::debug_println!("p-value: {:?}", research.p_value);
            assert!(research.result == true);
        }

        #[ink::test]
        fn upload_one_by_one_works() {
            
            let sample: Vec<(String, String, String)> = vec![
                ("1", "Treatment", "Yes"), ("2", "Treatment", "Yes"), ("3", "Treatment", "Yes"), 
                ("4", "Treatment", "No"), ("5", "Treatment", "No"), ("6", "Treatment", "No"), 
                ("7", "Treatment", "No"), ("8", "Treatment", "No"), ("9", "Treatment", "No"), 
                ("10", "Treatment", "No"),("111", "Treatment", "No"), ("112", "Treatment", "No"), 
                ("113", "Treatment", "No"), ("114", "Treatment", "No"), ("115", "Treatment", "No"),
                ("431", "Placebo", "No"), ("432", "Placebo", "No"), ("433", "Placebo", "No"), 
                ("434", "Placebo", "No"), ("435", "Placebo", "No"), ("436", "Placebo", "No"), 
                ("437", "Placebo", "No"), ("438", "Placebo", "No"), ("439", "Placebo", "No"), 
                ("440", "Placebo", "No")]
                    .iter()
                    .map(|x| (x.0.to_string(), x.1.to_string(), x.2.to_string()))
                    .collect::<Vec<(String, String, String)>>();
            
            // initialize default contract with p = 0.05 and fisher's exact test
            let mut research = ClinicalTrialData::default();

            // set p-value and stat test
            research.set_p_thresh(6);
            research.set_stat_test(String::from("difference_of_means_test"));
            assert!(research.p_thresh == 6);
            assert!(research.stat_test == "difference_of_means_test");
            
            // revert back to default since contract doesn't have difference of means yet
            research.set_p_thresh(5);
            research.set_stat_test(String::from("fishers_exact_test"));

            // test raw records upload one by one
            for patient in sample.clone().iter() {
                research.upload_one_raw(patient.0.clone(), patient.1.clone(), patient.2.clone())
            }
            assert!(research.raw_records == sample);

            // test raw records download
            let download = research.download_raw();
            assert!(download == research.raw_records);

            // test preprocessed records upload 
            for patient in sample.clone().iter() {
                research.upload_one_preprocessed(patient.0.clone(), patient.1.clone(), patient.2.clone())
            };
            research.run_on_preprocessed();
            assert!(research.preprocessed_records == sample);

            // test data aggregation
            assert!(research.data_summary.get(&String::from("Treatment Positive")).unwrap() == 3);
            assert!(research.data_summary.get(&String::from("Treatment Negative")).unwrap() == 12);
            assert!(research.data_summary.get(&String::from("Placebo Positive")).unwrap() == 0);
            assert!(research.data_summary.get(&String::from("Placebo Negative")).unwrap() == 10);
            
            // test statistical test
            ink_env::debug_println!("p-value: {:?}", research.p_value);
            assert!(research.result == true);
        }
    }
}