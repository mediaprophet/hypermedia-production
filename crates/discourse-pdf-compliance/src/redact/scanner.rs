//! PII pattern scanner: credit cards, SSNs, tax IDs, emails, phone numbers, and dates.

#![forbid(unsafe_code)]

use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PiiKind {
    CreditCard,
    Ssn,
    TaxId,
    Email,
    Phone,
    Date,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PiiMatch {
    pub kind: PiiKind,
    pub range: Range<usize>,
    pub text: String,
}

/// Luhn algorithm for credit card checksum verification.
pub fn luhn_check(digits: &[u8]) -> bool {
    if digits.len() < 13 || digits.len() > 19 {
        return false;
    }
    let mut sum: u32 = 0;
    let mut double = false;
    for &d in digits.iter().rev() {
        let mut val = d as u32;
        if double {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
        double = !double;
    }
    sum % 10 == 0
}

/// Scan text for credit card numbers (Visa, MasterCard, Amex, Discover, etc.) with Luhn verification.
pub fn scan_credit_cards(text: &str) -> Vec<PiiMatch> {
    let mut matches = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i < n {
        if !chars[i].is_ascii_digit() {
            i += 1;
            continue;
        }

        // Collect digits and spaces/dashes
        let start = i;
        let mut digits = Vec::new();
        let mut end = i;

        while end < n {
            let c = chars[end];
            if c.is_ascii_digit() {
                digits.push(c as u8 - b'0');
                end += 1;
            } else if (c == ' ' || c == '-') && end + 1 < n && chars[end + 1].is_ascii_digit() {
                end += 1;
            } else {
                break;
            }
        }

        if (13..=19).contains(&digits.len()) && luhn_check(&digits) {
            let matched_str: String = chars[start..end].iter().collect();
            matches.push(PiiMatch {
                kind: PiiKind::CreditCard,
                range: start..end,
                text: matched_str,
            });
            i = end;
        } else {
            i += 1;
        }
    }
    matches
}

/// Scan text for US Social Security Numbers (format `AAA-GG-SSSS` or 9 continuous digits with valid area).
pub fn scan_ssns(text: &str) -> Vec<PiiMatch> {
    let mut matches = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i + 10 < n {
        // Look for 3 digits - 2 digits - 4 digits
        if chars[i].is_ascii_digit()
            && chars[i + 1].is_ascii_digit()
            && chars[i + 2].is_ascii_digit()
            && (chars[i + 3] == '-' || chars[i + 3] == ' ')
            && chars[i + 4].is_ascii_digit()
            && chars[i + 5].is_ascii_digit()
            && (chars[i + 6] == '-' || chars[i + 6] == ' ')
            && chars[i + 7].is_ascii_digit()
            && chars[i + 8].is_ascii_digit()
            && chars[i + 9].is_ascii_digit()
            && chars[i + 10].is_ascii_digit()
        {
            let area: u32 = chars[i..i + 3].iter().collect::<String>().parse().unwrap_or(0);
            let group: u32 = chars[i + 4..i + 6].iter().collect::<String>().parse().unwrap_or(0);
            let serial: u32 = chars[i + 7..i + 11].iter().collect::<String>().parse().unwrap_or(0);

            // Valid SSN checks: area != 000, 666, 900..999; group != 00; serial != 0000
            if area > 0 && area != 666 && area < 900 && group > 0 && serial > 0 {
                let matched_str: String = chars[i..i + 11].iter().collect();
                matches.push(PiiMatch {
                    kind: PiiKind::Ssn,
                    range: i..i + 11,
                    text: matched_str,
                });
                i += 11;
                continue;
            }
        }
        i += 1;
    }
    matches
}

/// Scan text for Employer Identification Numbers / Tax IDs (`XX-XXXXXXX`).
pub fn scan_tax_ids(text: &str) -> Vec<PiiMatch> {
    let mut matches = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i + 9 < n {
        if chars[i].is_ascii_digit()
            && chars[i + 1].is_ascii_digit()
            && chars[i + 2] == '-'
            && chars[i + 3..i + 10].iter().all(|c| c.is_ascii_digit())
        {
            let matched_str: String = chars[i..i + 10].iter().collect();
            matches.push(PiiMatch {
                kind: PiiKind::TaxId,
                range: i..i + 10,
                text: matched_str,
            });
            i += 10;
            continue;
        }
        i += 1;
    }
    matches
}

/// Scan text for email addresses.
pub fn scan_emails(text: &str) -> Vec<PiiMatch> {
    let mut matches = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i < n {
        if chars[i] == '@' && i > 0 && i + 1 < n {
            // Scan backwards for local part
            let mut start = i;
            while start > 0 && is_email_local_char(chars[start - 1]) {
                start -= 1;
            }
            // Scan forwards for domain part
            let mut end = i + 1;
            while end < n && is_email_domain_char(chars[end]) {
                end += 1;
            }
            if start < i && end > i + 1 {
                let domain: String = chars[i + 1..end].iter().collect();
                if domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.') {
                    let matched_str: String = chars[start..end].iter().collect();
                    matches.push(PiiMatch {
                        kind: PiiKind::Email,
                        range: start..end,
                        text: matched_str,
                    });
                    i = end;
                    continue;
                }
            }
        }
        i += 1;
    }
    matches
}

fn is_email_local_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '%' || c == '+' || c == '-'
}

fn is_email_domain_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-'
}

/// Scan text for phone numbers (E.164, North American NANP, and international trunk).
pub fn scan_phones(text: &str) -> Vec<PiiMatch> {
    let mut matches = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut i = 0;
    while i < n {
        let is_lead = chars[i] == '+'
            || (chars[i] == '(' && i + 1 < n && chars[i + 1].is_ascii_digit())
            || chars[i].is_ascii_digit();

        if is_lead && (i == 0 || !chars[i - 1].is_alphanumeric()) {
            let start = i;
            let mut digits = 0;
            let mut end = i;

            while end < n {
                let c = chars[end];
                if c.is_ascii_digit() {
                    digits += 1;
                    end += 1;
                } else if c == ' ' || c == '-' || c == '.' || c == '(' || c == ')' || c == '+' {
                    end += 1;
                } else {
                    break;
                }
            }

            // Remove trailing non-digits
            while end > start && !chars[end - 1].is_ascii_digit() {
                end -= 1;
            }

            if digits >= 7 && digits <= 15 {
                let matched_str: String = chars[start..end].iter().collect();
                matches.push(PiiMatch {
                    kind: PiiKind::Phone,
                    range: start..end,
                    text: matched_str,
                });
                i = end;
                continue;
            }
        }
        i += 1;
    }
    matches
}

/// Scans for all PII patterns concurrently.
pub fn scan_all_pii(text: &str) -> Vec<PiiMatch> {
    let mut all = Vec::new();
    all.extend(scan_credit_cards(text));
    all.extend(scan_ssns(text));
    all.extend(scan_tax_ids(text));
    all.extend(scan_emails(text));
    all.extend(scan_phones(text));
    all.sort_by_key(|m| m.range.start);
    all
}
