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

pub mod classification {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CompliancyStatus {
        Compliant,
        /// A leading license was found to be incompliant with the items in the supplied vector.
        NonCompliant(Vec<CompatibilityEntry>),

        /// The provided leading license is not known to the index.
        UnknownLeading,

        /// The license keys provided within the supplied vector are not known to the given license entry within index (i.e., no compatibility specified for the given subordinate license).
        Unknown(Vec<String>),
    }

    #[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum CompatibilityStatus {
        Compatible,
        Incompatible,
        Unknown,
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct CompatibilityEntry {
        pub name: String,
        pub compatible: CompatibilityStatus,
        pub explanation: String,
    }

    impl CompatibilityEntry {
        pub fn new_unknown(key: &str) -> CompatibilityEntry {
            CompatibilityEntry {
                name: key.to_string(),
                compatible: CompatibilityStatus::Unknown,
                explanation: format!("No compliancy data found for {}", key),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct LicenseEntry {
        pub name: String,
        pub compatibility: HashMap<String, CompatibilityEntry>,
        pub spdx_license_key: Option<String>,
    }

    impl LicenseEntry {
        pub fn check_compatibility(&self, other: &LicenseEntry) -> CompatibilityStatus {
            if let Some(compatibility) = self.compatibility.get(&other.name) {
                compatibility.compatible
            } else {
                CompatibilityStatus::Unknown
            }
        }

        pub fn get_all(&self, keys: &Vec<&str>) -> Vec<Option<&CompatibilityEntry>> {
            keys.iter()
                .map(|key| self.compatibility.get(*key))
                .collect()
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct CompatibilityIndex {
        pub data: HashMap<String, LicenseEntry>,
    }

    impl CompatibilityIndex {
        pub fn new() -> CompatibilityIndex {
            CompatibilityIndex {
                data: HashMap::new(),
            }
        }

        pub fn add(&mut self, key: &str, classification: LicenseEntry) {
            self.data.insert(key.to_owned(), classification);
        }

        pub fn get(&self, key: &str) -> Option<&LicenseEntry> {
            self.data.get(key)
        }

        pub fn get_all(&self, keys: &Vec<&str>) -> Vec<Option<&LicenseEntry>> {
            keys.iter().map(|key| self.get(key)).collect()
        }

        pub fn load_from_memory(&mut self, raw: &[u8]) {
            self.data = bincode::deserialize(&raw[..]).unwrap();
        }

        pub fn from_memory(raw: &[u8]) -> CompatibilityIndex {
            let mut classifier = CompatibilityIndex::new();
            classifier.load_from_memory(raw);
            classifier
        }

        pub fn load_from_file(&mut self, path: &str) {
            let raw = std::fs::read(path).unwrap();
            self.load_from_memory(&raw);
        }

        pub fn from_file(path: &str) -> CompatibilityIndex {
            let mut classifier = CompatibilityIndex::new();
            classifier.load_from_file(path);
            classifier
        }

        pub fn save_to_file(&self, path: &str) {
            let raw = bincode::serialize(&self.data).unwrap();
            std::fs::write(path, raw).unwrap();
        }

        pub fn check_compliancy(
            &self,
            leading_license: &str,
            subordinate_licenses: &Vec<&str>,
        ) -> CompliancyStatus {
            let host_classification = self.get(leading_license);
            if host_classification.is_none() {
                return CompliancyStatus::UnknownLeading;
            }

            let slimmed_matrix: Vec<CompatibilityEntry> = host_classification
                .unwrap()
                .get_all(subordinate_licenses)
                .iter()
                .map(|c| {
                    if let Some(c) = c {
                        c.clone().clone()
                    } else {
                        CompatibilityEntry::new_unknown(leading_license)
                    }
                })
                .collect();

            if slimmed_matrix
                .iter()
                .any(|classification| classification.compatible == CompatibilityStatus::Unknown)
            {
                CompliancyStatus::Unknown(
                    slimmed_matrix
                        .iter()
                        .filter(|classification| {
                            classification.compatible == CompatibilityStatus::Unknown
                        })
                        .map(|classification| classification.name.to_owned())
                        .collect(),
                )
            } else if slimmed_matrix.iter().any(|classification| {
                classification.compatible == CompatibilityStatus::Incompatible
            }) {
                CompliancyStatus::NonCompliant(
                    slimmed_matrix
                        .iter()
                        .filter(|classification| {
                            classification.compatible == CompatibilityStatus::Incompatible
                        })
                        .map(|classification| classification.clone())
                        .collect(),
                )
            } else {
                CompliancyStatus::Compliant
            }
        }
    }
}
