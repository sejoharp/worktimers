use std::fs;
use std::ops::Sub;

use chrono::{Duration, Local, NaiveDateTime, Timelike};
use clap::{command, Command};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    absolute_persistence_path: String,
    lunch_break_in_mins: i64,
}

impl Config {
    fn parse_into_config(json_data: String) -> Config {
        let error_message = format!("failed to parse {}", json_data);
        return serde_json::from_str(json_data.as_str()).expect(error_message.as_str());
    }
    fn read_config(config_path: &str) -> Config {
        let input = read_file(config_path);

        Config::parse_into_config(input)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Interval {
    start: NaiveDateTime,
    stop: Option<NaiveDateTime>,
}

impl Interval {
    fn calculate_duration(&self, lunch_break_duration: Option<Duration>) -> Duration {
        self.stop
            .unwrap_or(now())
            .signed_duration_since(self.start)
            .sub(lunch_break_duration.unwrap_or(Duration::minutes(0)))
    }

    fn parse_intervals(json_data: String) -> Vec<Interval> {
        let error_message = format!("failed to parse {}", json_data);
        return serde_json::from_str(json_data.as_str()).expect(error_message.as_str());
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Tabled)]
struct DisplayInterval {
    start: String,
    stop: String,
    duration: String,
}

impl DisplayInterval {
    pub fn from_interval(interval: &Interval, lunch_break: Option<Duration>) -> DisplayInterval {
        match interval.stop {
            Some(stop) => DisplayInterval {
                start: interval.start.to_string(),
                stop: stop.to_string(),
                duration: format_duration(&interval.calculate_duration(lunch_break)),
            },
            None => DisplayInterval {
                start: interval.start.to_string(),
                stop: "".to_string(),
                duration: format_duration(&interval.calculate_duration(lunch_break)),
            }
        }
    }
    pub fn from_intervals(intervals: Vec<Interval>, lunch_break: Option<Duration>) -> Vec<DisplayInterval> {
        intervals
            .iter()
            .map(|interval| DisplayInterval::from_interval(&interval, lunch_break))
            .collect()
    }
    pub fn print(cli_intervals: Vec<DisplayInterval>) {
        let mut table = Table::new(cli_intervals);
        table.with(Style::modern());
        println!("{}", table.to_string());
    }
}

fn main() {
    let matches = command!()
        .about("A command line tool to manage working hours.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("prints all intervals")
        ).subcommand(
        Command::new("start")
            .about("start working"))
        .subcommand(
            Command::new("stop")
                .about("stop working")
        )
        .get_matches();

    let config = Config::read_config(get_config().as_str());

    if let Some(_matches) = matches.subcommand_matches("list") {
        list_command(config);
    } else if let Some(_matches) = matches.subcommand_matches("start") {
        let mut intervals = read_intervals(config.absolute_persistence_path.as_str());
        start_command(&mut intervals);
        save_to_file(&intervals, config.absolute_persistence_path.clone());
        list_command(config);
    } else if let Some(_matches) = matches.subcommand_matches("stop") {
        let mut intervals = read_intervals(config.absolute_persistence_path.as_str());
        stop_command(&mut intervals);
        save_to_file(&intervals, config.absolute_persistence_path.clone());
        list_command(config);
    }
}

fn start_command(intervals: &mut Vec<Interval>) {
    intervals.push(Interval { start: now(), stop: None });
}

fn stop_command(intervals: &mut Vec<Interval>) {
    if let Some(interval) = intervals.pop() {
        intervals.push(Interval { start: interval.start, stop: Some(now()) });
    }
}

fn list_command(config: Config) {
    let intervals = read_intervals(config.absolute_persistence_path.as_str());
    print_intervals(intervals, Some(Duration::minutes(config.lunch_break_in_mins)));
}

fn now() -> NaiveDateTime {
    Local::now().naive_local().with_nanosecond(0).unwrap()
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("failed to read {}", path).as_str())
}

fn get_config() -> String {
    let home_directory = std::env::var("HOME").unwrap();
    format!("{}/{}", home_directory, ".worktimers.json")
}

fn read_intervals(path: &str) -> Vec<Interval> {
    let input = read_file(path);

    Interval::parse_intervals(input)
}

fn print_intervals(intervals: Vec<Interval>, lunch_break: Option<Duration>) {
    let cli_intervals: Vec<DisplayInterval> = DisplayInterval::from_intervals(intervals, lunch_break);
    DisplayInterval::print(cli_intervals);
}

fn save_to_file(intervals: &Vec<Interval>, persistence_path: String) {
    fs::write(
        persistence_path.clone(),
        serde_json::to_string_pretty(&intervals).unwrap(),
    ).expect(format!("Failed to save the intervals to {}", persistence_path).as_str())
}

fn format_duration(duration: &Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = (duration.num_seconds() / 60) / 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};

    use chrono::{Duration, NaiveDateTime};

    use super::*;

    #[test]
    fn parses_json_to_config() {
        let data = r#"
        {
            "absolute_persistence_path": "/home/joscha/workingtimes.json",
            "lunch_break_in_mins": 50
        }"#;
        let expected = Config {
            absolute_persistence_path: "/home/joscha/workingtimes.json".to_string(),
            lunch_break_in_mins: 50,
        };

        assert_eq!(Config::parse_into_config(data.to_string()), expected);
    }

    #[test]
    fn creates_display_interval() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };
        let expected = DisplayInterval {
            start: NaiveDateTime::default().add(Duration::hours(10)).to_string(),
            stop: NaiveDateTime::default().add(Duration::hours(12)).to_string(),
            duration: "02:00:00".to_string(),
        };

        let cli_interval = DisplayInterval::from_interval(&interval, None);

        assert_eq!(cli_interval, expected)
    }

    #[test]
    fn calculates_duation_with_lunch_break() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        let expected = Duration::hours(2).sub(Duration::minutes(30));

        assert_eq!(interval.calculate_duration(Some(Duration::minutes(30))), expected);
    }

    #[test]
    fn calculates_duation() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        assert_eq!(interval.calculate_duration(None), Duration::hours(2));
    }

    #[test]
    fn calculates_duation_if_interval_is_ongoing() {
        let interval = Interval {
            start: now().sub(Duration::hours(2)),
            stop: None,
        };

        assert_eq!(interval.calculate_duration(None), Duration::hours(2));
    }

    #[test]
    fn formats_duration() {
        let duration = Duration::seconds(3661);

        let formatted_duration = format_duration(&duration);
        assert_eq!(formatted_duration, "01:01:01");
    }

    #[test]
    fn read_intervals_from_file() {
        let input = read_file("tests/resources/workingtimes-intervals.json");

        let intervals = Interval::parse_intervals(input);

        assert_eq!(intervals.len(), 2);
    }

    #[test]
    fn prints_intervals() {
        let sample_intervals = vec![
            Interval { start: NaiveDateTime::default(), stop: Some(NaiveDateTime::default().add(Duration::hours(2))) },
            Interval { start: now().sub(Duration::hours(8)), stop: None }];

        print_intervals(sample_intervals, None);
    }
}
