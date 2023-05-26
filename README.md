# WhichLicense classification & Compliancy tool
This tool is constructed with the purpose of holding a small database of license identifiers with their respective license classification alongside other important parameters. The tool provides a mechanism to detect the compliance status of various license classifications based on a matrix that it holds internally.

## Basic usage (classification)

```Rust
// loading from file
let classifier = Classifier::from_file("./data");
// or the longer way
let mut classifier = Classifier {
    data: std::collections::HashMap::new(),
};
classifier.load_from_file("./data");

// loading from memory
let classifier = Classifier::from_memory(&raw)
// or the longer way
let mut classifier = Classifier {
    data: std::collections::HashMap::new(),
};
classifier.load_from_memory(&raw);

// saving to a file
classifier.save_to_file("./test_data");

// adding an entry
classifier.add(
    ""<id_here>"",
    ClassificationEntry {
        // the license classification
        classification: LicenseClassification::Unknown,
    },
);

// classifying a license
classifier.classify("<id_here>");
```


## Basic usage (compliance checking)
The ```compliancy_check``` function takes in the host license classification that is under the repository it is supposed to check against and all the other license classifications found (e.g., in all transitive dependencies). Said method returns a ```CompliancyStatus``` enum that can be either ```Compliant``` or ```NonCompliant``` with the latter containing a vector of all the non-compliant licenses classifications (i.e., all the classifications that are directly incompatible with the host license classification).

> NOTE: the ```CompliancyStatus::NonCompliant``` does NOT return the classifications that are found to be compliant with the host classification, only the ones that are not.


> NOTE: the ```LicenseClassification::Unknown``` and ```LicenseClassification::Special``` are always considered to be incompliant with any other license classification; However, there exists an option to force unknown licenses to be compliant.

```Rust
// some examples
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
        &LicenseClassification::Unknown,
        &vec![LicenseClassification::Viral],
        false,
    ),
    CompliancyStatus::NonCompliant(vec![LicenseClassification::Viral])
);

// more advanced example
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
```