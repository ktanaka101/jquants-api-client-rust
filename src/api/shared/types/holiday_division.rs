//! Issue Type Module for Margin Trading Outstandings API.

use serde::{Deserialize, Serialize};

/// Holiday division
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum HolidayDivision {
    /// 0: Non-business day
    #[serde(rename = "0")]
    NonBusinessDay,
    /// 1: Business day
    #[serde(rename = "1")]
    BusinessDay,
    /// 2: Day of TSE Half-Day Trading Sessions
    #[serde(rename = "2")]
    HalfDayTrading,
    /// 3: Non-business days (with holiday trading)
    #[serde(rename = "3")]
    NonBusinessDaysWithHolidayTrading,

    /// Unknown holiday division
    #[serde(untagged)]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_holiday_division_deserialize() {
        let json_data = json!({
            "divisions": [
                { "Name": "0" },
                { "Name": "1" },
                { "Name": "2" },
                { "Name": "3" },
                { "Name": "AAA" },
            ]
        });

        #[derive(Debug, Deserialize)]
        struct Division {
            #[serde(rename = "Name")]
            name: HolidayDivision,
        }

        #[derive(Debug, Deserialize)]
        struct Root {
            divisions: Vec<Division>,
        }

        let root: Root = serde_json::from_value(json_data).unwrap();

        assert_eq!(root.divisions.len(), 5);

        assert_eq!(root.divisions[0].name, HolidayDivision::NonBusinessDay);
        assert_eq!(root.divisions[1].name, HolidayDivision::BusinessDay);
        assert_eq!(root.divisions[2].name, HolidayDivision::HalfDayTrading);
        assert_eq!(
            root.divisions[3].name,
            HolidayDivision::NonBusinessDaysWithHolidayTrading
        );
        assert_eq!(
            root.divisions[4].name,
            HolidayDivision::Unknown("AAA".to_string())
        );
    }
}
