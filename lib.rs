#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod clinical_trial_data {

    use ink_prelude::string::String; 
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct ClinicalTrialData {

        // data
        raw_records: Vec<(u128, String, String)>, // Vec[(1, "Treatment", "Positive"), (2, "Placebo", "Negative"), ...]
        preprocessed_records: Vec<(u128, String, String)>,
        data_summary: Mapping<String, u128>, // {'Treatment Positive': 3, 'Treatment Negative': 385, 'Placebo Positive': 28, 'Placebo Negative': 358}

        // study characteristics
        p_value: u128, // i.e. ink doesn't allow for float, use significant figure multiplier method
        stat_test: String, // i.e. fishers_exact_test
        result: bool // true for significant result, i.e. < p-value, false for insignificant, i.e. > p-value        
    }

    impl ClinicalTrialData {

        // creates a new ClinicalTrialData contract initialized to the given values
        #[ink(constructor)] 
        pub fn new(custom_p_value: u128, custom_stat_test: String) -> Self {

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
        pub fn get_p_value(&self) -> u128 {
            self.p_value
        }

        // gets the statistical test of the ClinicalTrialData contract
        #[ink(message)]
        pub fn get_stat_test(&self) -> String {
            self.stat_test.clone()
        }

        // uploads records to contract storage from frontend (access: authorized data collectors, i.e. doctors, nurses)
        #[ink(message)]
        pub fn upload_raw(&mut self, records: Vec<(u128, String, String)>) {
            for record in records {
                self.raw_records.push(record);
            }
        }

        // returns records from contract storage to frontend (access: contract owner, i.e. researcher)
        #[ink(message)]
        pub fn download_raw(&self) -> Vec<(u128, String, String)>{
            self.raw_records.clone()
        }

        // uploads preprocessed record to contract storage from frontend and returns stat test results (access: owner)
        #[ink(message)]
        pub fn upload_preprocessed(&mut self, records: Vec<(u128, String, String)>) {
            for record in records {
                self.preprocessed_records.push(record);
            }
            self.aggregate_data();
            self.run_stat_test();
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

        pub fn factorial(&self, num: u128) -> u128 {
            (1..=num).fold(1, |acc, v| acc * v)
        }
        
        pub fn binomial(&self, val1: u128, val2: u128) -> u128{
            self.factorial(val1)/(self.factorial(val2)*self.factorial(&val1-&val2))
        }

        pub fn hypergeom_cdf(&self, population: u128, cured: u128, treatment: u128, mut observed: u128) -> u128 {
            let mut hypergeom_sum: u128 = 0;
            while observed <= treatment && observed <= cured{
                hypergeom_sum += self.binomial(cured, observed) * self.binomial(population-cured, treatment-&observed);
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
            let scaled_p: u128 = self.p_value*self.binomial(population, treatment); // since self.p_value = 5 is already scaled by 100 from 0.05
            let scaled_right_cdf: u128 = self.hypergeom_cdf(population, cured, treatment, observed)*scalar;
            // 4. compare p with p-value
            if scaled_right_cdf < scaled_p {
                self.result = true;
            }
        }

        #[ink(message)]
        pub fn get_result(&self) {
            self.result
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
            assert!(research.get_p_value() == 5 && research.get_stat_test() == String::from("fishers_exact_test"));

            let research = ClinicalTrialData::new(2, String::from("t_test"));
            assert!(research.get_p_value() == 2 && research.get_stat_test() == String::from("t_test"));
        }

        #[ink::test]
        fn upload_download_calculate_works() {
            let sample: Vec<(u128, String, String )> = vec![(1, "Treatment", "Yes"), (2, "Treatment", "Yes"), (3, "Treatment", "Yes"), (4, "Treatment", "No"), (5, "Treatment", "No"), (6, "Treatment", "No"), (7, "Treatment", "No"), (8, "Treatment", "No"), (9, "Treatment", "No"), (10, "Treatment", "No"),(111, "Treatment", "No"), (112, "Treatment", "No"), (113, "Treatment", "No"), (114, "Treatment", "No"), (115, "Treatment", "No"),(431, "Placebo", "No"), (432, "Placebo", "No"), (433, "Placebo", "No"), (434, "Placebo", "No"), (435, "Placebo", "No"), (436, "Placebo", "No"), (437, "Placebo", "No"), (438, "Placebo", "No"), (439, "Placebo", "No"), (440, "Placebo", "No")]
            .iter()
            .map(|x| (x.0, x.1.to_string(), x.2.to_string()))
            .collect::<Vec<(u128, String, String)>>();


            // // generate sample data referenced from sample.csv
            // let sample: Vec<(u128, String, String )> = vec![(1, "Treatment", "Yes"), (2, "Treatment", "Yes"), (3, "Treatment", "Yes"), (4, "Treatment", "No"), (5, "Treatment", "No"), (6, "Treatment", "No"), (7, "Treatment", "No"), (8, "Treatment", "No"), (9, "Treatment", "No"), (10, "Treatment", "No"), (11, "Treatment", "No"), (12, "Treatment", "No"), (13, "Treatment", "No"), (14, "Treatment", "No"), (15, "Treatment", "No"), (16, "Treatment", "No"), (17, "Treatment", "No"), (18, "Treatment", "No"), (19, "Treatment", "No"), (20, "Treatment", "No"), (21, "Treatment", "No"), (22, "Treatment", "No"), (23, "Treatment", "No"), (24, "Treatment", "No"), (25, "Treatment", "No"), (26, "Treatment", "No"), (27, "Treatment", "No"), (28, "Treatment", "No"), (29, "Treatment", "No"), (30, "Treatment", "No"), (31, "Treatment", "No"), (32, "Treatment", "No"), (33, "Treatment", "No"), (34, "Treatment", "No"), (35, "Treatment", "No"), (36, "Treatment", "No"), (37, "Treatment", "No"), (38, "Treatment", "No"), (39, "Treatment", "No"), (40, "Treatment", "No"), (41, "Treatment", "No"), (42, "Treatment", "No"), (43, "Treatment", "No"), (44, "Treatment", "No"), (45, "Treatment", "No"), (46, "Treatment", "No"), (47, "Treatment", "No"), (48, "Treatment", "No"), (49, "Treatment", "No"), (50, "Treatment", "No"), (51, "Treatment", "No"), (52, "Treatment", "No"), (53, "Treatment", "No"), (54, "Treatment", "No"), (55, "Treatment", "No"), (56, "Treatment", "No"), (57, "Treatment", "No"), (58, "Treatment", "No"), (59, "Treatment", "No"), (60, "Treatment", "No"), (61, "Treatment", "No"), (62, "Treatment", "No"), (63, "Treatment", "No"), (64, "Treatment", "No"), (65, "Treatment", "No"), (66, "Treatment", "No"), (67, "Treatment", "No"), (68, "Treatment", "No"), (69, "Treatment", "No"), (70, "Treatment", "No"), (71, "Treatment", "No"), (72, "Treatment", "No"), (73, "Treatment", "No"), (74, "Treatment", "No"), (75, "Treatment", "No"), (76, "Treatment", "No"), (77, "Treatment", "No"), (78, "Treatment", "No"), (79, "Treatment", "No"), (80, "Treatment", "No"), (81, "Treatment", "No"), (82, "Treatment", "No"), (83, "Treatment", "No"), (84, "Treatment", "No"), (85, "Treatment", "No"), (86, "Treatment", "No"), (87, "Treatment", "No"), (88, "Treatment", "No"), (89, "Treatment", "No"), (90, "Treatment", "No"), (91, "Treatment", "No"), (92, "Treatment", "No"), (93, "Treatment", "No"), (94, "Treatment", "No"), (95, "Treatment", "No"), (96, "Treatment", "No"), (97, "Treatment", "No"), (98, "Treatment", "No"), (99, "Treatment", "No"), (100, "Treatment", "No"), (101, "Treatment", "No"), (102, "Treatment", "No"), (103, "Treatment", "No"), (104, "Treatment", "No"), (105, "Treatment", "No"), (106, "Treatment", "No"), (107, "Treatment", "No"), (108, "Treatment", "No"), (109, "Treatment", "No"), (110, "Treatment", "No"), (111, "Treatment", "No"), (112, "Treatment", "No"), (113, "Treatment", "No"), (114, "Treatment", "No"), (115, "Treatment", "No"), (116, "Treatment", "No"), (117, "Treatment", "No"), (118, "Treatment", "No"), (119, "Treatment", "No"), (120, "Treatment", "No"), (121, "Treatment", "No"), (122, "Treatment", "No"), (123, "Treatment", "No"), (124, "Treatment", "No"), (125, "Treatment", "No"), (126, "Treatment", "No"), (127, "Treatment", "No"), (128, "Treatment", "No"), (129, "Treatment", "No"), (130, "Treatment", "No"), (131, "Treatment", "No"), (132, "Treatment", "No"), (133, "Treatment", "No"), (134, "Treatment", "No"), (135, "Treatment", "No"), (136, "Treatment", "No"), (137, "Treatment", "No"), (138, "Treatment", "No"), (139, "Treatment", "No"), (140, "Treatment", "No"), (141, "Treatment", "No"), (142, "Treatment", "No"), (143, "Treatment", "No"), (144, "Treatment", "No"), (145, "Treatment", "No"), (146, "Treatment", "No"), (147, "Treatment", "No"), (148, "Treatment", "No"), (149, "Treatment", "No"), (150, "Treatment", "No"), (151, "Treatment", "No"), (152, "Treatment", "No"), (153, "Treatment", "No"), (154, "Treatment", "No"), (155, "Treatment", "No"), (156, "Treatment", "No"), (157, "Treatment", "No"), (158, "Treatment", "No"), (159, "Treatment", "No"), (160, "Treatment", "No"), (161, "Treatment", "No"), (162, "Treatment", "No"), (163, "Treatment", "No"), (164, "Treatment", "No"), (165, "Treatment", "No"), (166, "Treatment", "No"), (167, "Treatment", "No"), (168, "Treatment", "No"), (169, "Treatment", "No"), (170, "Treatment", "No"), (171, "Treatment", "No"), (172, "Treatment", "No"), (173, "Treatment", "No"), (174, "Treatment", "No"), (175, "Treatment", "No"), (176, "Treatment", "No"), (177, "Treatment", "No"), (178, "Treatment", "No"), (179, "Treatment", "No"), (180, "Treatment", "No"), (181, "Treatment", "No"), (182, "Treatment", "No"), (183, "Treatment", "No"), (184, "Treatment", "No"), (185, "Treatment", "No"), (186, "Treatment", "No"), (187, "Treatment", "No"), (188, "Treatment", "No"), (189, "Treatment", "No"), (190, "Treatment", "No"), (191, "Treatment", "No"), (192, "Treatment", "No"), (193, "Treatment", "No"), (194, "Treatment", "No"), (195, "Treatment", "No"), (196, "Treatment", "No"), (197, "Treatment", "No"), (198, "Treatment", "No"), (199, "Treatment", "No"), (200, "Treatment", "No"), (201, "Treatment", "No"), (202, "Treatment", "No"), (203, "Treatment", "No"), (204, "Treatment", "No"), (205, "Treatment", "No"), (206, "Treatment", "No"), (207, "Treatment", "No"), (208, "Treatment", "No"), (209, "Treatment", "No"), (210, "Treatment", "No"), (211, "Treatment", "No"), (212, "Treatment", "No"), (213, "Treatment", "No"), (214, "Treatment", "No"), (215, "Treatment", "No"), (216, "Treatment", "No"), (217, "Treatment", "No"), (218, "Treatment", "No"), (219, "Treatment", "No"), (220, "Treatment", "No"), (221, "Treatment", "No"), (222, "Treatment", "No"), (223, "Treatment", "No"), (224, "Treatment", "No"), (225, "Treatment", "No"), (226, "Treatment", "No"), (227, "Treatment", "No"), (228, "Treatment", "No"), (229, "Treatment", "No"), (230, "Treatment", "No"), (231, "Treatment", "No"), (232, "Treatment", "No"), (233, "Treatment", "No"), (234, "Treatment", "No"), (235, "Treatment", "No"), (236, "Treatment", "No"), (237, "Treatment", "No"), (238, "Treatment", "No"), (239, "Treatment", "No"), (240, "Treatment", "No"), (241, "Treatment", "No"), (242, "Treatment", "No"), (243, "Treatment", "No"), (244, "Treatment", "No"), (245, "Treatment", "No"), (246, "Treatment", "No"), (247, "Treatment", "No"), (248, "Treatment", "No"), (249, "Treatment", "No"), (250, "Treatment", "No"), (251, "Treatment", "No"), (252, "Treatment", "No"), (253, "Treatment", "No"), (254, "Treatment", "No"), (255, "Treatment", "No"), (256, "Treatment", "No"), (257, "Treatment", "No"), (258, "Treatment", "No"), (259, "Treatment", "No"), (260, "Treatment", "No"), (261, "Treatment", "No"), (262, "Treatment", "No"), (263, "Treatment", "No"), (264, "Treatment", "No"), (265, "Treatment", "No"), (266, "Treatment", "No"), (267, "Treatment", "No"), (268, "Treatment", "No"), (269, "Treatment", "No"), (270, "Treatment", "No"), (271, "Treatment", "No"), (272, "Treatment", "No"), (273, "Treatment", "No"), (274, "Treatment", "No"), (275, "Treatment", "No"), (276, "Treatment", "No"), (277, "Treatment", "No"), (278, "Treatment", "No"), (279, "Treatment", "No"), (280, "Treatment", "No"), (281, "Treatment", "No"), (282, "Treatment", "No"), (283, "Treatment", "No"), (284, "Treatment", "No"), (285, "Treatment", "No"), (286, "Treatment", "No"), (287, "Treatment", "No"), (288, "Treatment", "No"), (289, "Treatment", "No"), (290, "Treatment", "No"), (291, "Treatment", "No"), (292, "Treatment", "No"), (293, "Treatment", "No"), (294, "Treatment", "No"), (295, "Treatment", "No"), (296, "Treatment", "No"), (297, "Treatment", "No"), (298, "Treatment", "No"), (299, "Treatment", "No"), (300, "Treatment", "No"), (301, "Treatment", "No"), (302, "Treatment", "No"), (303, "Treatment", "No"), (304, "Treatment", "No"), (305, "Treatment", "No"), (306, "Treatment", "No"), (307, "Treatment", "No"), (308, "Treatment", "No"), (309, "Treatment", "No"), (310, "Treatment", "No"), (311, "Treatment", "No"), (312, "Treatment", "No"), (313, "Treatment", "No"), (314, "Treatment", "No"), (315, "Treatment", "No"), (316, "Treatment", "No"), (317, "Treatment", "No"), (318, "Treatment", "No"), (319, "Treatment", "No"), (320, "Treatment", "No"), (321, "Treatment", "No"), (322, "Treatment", "No"), (323, "Treatment", "No"), (324, "Treatment", "No"), (325, "Treatment", "No"), (326, "Treatment", "No"), (327, "Treatment", "No"), (328, "Treatment", "No"), (329, "Treatment", "No"), (330, "Treatment", "No"), (331, "Treatment", "No"), (332, "Treatment", "No"), (333, "Treatment", "No"), (334, "Treatment", "No"), (335, "Treatment", "No"), (336, "Treatment", "No"), (337, "Treatment", "No"), (338, "Treatment", "No"), (339, "Treatment", "No"), (340, "Treatment", "No"), (341, "Treatment", "No"), (342, "Treatment", "No"), (343, "Treatment", "No"), (344, "Treatment", "No"), (345, "Treatment", "No"), (346, "Treatment", "No"), (347, "Treatment", "No"), (348, "Treatment", "No"), (349, "Treatment", "No"), (350, "Treatment", "No"), (351, "Treatment", "No"), (352, "Treatment", "No"), (353, "Treatment", "No"), (354, "Treatment", "No"), (355, "Treatment", "No"), (356, "Treatment", "No"), (357, "Treatment", "No"), (358, "Treatment", "No"), (359, "Treatment", "No"), (360, "Treatment", "No"), (361, "Treatment", "No"), (362, "Treatment", "No"), (363, "Treatment", "No"), (364, "Treatment", "No"), (365, "Treatment", "No"), (366, "Treatment", "No"), (367, "Treatment", "No"), (368, "Treatment", "No"), (369, "Treatment", "No"), (370, "Treatment", "No"), (371, "Treatment", "No"), (372, "Treatment", "No"), (373, "Treatment", "No"), (374, "Treatment", "No"), (375, "Treatment", "No"), (376, "Treatment", "No"), (377, "Treatment", "No"), (378, "Treatment", "No"), (379, "Treatment", "No"), (380, "Treatment", "No"), (381, "Treatment", "No"), (382, "Treatment", "No"), (383, "Treatment", "No"), (384, "Treatment", "No"), (385, "Treatment", "No"), (386, "Treatment", "No"), (387, "Treatment", "No"), (388, "Treatment", "No"), (389, "Placebo", "Yes"), (390, "Placebo", "Yes"), (391, "Placebo", "Yes"), (392, "Placebo", "Yes"), (393, "Placebo", "Yes"), (394, "Placebo", "Yes"), (395, "Placebo", "Yes"), (396, "Placebo", "Yes"), (397, "Placebo", "Yes"), (398, "Placebo", "Yes"), (399, "Placebo", "Yes"), (400, "Placebo", "Yes"), (401, "Placebo", "Yes"), (402, "Placebo", "Yes"), (403, "Placebo", "Yes"), (404, "Placebo", "Yes"), (405, "Placebo", "Yes"), (406, "Placebo", "Yes"), (407, "Placebo", "Yes"), (408, "Placebo", "Yes"), (409, "Placebo", "Yes"), (410, "Placebo", "Yes"), (411, "Placebo", "Yes"), (412, "Placebo", "Yes"), (413, "Placebo", "Yes"), (414, "Placebo", "Yes"), (415, "Placebo", "Yes"), (416, "Placebo", "Yes"), (417, "Placebo", "No"), (418, "Placebo", "No"), (419, "Placebo", "No"), (420, "Placebo", "No"), (421, "Placebo", "No"), (422, "Placebo", "No"), (423, "Placebo", "No"), (424, "Placebo", "No"), (425, "Placebo", "No"), (426, "Placebo", "No"), (427, "Placebo", "No"), (428, "Placebo", "No"), (429, "Placebo", "No"), (430, "Placebo", "No"), (431, "Placebo", "No"), (432, "Placebo", "No"), (433, "Placebo", "No"), (434, "Placebo", "No"), (435, "Placebo", "No"), (436, "Placebo", "No"), (437, "Placebo", "No"), (438, "Placebo", "No"), (439, "Placebo", "No"), (440, "Placebo", "No"), (441, "Placebo", "No"), (442, "Placebo", "No"), (443, "Placebo", "No"), (444, "Placebo", "No"), (445, "Placebo", "No"), (446, "Placebo", "No"), (447, "Placebo", "No"), (448, "Placebo", "No"), (449, "Placebo", "No"), (450, "Placebo", "No"), (451, "Placebo", "No"), (452, "Placebo", "No"), (453, "Placebo", "No"), (454, "Placebo", "No"), (455, "Placebo", "No"), (456, "Placebo", "No"), (457, "Placebo", "No"), (458, "Placebo", "No"), (459, "Placebo", "No"), (460, "Placebo", "No"), (461, "Placebo", "No"), (462, "Placebo", "No"), (463, "Placebo", "No"), (464, "Placebo", "No"), (465, "Placebo", "No"), (466, "Placebo", "No"), (467, "Placebo", "No"), (468, "Placebo", "No"), (469, "Placebo", "No"), (470, "Placebo", "No"), (471, "Placebo", "No"), (472, "Placebo", "No"), (473, "Placebo", "No"), (474, "Placebo", "No"), (475, "Placebo", "No"), (476, "Placebo", "No"), (477, "Placebo", "No"), (478, "Placebo", "No"), (479, "Placebo", "No"), (480, "Placebo", "No"), (481, "Placebo", "No"), (482, "Placebo", "No"), (483, "Placebo", "No"), (484, "Placebo", "No"), (485, "Placebo", "No"), (486, "Placebo", "No"), (487, "Placebo", "No"), (488, "Placebo", "No"), (489, "Placebo", "No"), (490, "Placebo", "No"), (491, "Placebo", "No"), (492, "Placebo", "No"), (493, "Placebo", "No"), (494, "Placebo", "No"), (495, "Placebo", "No"), (496, "Placebo", "No"), (497, "Placebo", "No"), (498, "Placebo", "No"), (499, "Placebo", "No"), (500, "Placebo", "No"), (501, "Placebo", "No"), (502, "Placebo", "No"), (503, "Placebo", "No"), (504, "Placebo", "No"), (505, "Placebo", "No"), (506, "Placebo", "No"), (507, "Placebo", "No"), (508, "Placebo", "No"), (509, "Placebo", "No"), (510, "Placebo", "No"), (511, "Placebo", "No"), (512, "Placebo", "No"), (513, "Placebo", "No"), (514, "Placebo", "No"), (515, "Placebo", "No"), (516, "Placebo", "No"), (517, "Placebo", "No"), (518, "Placebo", "No"), (519, "Placebo", "No"), (520, "Placebo", "No"), (521, "Placebo", "No"), (522, "Placebo", "No"), (523, "Placebo", "No"), (524, "Placebo", "No"), (525, "Placebo", "No"), (526, "Placebo", "No"), (527, "Placebo", "No"), (528, "Placebo", "No"), (529, "Placebo", "No"), (530, "Placebo", "No"), (531, "Placebo", "No"), (532, "Placebo", "No"), (533, "Placebo", "No"), (534, "Placebo", "No"), (535, "Placebo", "No"), (536, "Placebo", "No"), (537, "Placebo", "No"), (538, "Placebo", "No"), (539, "Placebo", "No"), (540, "Placebo", "No"), (541, "Placebo", "No"), (542, "Placebo", "No"), (543, "Placebo", "No"), (544, "Placebo", "No"), (545, "Placebo", "No"), (546, "Placebo", "No"), (547, "Placebo", "No"), (548, "Placebo", "No"), (549, "Placebo", "No"), (550, "Placebo", "No"), (551, "Placebo", "No"), (552, "Placebo", "No"), (553, "Placebo", "No"), (554, "Placebo", "No"), (555, "Placebo", "No"), (556, "Placebo", "No"), (557, "Placebo", "No"), (558, "Placebo", "No"), (559, "Placebo", "No"), (560, "Placebo", "No"), (561, "Placebo", "No"), (562, "Placebo", "No"), (563, "Placebo", "No"), (564, "Placebo", "No"), (565, "Placebo", "No"), (566, "Placebo", "No"), (567, "Placebo", "No"), (568, "Placebo", "No"), (569, "Placebo", "No"), (570, "Placebo", "No"), (571, "Placebo", "No"), (572, "Placebo", "No"), (573, "Placebo", "No"), (574, "Placebo", "No"), (575, "Placebo", "No"), (576, "Placebo", "No"), (577, "Placebo", "No"), (578, "Placebo", "No"), (579, "Placebo", "No"), (580, "Placebo", "No"), (581, "Placebo", "No"), (582, "Placebo", "No"), (583, "Placebo", "No"), (584, "Placebo", "No"), (585, "Placebo", "No"), (586, "Placebo", "No"), (587, "Placebo", "No"), (588, "Placebo", "No"), (589, "Placebo", "No"), (590, "Placebo", "No"), (591, "Placebo", "No"), (592, "Placebo", "No"), (593, "Placebo", "No"), (594, "Placebo", "No"), (595, "Placebo", "No"), (596, "Placebo", "No"), (597, "Placebo", "No"), (598, "Placebo", "No"), (599, "Placebo", "No"), (600, "Placebo", "No"), (601, "Placebo", "No"), (602, "Placebo", "No"), (603, "Placebo", "No"), (604, "Placebo", "No"), (605, "Placebo", "No"), (606, "Placebo", "No"), (607, "Placebo", "No"), (608, "Placebo", "No"), (609, "Placebo", "No"), (610, "Placebo", "No"), (611, "Placebo", "No"), (612, "Placebo", "No"), (613, "Placebo", "No"), (614, "Placebo", "No"), (615, "Placebo", "No"), (616, "Placebo", "No"), (617, "Placebo", "No"), (618, "Placebo", "No"), (619, "Placebo", "No"), (620, "Placebo", "No"), (621, "Placebo", "No"), (622, "Placebo", "No"), (623, "Placebo", "No"), (624, "Placebo", "No"), (625, "Placebo", "No"), (626, "Placebo", "No"), (627, "Placebo", "No"), (628, "Placebo", "No"), (629, "Placebo", "No"), (630, "Placebo", "No"), (631, "Placebo", "No"), (632, "Placebo", "No"), (633, "Placebo", "No"), (634, "Placebo", "No"), (635, "Placebo", "No"), (636, "Placebo", "No"), (637, "Placebo", "No"), (638, "Placebo", "No"), (639, "Placebo", "No"), (640, "Placebo", "No"), (641, "Placebo", "No"), (642, "Placebo", "No"), (643, "Placebo", "No"), (644, "Placebo", "No"), (645, "Placebo", "No"), (646, "Placebo", "No"), (647, "Placebo", "No"), (648, "Placebo", "No"), (649, "Placebo", "No"), (650, "Placebo", "No"), (651, "Placebo", "No"), (652, "Placebo", "No"), (653, "Placebo", "No"), (654, "Placebo", "No"), (655, "Placebo", "No"), (656, "Placebo", "No"), (657, "Placebo", "No"), (658, "Placebo", "No"), (659, "Placebo", "No"), (660, "Placebo", "No"), (661, "Placebo", "No"), (662, "Placebo", "No"), (663, "Placebo", "No"), (664, "Placebo", "No"), (665, "Placebo", "No"), (666, "Placebo", "No"), (667, "Placebo", "No"), (668, "Placebo", "No"), (669, "Placebo", "No"), (670, "Placebo", "No"), (671, "Placebo", "No"), (672, "Placebo", "No"), (673, "Placebo", "No"), (674, "Placebo", "No"), (675, "Placebo", "No"), (676, "Placebo", "No"), (677, "Placebo", "No"), (678, "Placebo", "No"), (679, "Placebo", "No"), (680, "Placebo", "No"), (681, "Placebo", "No"), (682, "Placebo", "No"), (683, "Placebo", "No"), (684, "Placebo", "No"), (685, "Placebo", "No"), (686, "Placebo", "No"), (687, "Placebo", "No"), (688, "Placebo", "No"), (689, "Placebo", "No"), (690, "Placebo", "No"), (691, "Placebo", "No"), (692, "Placebo", "No"), (693, "Placebo", "No"), (694, "Placebo", "No"), (695, "Placebo", "No"), (696, "Placebo", "No"), (697, "Placebo", "No"), (698, "Placebo", "No"), (699, "Placebo", "No"), (700, "Placebo", "No"), (701, "Placebo", "No"), (702, "Placebo", "No"), (703, "Placebo", "No"), (704, "Placebo", "No"), (705, "Placebo", "No"), (706, "Placebo", "No"), (707, "Placebo", "No"), (708, "Placebo", "No"), (709, "Placebo", "No"), (710, "Placebo", "No"), (711, "Placebo", "No"), (712, "Placebo", "No"), (713, "Placebo", "No"), (714, "Placebo", "No"), (715, "Placebo", "No"), (716, "Placebo", "No"), (717, "Placebo", "No"), (718, "Placebo", "No"), (719, "Placebo", "No"), (720, "Placebo", "No"), (721, "Placebo", "No"), (722, "Placebo", "No"), (723, "Placebo", "No"), (724, "Placebo", "No"), (725, "Placebo", "No"), (726, "Placebo", "No"), (727, "Placebo", "No"), (728, "Placebo", "No"), (729, "Placebo", "No"), (730, "Placebo", "No"), (731, "Placebo", "No"), (732, "Placebo", "No"), (733, "Placebo", "No"), (734, "Placebo", "No"), (735, "Placebo", "No"), (736, "Placebo", "No"), (737, "Placebo", "No"), (738, "Placebo", "No"), (739, "Placebo", "No"), (740, "Placebo", "No"), (741, "Placebo", "No"), (742, "Placebo", "No"), (743, "Placebo", "No"), (744, "Placebo", "No"), (745, "Placebo", "No"), (746, "Placebo", "No"), (747, "Placebo", "No"), (748, "Placebo", "No"), (749, "Placebo", "No"), (750, "Placebo", "No"), (751, "Placebo", "No"), (752, "Placebo", "No"), (753, "Placebo", "No"), (754, "Placebo", "No"), (755, "Placebo", "No"), (756, "Placebo", "No"), (757, "Placebo", "No"), (758, "Placebo", "No"), (759, "Placebo", "No"), (760, "Placebo", "No"), (761, "Placebo", "No"), (762, "Placebo", "No"), (763, "Placebo", "No"), (764, "Placebo", "No"), (765, "Placebo", "No"), (766, "Placebo", "No"), (767, "Placebo", "No"), (768, "Placebo", "No"), (769, "Placebo", "No"), (770, "Placebo", "No"), (771, "Placebo", "No"), (772, "Placebo", "No"), (773, "Placebo", "No"), (774, "Placebo", "No")]
            //     .iter()
            //     .map(|x| (x.0, x.1.to_string(), x.2.to_string()))
            //     .collect::<Vec<(u128, String, String)>>();
            
            // initialize default contract with p = 0.05 and fisher's exact test
            let mut research = ClinicalTrialData::default();

            // test raw records upload
            research.upload_raw(sample.clone());
            assert!(research.raw_records == sample);

            // test raw records download
            let download = research.download_raw();
            assert!(download == research.raw_records);

            // test preprocessed records upload 
            research.upload_preprocessed(sample.clone());
            assert!(research.preprocessed_records == sample);

            // test data aggregation
            // assert!(research.data_summary.get(&String::from("Treatment Positive")).unwrap() == 3);
            // assert!(research.data_summary.get(&String::from("Treatment Negative")).unwrap() == 385);
            // assert!(research.data_summary.get(&String::from("Placebo Positive")).unwrap() == 28);
            // assert!(research.data_summary.get(&String::from("Placebo Negative")).unwrap() == 358);
            
            // test statistical test
            assert!(research.result == true);
            ink_env::debug_println!("is result significant: {}", research.result);
        }
    }
}