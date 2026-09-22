use template_cke::Interpolator;
use std::str::FromStr;
use std::sync::LazyLock;

pub const TSV_TEXT: &str = include_str!("../fixtures/examples.tsv");

#[derive(Debug, Clone)]
pub struct TestCase {
    pub template: String,
    pub separator: String,
    pub chunk_coordinates: Vec<u64>,
    pub encoded: String,
    pub note: String,
}

impl TestCase {
    pub fn interpolator(&self) -> Result<Interpolator, String> {
        Interpolator::try_new(&self.template, Some(&self.separator))
    }

    pub fn run(&self) -> Result<(), String> {
        let interpolator = self.interpolator()?;
        let encoded = interpolator.interpolate(&self.chunk_coordinates)?;
        assert_eq!(
            encoded, self.encoded,
            "Encoding mismatch for template='{}', separator='{}':\n  expected: '{}'\n       got: '{}'",
            self.template, self.separator, self.encoded, encoded
        );
        Ok(())
    }

    pub fn from_tsv_row(row: &str) -> Self {
        let fields: Vec<&str> = row.trim().split('\t').collect();
        if fields.len() != 5 {
            panic!("Expected 5 fields in TSV row, got {}", fields.len());
        }
        let template = fields[0].trim().to_string();
        let separator = fields[1].trim().to_string();
        let chunk_coordinates = fields[2]
            .trim()
            .split(',')
            .map(|s| u64::from_str(s.trim()).expect("Invalid chunk coordinate"))
            .collect();
        let encoded = fields[3].trim().to_string();
        let note = fields[4].trim().to_string();

        Self {
            template,
            separator,
            chunk_coordinates,
            encoded,
            note,
        }
    }

    pub fn from_tsv(tsv: &str) -> Vec<Self> {
        let mut lines = tsv
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'));
        let headers: Vec<_> = lines
            .next()
            .expect("TSV must have a header row")
            .trim()
            .split('\t')
            .map(|s| s.trim())
            .collect();
        if headers.len() != 5 {
            panic!("Expected 5 columns in TSV header, got {}", headers.len());
        }
        lines.map(Self::from_tsv_row).collect()
    }
}

pub static TEST_CASES: LazyLock<Vec<TestCase>> = LazyLock::new(|| TestCase::from_tsv(TSV_TEXT));

#[test]
fn test_tsv_cases() {
    assert!(!TEST_CASES.is_empty());
}

#[test]
fn test_interpolator_encoding() {
    for test_case in TEST_CASES.iter() {
        test_case.run().unwrap()
    }
}
