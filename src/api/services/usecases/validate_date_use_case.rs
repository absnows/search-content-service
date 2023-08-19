use regex::Regex;

// Regular expression pattern for yyyy-mm-dd format
const REGEX_DATE_PATTERN_VALIDATE: &str =
    r"^(?P<year>\d{4})-(?P<month>0[1-9]|1[0-2])-(?P<day>0[1-9]|[12][0-9]|3[01])$";

pub fn validate(date_to_validate: &String) -> bool {
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
            .parse::<i32>()
            .unwrap();
        let day = captures
            .name("day")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        // add validation for february
        if month == 2 && day > 28 {
            return false;
        }

        if year >= 1900 && month >= 1 && month <= 12 && day >= 1 && day <= 31 {
            return true;
        }
    }

    false
}