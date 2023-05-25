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
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum LicenseClassification {
        /// previously "Open Source"
        Open,
        Viral,
        Affero,
        Commercial,

        /// This entry shall contain the special case defined within
        Special(String),

        Unknown,
    }

    #[derive(Debug)]
    pub enum CompliancyStatus {
        Compliant,
        NonCompliant(Vec<LicenseClassification>),
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct SPDXDetails {
        pub key: Option<String>,
        pub short_name: Option<String>,
        pub name: Option<String>,
        pub category: Option<String>,
    }

    pub fn compliancy_check(
        host_classification: &LicenseClassification,
        found_classifications: &Vec<LicenseClassification>,
        unknown_is_compliant: bool,
    ) -> CompliancyStatus {
        let incompliant_pillars: Vec<LicenseClassification> = found_classifications
            .iter()
            .filter(|c| !match (host_classification, c) {
                // (what you have in your project, another dependency's license classification)
                (_, LicenseClassification::Unknown) | (LicenseClassification::Unknown, _) => {
                    unknown_is_compliant
                }

                (LicenseClassification::Open, LicenseClassification::Open) => true,
                (LicenseClassification::Open, LicenseClassification::Affero) => false,
                (LicenseClassification::Open, LicenseClassification::Viral) => false,
                (LicenseClassification::Open, LicenseClassification::Commercial) => false,
                (LicenseClassification::Viral, LicenseClassification::Viral) => true,
                (LicenseClassification::Viral, LicenseClassification::Open) => true,
                (LicenseClassification::Viral, LicenseClassification::Affero) => false,
                (LicenseClassification::Viral, LicenseClassification::Commercial) => false,
                (LicenseClassification::Affero, LicenseClassification::Open) => true,
                (LicenseClassification::Affero, LicenseClassification::Viral) => true,
                (LicenseClassification::Affero, LicenseClassification::Affero) => true,
                (LicenseClassification::Affero, LicenseClassification::Commercial) => false,
                (LicenseClassification::Commercial, LicenseClassification::Commercial) => true,
                (LicenseClassification::Commercial, LicenseClassification::Open) => true,
                (LicenseClassification::Commercial, LicenseClassification::Affero) => true,
                (LicenseClassification::Commercial, LicenseClassification::Viral) => true,
                (_, LicenseClassification::Special(_)) => false,
                (LicenseClassification::Special(_), _) => false,
                // _ => false,
            })
            .map(|c| c.to_owned())
            .collect();

        match incompliant_pillars.len() {
            0 => CompliancyStatus::Compliant,
            _ => CompliancyStatus::NonCompliant(incompliant_pillars),
        }
    }

    pub fn spdx_category_to_license_classification(spdx_category: &str) -> LicenseClassification {
        match spdx_category {
            "Public Domain" => LicenseClassification::Open,
            "Permissive" => LicenseClassification::Open,
            // open for interpretation whether this is viral or not. need manual human intervention.
            "Copyleft" => LicenseClassification::Unknown,
            // open for interpretation whether this is viral or not. need manual human intervention.
            "Copyleft Limited" => LicenseClassification::Unknown,

            // TODO: i think?
            "Commercial" => LicenseClassification::Commercial,

            // TODO: Lump this in with "Commercial"?
            "Source-available" => LicenseClassification::Commercial,

            // TODO: lump again with commercial?
            "Proprietary Free" => LicenseClassification::Commercial,
            "CLA" => LicenseClassification::Unknown,

            // TODO: not really licenses? require manual human intervention?
            "Patent License" => LicenseClassification::Unknown,
            "Unstated License" => LicenseClassification::Unknown,
            _ => LicenseClassification::Unknown,
        }
    }
}
