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

use std::collections::HashMap;

use whichlicense_classification::classification::{CompatibilityIndex, LicenseEntry, CompatibilityEntry, CompatibilityStatus};

fn main() {
    let separator = "[:::]";
    let mut classifier = CompatibilityIndex::new();

    let raw_data = std::fs::read_to_string("./x.txt").unwrap();

    let data: Vec<(String, String, String)> = raw_data
        .lines()
        .map(|line| {
            let mut split = line.split(separator);
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            )
        })
        .collect();

    for (leading, subordinate, compatible) in data {
        if classifier.data.contains_key(&leading) {
            classifier.data.get_mut(&leading).unwrap().compatibility.insert(subordinate.clone(), CompatibilityEntry {
                name: subordinate.clone(),
                compatible: match compatible.as_str() {
                    "Yes" => CompatibilityStatus::Compatible,
                    "No" => CompatibilityStatus::Incompatible,
                    _ => CompatibilityStatus::Unknown,
                },
                explanation: "".to_string(),
            });
        }

        let mut compatibility = HashMap::new();
        compatibility.insert(subordinate.clone(), CompatibilityEntry {
            name: subordinate,
            compatible: match compatible.as_str() {
                "Yes" => CompatibilityStatus::Compatible,
                "No" => CompatibilityStatus::Incompatible,
                _ => CompatibilityStatus::Unknown,
            },
            explanation: "".to_string(),
        });
        classifier.add(&leading, LicenseEntry {
            name: leading.clone(),
            compatibility,
            spdx_license_key: Some(leading.clone()),
        })
    }

    classifier.save_to_file("./data");

    // test loading just in case
    let test_load_back = CompatibilityIndex::from_file("./data");
    assert!(test_load_back.data.len() > 0);
}
