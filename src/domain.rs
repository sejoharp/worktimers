use chrono::{Duration, Local, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Interval {
    pub start: NaiveDateTime,
    pub stop: Option<NaiveDateTime>,
}

impl Interval {
    pub fn end_iteration(&self) -> Result<Interval, String> {
        if self.start.date() != Local::today().naive_local() {
            return Err("interval_not_from_today".to_string());
        }
        return Ok(Interval {
            start: self.start,
            stop: Some(now()),
        });
    }
    pub fn calculate_duration_with_lunch_break(
        &self,
        lunch_break_duration: Option<Duration>,
    ) -> Duration {
        self.stop
            .unwrap_or(now())
            .signed_duration_since(self.start)
            .sub(lunch_break_duration.unwrap_or(Duration::minutes(0)))
    }
    pub fn calculate_duration(&self) -> Duration {
        self.stop.unwrap_or(now()).signed_duration_since(self.start)
    }
    pub fn parse_intervals(json_data: String) -> Vec<Interval> {
        let error_message = format!("failed to parse {}", json_data);
        return serde_json::from_str(json_data.as_str()).expect(error_message.as_str());
    }
}

pub fn now() -> NaiveDateTime {
    Local::now().naive_local().with_nanosecond(0).unwrap()
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};

    use chrono::{Duration, NaiveDateTime};

    use super::*;

    #[test]
    fn calculates_duation_with_lunch_break() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        let expected = Duration::hours(2).sub(Duration::minutes(30));

        assert_eq!(
            interval.calculate_duration_with_lunch_break(Some(Duration::minutes(30))),
            expected
        );
    }

    #[test]
    fn calculates_duration() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        assert_eq!(
            interval.calculate_duration_with_lunch_break(None),
            Duration::hours(2)
        );
    }

    #[test]
    fn calculates_duation_if_interval_is_ongoing() {
        let interval = Interval {
            start: now().sub(Duration::hours(2)),
            stop: None,
        };

        assert_eq!(
            interval.calculate_duration_with_lunch_break(None),
            Duration::hours(2)
        );
    }

    #[test]
    fn does_not_end_intervals_older_then_24h() {
        let input = Interval {
            start: NaiveDateTime::default().sub(Duration::days(1)),
            stop: None,
        };

        let result: Result<Interval, String> = input.end_iteration();

        assert_eq!(result, Err("interval_not_from_today".to_string()));
    }

    #[test]
    fn ends_interval() {
        let input = Interval {
            start: now().sub(Duration::hours(1)),
            stop: None,
        };

        let result: Result<Interval, String> = input.end_iteration();

        assert_eq!(
            result,
            Ok(Interval {
                start: input.start,
                stop: Some(now()),
            })
        );
    }

  
}

