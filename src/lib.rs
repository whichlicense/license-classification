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
    use serde::{Serialize, Deserialize};

    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    pub enum LicenseClassification {
        /// previously "Open Source"
        Open,
        Viral,
        Affero,

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
        host_classification: LicenseClassification,
        found_classifications: &Vec<LicenseClassification>,
    ) -> CompliancyStatus {
        let incompliant_pillars: Vec<LicenseClassification> = found_classifications
            .iter()
            .filter(|c| !match (host_classification, c) {
                (LicenseClassification::Open, LicenseClassification::Open) => true,
                (LicenseClassification::Open, LicenseClassification::Viral) => false,
                (LicenseClassification::Open, LicenseClassification::Affero) => false,
                (LicenseClassification::Viral, LicenseClassification::Open) => true,
                (LicenseClassification::Viral, LicenseClassification::Viral) => true,
                (LicenseClassification::Viral, LicenseClassification::Affero) => false,
                (LicenseClassification::Affero, LicenseClassification::Open) => true,
                (LicenseClassification::Affero, LicenseClassification::Viral) => true,
                (LicenseClassification::Affero, LicenseClassification::Affero) => true,
                _ => false,
            })
            .map(|c| c.to_owned())
            .collect();

        match incompliant_pillars.len() {
            0 => CompliancyStatus::Compliant,
            _ => CompliancyStatus::NonCompliant(incompliant_pillars),
        }
    }

    pub fn spdx_category_to_license_classification(spdx_details: &SPDXDetails) -> LicenseClassification {
        match &spdx_details.category {
            Some(category) => match category.as_str() {
                "Public Domain" => LicenseClassification::Open,
                "Permissive" => LicenseClassification::Open,
                "Copyleft" => LicenseClassification::Viral,
                "Copyleft Limited" => LicenseClassification::Viral,
                "Source-available" => LicenseClassification::Affero,
                "Commercial" => LicenseClassification::Affero,
                "Unstated License" => LicenseClassification::Unknown,
                "Proprietary Free" => LicenseClassification::Unknown,
                "CLA" => LicenseClassification::Unknown,
                "Patent License" => LicenseClassification::Unknown,
                _ => LicenseClassification::Unknown,
            },
            None => LicenseClassification::Unknown,
        }
    }
}
