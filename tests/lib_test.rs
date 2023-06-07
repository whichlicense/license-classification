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
#[macro_use]extern crate hmap;
use std::collections::HashMap;

use whichlicense_classification::classification::{
 CompliancyStatus, CompatibilityIndex, LicenseEntry, CompatibilityEntry, CompatibilityStatus
};

fn make_test_data() -> CompatibilityIndex {
    let mut index = CompatibilityIndex {
        data: HashMap::new(),
    };

    index.add("host_license", LicenseEntry {
        name: "host_license".to_owned(),
        compatibility: hmap!(
            "compatible_1".to_owned() => CompatibilityEntry {
                name: "compatible_1".to_owned(),
                compatible: CompatibilityStatus::Compatible,
                explanation: "compatible_1".to_owned(),
            },
            "compatible_2".to_owned() => CompatibilityEntry {
                name: "compatible_2".to_owned(),
                compatible: CompatibilityStatus::Compatible,
                explanation: "compatible_2".to_owned(),
            },
            "incompatible_1".to_owned() => CompatibilityEntry {
                name: "incompatible_1".to_owned(),
                compatible: CompatibilityStatus::Incompatible,
                explanation: "incompatible_1".to_owned(),
            },
            "incompatible_2".to_owned() => CompatibilityEntry {
                name: "incompatible_2".to_owned(),
                compatible: CompatibilityStatus::Incompatible,
                explanation: "incompatible_2".to_owned(),
            },
            "unknown_1".to_owned() => CompatibilityEntry {
                name: "unknown_1".to_owned(),
                compatible: CompatibilityStatus::Unknown,
                explanation: "unknown_1".to_owned(),
            },
            "unknown_2".to_owned() => CompatibilityEntry {
                name: "unknown_2".to_owned(),
                compatible: CompatibilityStatus::Unknown,
                explanation: "unknown_2".to_owned(),
            }
        ),
        spdx_license_key: None,
    });

    index
}


#[test]
fn it_adds_new_values() {
    let mut index = CompatibilityIndex {
        data: HashMap::new(),
    };

    assert!(index.data.is_empty());

    index.add("test", LicenseEntry {
        name: "other".to_owned(),
        compatibility: hmap!(
            "test".to_owned() => CompatibilityEntry {
                name: "test".to_owned(),
                compatible: CompatibilityStatus::Compatible,
                explanation: "test".to_owned(),
            }
        ),
        spdx_license_key: None,
    });

    assert_eq!(index.data.len(), 1);
}

#[test]
fn it_saved_and_loads_from_disk(){
    let index = make_test_data();
    index.save_to_file("test_data");


    let index = CompatibilityIndex::from_file("test_data");
    assert!(index.data.len() > 0);
    assert!(index.data.contains_key("host_license"));
    assert!(index.data.get("host_license").unwrap().compatibility.len() > 0);
    assert!(index.data.get("host_license").unwrap().compatibility.contains_key("compatible_1"));


    let mut index = CompatibilityIndex {
        data: HashMap::new(),
    };
    index.load_from_file("test_data");
    assert!(index.data.len() > 0);
    assert!(index.data.contains_key("host_license"));
    assert!(index.data.get("host_license").unwrap().compatibility.len() > 0);
    assert!(index.data.get("host_license").unwrap().compatibility.contains_key("compatible_1"));

}

#[test]
fn it_is_compliant_on_all_compatible(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "compatible_1",
        "compatible_2",
    ]);

    assert_eq!(res, CompliancyStatus::Compliant);
}

#[test]
fn it_is_incompliant_on_one_incompatible(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "compatible_1",
        "compatible_2",
        "incompatible_1",
    ]);

    assert_eq!(res, CompliancyStatus::NonCompliant(vec![
        CompatibilityEntry {
            name: "incompatible_1".to_owned(),
            compatible: CompatibilityStatus::Incompatible,
            explanation: "incompatible_1".to_owned(),
        }
    ]));
}

#[test]
fn it_is_incompliant_on_multiple_incompatible(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "compatible_1",
        "compatible_2",
        "incompatible_1",
        "incompatible_2",
    ]);

    assert_eq!(res, CompliancyStatus::NonCompliant(vec![
        CompatibilityEntry {
            name: "incompatible_1".to_owned(),
            compatible: CompatibilityStatus::Incompatible,
            explanation: "incompatible_1".to_owned(),
        },
        CompatibilityEntry {
            name: "incompatible_2".to_owned(),
            compatible: CompatibilityStatus::Incompatible,
            explanation: "incompatible_2".to_owned(),
        }
    ]));
}

#[test]
fn it_is_incompliant_on_all_incompatible(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "incompatible_1",
        "incompatible_2",
    ]);

    assert_eq!(res, CompliancyStatus::NonCompliant(vec![
        CompatibilityEntry {
            name: "incompatible_1".to_owned(),
            compatible: CompatibilityStatus::Incompatible,
            explanation: "incompatible_1".to_owned(),
        },
        CompatibilityEntry {
            name: "incompatible_2".to_owned(),
            compatible: CompatibilityStatus::Incompatible,
            explanation: "incompatible_2".to_owned(),
        }
    ]));
}

#[test]
fn it_is_unknown_on_all_unknown(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "unknown_1",
        "unknown_2",
    ]);

    assert_eq!(res, CompliancyStatus::Unknown(vec![
        "unknown_1".to_string(),
        "unknown_2".to_string(),
    ]));
}

#[test]
fn it_is_unknown_on_any_unknown(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "compatible_1",
        "compatible_2",
        "unknown_1",
        "unknown_2",
    ]);

    assert_eq!(res, CompliancyStatus::Unknown(vec![
        "unknown_1".to_string(),
        "unknown_2".to_string(),
    ]));
}

#[test]
fn it_prioritizes_unknown(){
    let index = make_test_data();

    let res = index.check_compliancy("host_license", &vec![
        "compatible_1",
        "compatible_2",
        "incompatible_1",
        "incompatible_2",
        "unknown_1",
        "unknown_2",
    ]);

    assert_eq!(res, CompliancyStatus::Unknown(vec![
        "unknown_1".to_string(),
        "unknown_2".to_string(),
    ]));
}

#[test]
fn it_provides_unknown_leading_on_unknown_leading() {
    let index = make_test_data();

    let res = index.check_compliancy("some_random_license", &vec![
        "compatible_1",
        "compatible_2",
    ]);

    assert_eq!(res, CompliancyStatus::UnknownLeading);
}