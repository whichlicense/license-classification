# WhichLicense classification & Compliancy tool
This tool is constructed with the purpose of holding a small database of license identifiers with their respective license classification alongside other important parameters. The tool provides a mechanism to detect the compliance status of various license classifications based on a matrix that it holds internally.

## Basic usage (classification)

```rust
// loading from file
let classifier = CompatibilityIndex::from_file("./data");
// or the longer way
let mut classifier = CompatibilityIndex {
    data: std::collections::HashMap::new(),
};
classifier.load_from_file("./data");

// loading from memory
let classifier = CompatibilityIndex::from_memory(&raw)
// or the longer way
let mut classifier = CompatibilityIndex {
    data: std::collections::HashMap::new(),
};
classifier.load_from_memory(&raw);

// saving to a file
classifier.save_to_file("./test_data");

// adding an entry
classifier.add("host_license", LicenseEntry {
        name: "host_license".to_owned(),
        compatibility: hmap!(
            "compatible_1".to_owned() => CompatibilityEntry {
                name: "compatible_1".to_owned(),
                compatible: CompatibilityStatus::Compatible,
                explanation: "compatible_1".to_owned(),
            },
            "incompatible_1".to_owned() => CompatibilityEntry {
                name: "incompatible_1".to_owned(),
                compatible: CompatibilityStatus::Incompatible,
                explanation: "incompatible_1".to_owned(),
            },
            
            "unknown_1".to_owned() => CompatibilityEntry {
                name: "unknown_1".to_owned(),
                compatible: CompatibilityStatus::Unknown,
                explanation: "unknown_1".to_owned(),
            },
        ),
        spdx_license_key: None,
});
```


## Compliance checking
The ```compliancy_check``` function takes in the host license classification (i.e., leading license) that is under the repository it is supposed to check against and all the other license classifications (i.e., subordinate licenses) found (e.g., in all transitive dependencies).
Said method returns a ```CompliancyStatus```.

> NOTE: the ```CompliancyStatus::NonCompliant``` does NOT return the classifications that are found to be compliant with the host classification, only the ones that are not.

```rust
// ...
let res = index.check_compliancy("host_license", &vec![
    "compatible_1", // assuming that this is compatible with the host license
    "compatible_2", // assuming that this is compatible with the host license
]);



let res = index.check_compliancy("host_license", &vec![
    "compatible_1", // assuming that this is compatible with the host license
    "compatible_2", // assuming that this is compatible with the host license
    "incompatible_1", // assuming that this is NOT compatible with the host license
    "incompatible_2", // assuming that this is NOT compatible with the host license
]);

assert_eq!(res, CompliancyStatus::NonCompliant(vec![
    // gives back the incompatible entries along with their explanations.
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
assert_eq!(res, CompliancyStatus::Compliant);
```