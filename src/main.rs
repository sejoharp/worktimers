extern crate itertools;

use std::fs;

use chrono::{Duration, Local, NaiveDateTime, Timelike};
use clap::{command, Command};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Interval {
    start: NaiveDateTime,
    stop: Option<NaiveDateTime>,
}

impl Interval {
    fn calculate_duration(&self) -> Duration {
        self.stop.unwrap_or(now()).signed_duration_since(self.start)
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
    pub fn from_interval(interval: &Interval) -> DisplayInterval {
        match interval.stop {
            Some(end) => DisplayInterval {
                start: interval.start.to_string(),
                stop: end.to_string(),
                duration: format_duration(&interval.calculate_duration()),
            },
            None => DisplayInterval {
                start: interval.start.to_string(),
                stop: "".to_string(),
                duration: format_duration(&interval.calculate_duration()),
            }
        }
    }
    pub fn from_intervals(intervals: Vec<Interval>) -> Vec<DisplayInterval> {
        intervals
            .iter()
            .map(DisplayInterval::from_interval)
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

    if let Some(_matches) = matches.subcommand_matches("list") {
        list_command();
    } else if let Some(_matches) = matches.subcommand_matches("start") {
        let mut intervals = read_intervals();
        start_command(&mut intervals);
        save_to_file(&intervals);
        list_command();
    } else if let Some(_matches) = matches.subcommand_matches("stop") {
        let mut intervals = read_intervals();
        stop_command(&mut intervals);
        save_to_file(&intervals);
        list_command();
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

fn list_command() {
    let intervals = read_intervals();
    print_intervals(intervals);
}

fn now() -> NaiveDateTime {
    Local::now().naive_local().with_nanosecond(0).unwrap()
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("failed to read {}", path).as_str())
}

fn get_persistance_path() -> String {
    let home_directory = std::env::var("HOME").unwrap();
    format!("{}/{}", home_directory, "workingtimes.json")
}

fn read_intervals() -> Vec<Interval> {
    let input = read_file(get_persistance_path().as_str());

    Interval::parse_intervals(input)
}

fn print_intervals(intervals: Vec<Interval>) {
    let cli_intervals: Vec<DisplayInterval> = DisplayInterval::from_intervals(intervals);
    DisplayInterval::print(cli_intervals);
}

fn save_to_file(intervals: &Vec<Interval>) {
    let path = get_persistance_path();
    fs::write(
        path.clone(),
        serde_json::to_string_pretty(&intervals).unwrap(),
    ).expect(format!("Failed to save the intervals to {}", path).as_str())
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

        let cli_interval = DisplayInterval::from_interval(&interval);

        assert_eq!(cli_interval, expected)
    }

    #[test]
    fn calculates_duation() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        assert_eq!(interval.calculate_duration(), Duration::hours(2));
    }

    #[test]
    fn calculates_duation_if_interval_is_ongoing() {
        let interval = Interval {
            start: now().sub(Duration::hours(2)),
            stop: None,
        };

        assert_eq!(interval.calculate_duration(), Duration::hours(2));
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
            Interval { start: NaiveDateTime::default().add(Duration::hours(8)), stop: None }];

        print_intervals(sample_intervals);
    }
}