/*
*   Copyright (c) 2023 Duart Snel
*   All rights reserved.

*   Licensed under the Apache License, Version 2.0 (the "License");
*   you may not use this file except in compliance with the License.
*   You may obtain a copy of the License at

*   http://www.apache.org/licenses/LICENSE-2.0

*   Unless required by applicable law or agreed to in writing, software
*   distributed under the License is distributed on an "AS IS" BASIS,
*   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*   See the License for the specific language governing permissions and
*   limitations under the License.
*/

use whichlicense_classification::classification::{
    compliancy_check, spdx_category_to_license_classification, ClassificationEntry, Classifier,
    LicenseClassification,
};

fn main() {
    println!("Hello, world!");

    // let res = compliancy_check(
    //     &whichlicense_classification::classification::LicenseClassification::Open,
    //     &vec![
    //         whichlicense_classification::classification::LicenseClassification::Open,
    //         whichlicense_classification::classification::LicenseClassification::Viral,
    //     ],
    //     false,
    // );

    // println!("res: {:?}", res);
    let separator = "[:::]";
    let mut classifier = Classifier::new();

    let raw_data = std::fs::read_to_string("./x.txt").unwrap();

    let mut data: Vec<(String, LicenseClassification, String)> = raw_data
        .lines()
        .map(|line| {
            let mut split = line.split(separator);
            (
                split.next().unwrap().to_string(),
                spdx_category_to_license_classification(split.next().unwrap()),
                split.next().unwrap().to_string(),
            )
        })
        .collect();

    for (key, classification, spdx_license_key) in data {
        classifier.add(&key, ClassificationEntry { classification, spdx_license_key: Some(spdx_license_key) })
    }

    classifier.save_to_file("./data");

    // test loading just in case
    let test_load_back = Classifier::from_file("./data");
    assert!(test_load_back.data.len() > 0);



    // TODO: overrides go here. create a file that is committed called defaults.txt and go over them just like above

}
