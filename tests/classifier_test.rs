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
    ClassificationEntry, Classifier, LicenseClassification,
};

#[test]
fn it_loads_from_file() {
    let classifier = Classifier::from_file("./data");
    assert!(classifier.data.len() > 0);

    let mut classifier = Classifier {
        data: std::collections::HashMap::new(),
    };
    classifier.load_from_file("./data");
    assert!(classifier.data.len() > 0);
}

#[test]
fn it_loads_from_memory() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Unknown,
            spdx_license_key: None,
        },
    );

    let raw = bincode::serialize(&classifier.data).unwrap();

    let mut classifier2 = Classifier::new();
    classifier2.load_from_memory(&raw);
    assert!(classifier2.data.len() > 0);
    assert!(classifier2.data.get("test").is_some());

    let classifier_inline = Classifier::from_memory(&raw);
    assert!(classifier_inline.data.len() > 0);
    assert!(classifier_inline.data.get("test").is_some());
}

#[test]
fn it_saves_to_file() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Unknown,
            spdx_license_key: None,
        },
    );

    classifier.save_to_file("./test_data");

    // assert file exists
    assert!(std::path::Path::new("./test_data").exists());
    assert!(Classifier::from_file("./test_data").data.len() > 0);
    assert!(Classifier::from_file("./test_data")
        .data
        .get("test")
        .is_some());
}

#[test]
fn it_classifies() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Unknown,
            spdx_license_key: None,
        },
    );

    let res = classifier.classify("test");
    assert_eq!(res, LicenseClassification::Unknown);
}

#[test]
fn it_classifies_all() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Open,
            spdx_license_key: None,
        },
    );

    classifier.add(
        "test2",
        ClassificationEntry {
            classification: LicenseClassification::Viral,
            spdx_license_key: None,
        },
    );

    let res = classifier.classify_all(&vec!["test", "test2"]);

    assert!(res.len() == 2);
    assert!(res.get(0).is_some());
    assert!(res.get(1).is_some());
    assert_eq!(res.get(0).unwrap(), &LicenseClassification::Open);
    assert_eq!(res.get(1).unwrap(), &LicenseClassification::Viral);
}

#[test]
fn it_gets_from_key() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Open,
            spdx_license_key: None,
        },
    );

    let res = classifier.get("test");

    assert!(res.is_some());
    assert_eq!(res.unwrap().classification, LicenseClassification::Open);
}

#[test]
fn it_gets_all_from_keys() {
    let mut classifier = Classifier::new();

    classifier.add(
        "test",
        ClassificationEntry {
            classification: LicenseClassification::Open,
            spdx_license_key: None,
        },
    );

    classifier.add(
        "test2",
        ClassificationEntry {
            classification: LicenseClassification::Viral,
            spdx_license_key: None,
        },
    );

    let res = classifier.get_all(&vec!["test", "test2"]);

    assert!(res.len() == 2);
    assert!(res.get(0).is_some());
    assert!(res.get(1).is_some());
    assert_eq!(
        res.get(0).unwrap().unwrap().classification,
        LicenseClassification::Open
    );
    assert_eq!(
        res.get(1).unwrap().unwrap().classification,
        LicenseClassification::Viral
    );
}
