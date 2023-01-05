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
struct CliInterval {
    start: String,
    stop: String,
    duration: String,
}

impl CliInterval {
    pub fn from_interval(interval: &Interval) -> CliInterval {
        let end = interval.stop.map_or_else(|| "".to_string(), |end_date| end_date.to_string());
        CliInterval { start: interval.start.to_string(), stop: end }
    }
    pub fn from_intervals(intervals: Vec<Interval>) -> Vec<CliInterval> {
        intervals
            .iter()
            .map(CliInterval::from_interval)
            .collect()
    }
    pub fn print(cli_intervals: Vec<CliInterval>) {
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

fn read_intervals() -> Vec<Interval> {
    let input = read_file("/Users/Joscha.Harpeng/private/worktimers/tests/resources/workingtimes-full-intervals.json");

    Interval::parse_intervals(input)
}

fn print_intervals(intervals: Vec<Interval>) {
    let cli_intervals: Vec<CliInterval> = CliInterval::from_intervals(intervals);
    CliInterval::print(cli_intervals);
}

fn save_to_file(intervals: &Vec<Interval>) {
    let path = "tests/resources/workingtimes-full-intervals.json";
    fs::write(
        path,
        serde_json::to_string_pretty(&intervals).unwrap(),
    ).expect(format!("Failed to save the intervals to {}", path).as_str())
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use chrono::{Duration, NaiveDateTime};

    use super::*;


    #[test]
    fn creates_cliinterval() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            stop: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };
        let expected = CliInterval {
            start: NaiveDateTime::default().add(Duration::hours(10)).to_string(),
            stop: NaiveDateTime::default().add(Duration::hours(12)).to_string(),
            duration: "".to_string(),
        };

        let cli_interval = CliInterval::from_interval(&interval);
        assert_eq!(cli_interval, actual
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