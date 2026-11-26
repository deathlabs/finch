use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scanner {
    Zap,
    Grype,
    Semgrep,
}

pub const SCANNERS: &[Scanner] = &[Scanner::Zap, Scanner::Grype, Scanner::Semgrep];

impl Scanner {
    pub fn find(name: &str) -> Result<Self, Error> {
        return SCANNERS
            .iter()
            .copied()
            .find(|scanner| scanner.name().eq_ignore_ascii_case(name.trim()))
            .ok_or_else(|| Error::Invalid(format!("expected a supported scanner, got {name:?}")));
    }

    pub fn name(self) -> &'static str {
        return match self {
            Self::Zap => "ZAP",
            Self::Grype => "Grype",
            Self::Semgrep => "Semgrep",
        };
    }

    pub fn known_rules(self) -> &'static [&'static str] {
        return match self {
            Self::Zap => &[
                "10020", "10021", "10038", "10062", "10202", "40012", "40003", "40018", "90019",
                "90022",
            ],
            Self::Grype | Self::Semgrep => &[],
        };
    }

    pub fn normalize_rule(self, rule: &str) -> Result<String, Error> {
        let rule: &str = rule.trim();
        let invalid = || {
            return Error::Invalid(format!(
                "expected a valid {} rule ID, got {rule:?}",
                self.name()
            ));
        };
        if rule.is_empty() {
            return Err(invalid());
        }
        return match self {
            Self::Zap if rule.bytes().all(|byte| byte.is_ascii_digit()) => {
                let normalized: &str = rule.trim_start_matches('0');
                Ok(if normalized.is_empty() {
                    "0"
                } else {
                    normalized
                }
                .to_owned())
            }
            Self::Grype
                if (3..=63).contains(&rule.len())
                    && rule.as_bytes()[0].is_ascii_alphanumeric()
                    && rule.contains('-')
                    && rule.bytes().all(is_identifier_byte) =>
            {
                if rule[..4.min(rule.len())].eq_ignore_ascii_case("CVE-") {
                    Ok(rule.to_ascii_uppercase())
                } else {
                    Ok(rule.to_owned())
                }
            }
            Self::Semgrep if rule.bytes().all(is_identifier_byte) => Ok(rule.to_owned()),
            _ => Err(invalid()),
        };
    }
}

fn is_identifier_byte(byte: u8) -> bool {
    return byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_normalizes_catalog_rules() {
        assert_eq!(Scanner::Zap.normalize_rule("010038").unwrap(), "10038");
        assert_eq!(Scanner::Zap.normalize_rule("000").unwrap(), "0");
        assert!(Scanner::Zap.normalize_rule("+10038").is_err());

        assert_eq!(
            Scanner::Grype.normalize_rule("cve-2021-44228").unwrap(),
            "CVE-2021-44228"
        );
        assert_eq!(
            Scanner::Grype.normalize_rule("GHSA-a_b.c").unwrap(),
            "GHSA-a_b.c"
        );
        assert!(Scanner::Grype.normalize_rule("GHSA").is_err());
        assert!(Scanner::Grype.normalize_rule("GHSA/a-b").is_err());
        assert!(Scanner::Grype.normalize_rule("-bad").is_err());
        assert!(
            Scanner::Semgrep
                .normalize_rule("python.security.xss")
                .is_ok()
        );
        assert!(Scanner::Semgrep.normalize_rule("two words").is_err());
    }
}
