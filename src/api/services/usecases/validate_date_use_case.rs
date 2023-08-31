use regex::Regex;

// Regular expression pattern for yyyy-mm-dd format
const REGEX_DATE_PATTERN_VALIDATE: &str =
    r"^(?P<year>\d{4})-(?P<month>0[1-9]|1[0-2])-(?P<day>0[1-9]|[12][0-9]|3[01])$";

pub fn validate(date_to_validate: &str) -> bool {
    let re = Regex::new(REGEX_DATE_PATTERN_VALIDATE).unwrap();

    if let Some(captures) = re.captures(date_to_validate) {
        let year = captures
            .name("year")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let month = captures
            .name("month")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let day = captures
            .name("day")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let is_leap_year = |year: i32| year % 4 == 0 && !year % 100 == 0 || year % 400 == 0;

        // add validation for february
        if month == 2 {
            if day > 29 {
                return false;
            }
            if day == 29 && !is_leap_year(year) {
                return false;
            }
        }

        if year >= 1900 && (1..=12).contains(&month) && (1..=31).contains(&day) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn validate_date_with_error() {
        let date_with_error = "1900-0101".to_string();
        let is_valid = validate(&date_with_error);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn validate_date_with_februery_error() {
        let date_with_error = "1900-02-31".to_string();
        let is_valid = validate(&date_with_error);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn validate_date_with_all_sentence_error() {
        let date_with_error = "AAAA".to_string();
        let is_valid = validate(&date_with_error);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn validate_date_with_leap_year_february_error() {
        let date_with_error = "1901-02-29".to_string();
        let is_valid = validate(&date_with_error);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn validate_date_with_non_leap_year_february_error() {
        let date_with_error = "2008-02-29".to_string();
        let is_valid = validate(&date_with_error);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn validate_date() {
        let date_with_error = "1900-01-01".to_string();
        let is_valid = validate(&date_with_error);
        assert!(is_valid)
    }
}
