mod roman;
pub use roman::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_lower_case_roman() {
        let num = 3549;
        let res: String = Roman::try_from(num).unwrap().into();
        assert_eq!(res, "mmmdxlix".to_string());
    }

    #[test]
    fn test_upper_case_roman() {
        let num = 3549;
        let res: String = Roman::try_from(num).unwrap().to_uppercase().into();
        assert_eq!(res, "MMMDXLIX".to_string());
    }
}
