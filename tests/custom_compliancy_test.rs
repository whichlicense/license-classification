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

use whichlicense_classification::classification::{
    compliancy_check_custom, CompliancyStatus, LicenseClassification,
};
#[macro_use]
extern crate hmap;

fn make_match_arms() -> HashMap<(LicenseClassification, LicenseClassification), bool> {
    hmap!(
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
        (LicenseClassification::Commercial, LicenseClassification::Commercial) => false,
        (LicenseClassification::Commercial, LicenseClassification::Open) => true,
        (LicenseClassification::Commercial, LicenseClassification::Affero) => true,
        (LicenseClassification::Commercial, LicenseClassification::Viral) => true
    )
}

#[test]
fn it_is_compliant_on_same_pillar() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Open],
            &make_match_arms()
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Viral],
            &make_match_arms()
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Affero],
            &make_match_arms()
        ),
        CompliancyStatus::Compliant
    );
}

#[test]
fn it_fails_on_commercial_pair() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Commercial
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );
}

#[test]
fn it_is_compliant_when_lower_pillars_used() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Open, LicenseClassification::Viral],
            &make_match_arms()
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero
            ],
            &make_match_arms()
        ),
        CompliancyStatus::Compliant
    );
}

#[test]
fn it_is_not_compliant_when_higher_pillars_used() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Affero],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Viral],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Affero],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );
}

#[test]
fn it_always_fails_on_unknown() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Open],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Open])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Viral],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Affero],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Open,
            LicenseClassification::Viral,
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );
}

#[test]
fn it_always_fails_on_special() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Open],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Open])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Viral],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Affero],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Commercial],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Unknown],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Special("".to_string())],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Special("".to_string())],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Special("".to_string())],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Special("".to_string())],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Special("".to_string())],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Special("".to_string()),
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Open,
            LicenseClassification::Viral,
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );
}

#[test]
fn it_returns_failed_classifications_in_non_compliant() {
    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Open,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral, // incompliant starts here
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Viral,
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Affero,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial, // incompliant starts here
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Viral,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero, // incompliant starts here
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Commercial,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial, // incompliant starts here
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check_custom(
            &LicenseClassification::Unknown,
            &vec![
                // everything is incompliant when unknown is used
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            &make_match_arms()
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Open,
            LicenseClassification::Viral,
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );
}
