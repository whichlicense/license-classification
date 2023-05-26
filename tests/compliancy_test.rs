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
    compliancy_check, CompliancyStatus, LicenseClassification,
};

#[test]
fn it_is_compliant_on_same_pillar() {
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Open],
            false,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Viral],
            false,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Affero],
            false,
        ),
        CompliancyStatus::Compliant
    );
}

#[test]
fn it_fails_on_commercial_pair(){
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Open, LicenseClassification::Commercial],
            true,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );
}

#[test]
fn it_is_compliant_when_lower_pillars_used() {
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Open, LicenseClassification::Viral],
            false,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero
            ],
            false,
        ),
        CompliancyStatus::Compliant
    );
}

#[test]
fn it_is_not_compliant_when_higher_pillars_used(){
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Affero],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Viral],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Affero],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );
}

#[test]
fn it_always_fails_on_unknown() {
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Open],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Open])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Viral],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Affero],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            false,
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
fn it_does_not_fail_on_unknown_if_specified(){
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Open],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Viral],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Affero],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Commercial],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Unknown],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Unknown],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Unknown],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Unknown],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Unknown],
            true,
        ),
        CompliancyStatus::Compliant
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            true,
        ),
        CompliancyStatus::Compliant
    );
}

#[test]
fn it_always_fails_on_special(){
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Open],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Open])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Viral],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Affero],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Affero])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Commercial],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Commercial])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![LicenseClassification::Unknown],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Unknown])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![LicenseClassification::Special("".to_string())],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![LicenseClassification::Special("".to_string())],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![LicenseClassification::Special("".to_string())],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![LicenseClassification::Special("".to_string())],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![LicenseClassification::Special("".to_string())],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![LicenseClassification::Special("".to_string())])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Special("".to_string()),
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            false,
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
fn it_returns_failed_classifications_in_non_compliant(){
    assert_eq!(
        compliancy_check(
            &LicenseClassification::Open,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral, // incompliant starts here
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Viral,
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );


    assert_eq!(
        compliancy_check(
            &LicenseClassification::Affero,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero, 
                LicenseClassification::Commercial, // incompliant starts here
                LicenseClassification::Unknown
            ],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Viral,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero, // incompliant starts here
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Affero,
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Commercial,
            &vec![
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial, // incompliant starts here
                LicenseClassification::Unknown
            ],
            false,
        ),
        CompliancyStatus::NonCompliant(vec![
            LicenseClassification::Commercial,
            LicenseClassification::Unknown
        ])
    );

    assert_eq!(
        compliancy_check(
            &LicenseClassification::Unknown,
            &vec![ // everything is incompliant when unknown is used
                LicenseClassification::Open,
                LicenseClassification::Viral,
                LicenseClassification::Affero,
                LicenseClassification::Commercial,
                LicenseClassification::Unknown
            ],
            false,
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