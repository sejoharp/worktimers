use std::fs;

use crate::domain;
use crate::domain::Interval;
use chrono::Duration;
use clap::{command, Command};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(format!("failed to read {}", path).as_str())
}

pub fn read_intervals(path: &str) -> Vec<Interval> {
    let input = read_file(path);

    Interval::parse_intervals(input)
}

pub fn save_to_file(intervals: &Vec<Interval>, persistence_path: String) {
    fs::write(
        persistence_path.clone(),
        serde_json::to_string_pretty(&intervals).unwrap(),
    )
    .expect(format!("Failed to save the intervals to {}", persistence_path).as_str())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Tabled)]
struct DisplayInterval {
    start: String,
    stop: String,
    duration: String,
    duration_with_lunch_break: String,
}

impl DisplayInterval {
    pub fn from_interval(interval: &Interval, lunch_break: Option<Duration>) -> DisplayInterval {
        match interval.stop {
            Some(stop) => DisplayInterval {
                start: interval.start.to_string(),
                stop: stop.to_string(),
                duration: DisplayInterval::format_duration(&interval.calculate_duration()),
                duration_with_lunch_break: DisplayInterval::format_duration(
                    &interval.calculate_duration_with_lunch_break(lunch_break),
                ),
            },
            None => DisplayInterval {
                start: interval.start.to_string(),
                stop: "".to_string(),
                duration: DisplayInterval::format_duration(&interval.calculate_duration()),
                duration_with_lunch_break: DisplayInterval::format_duration(
                    &interval.calculate_duration_with_lunch_break(lunch_break),
                ),
            },
        }
    }
    fn format_duration(duration: &Duration) -> String {
        let seconds = duration.num_seconds() % 60;
        let minutes = (duration.num_seconds() / 60) % 60;
        let hours = (duration.num_seconds() / 60) / 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    pub fn from_intervals(
        intervals: Vec<Interval>,
        lunch_break: Option<Duration>,
    ) -> Vec<DisplayInterval> {
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
    pub fn print_intervals(intervals: Vec<Interval>, lunch_break: Option<Duration>) {
        let cli_intervals: Vec<DisplayInterval> =
            DisplayInterval::from_intervals(intervals, lunch_break);
        DisplayInterval::print(cli_intervals);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub absolute_persistence_path: String,
    pub lunch_break_in_mins: i64,
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
    fn get_config_path() -> String {
        let home_directory = std::env::var("HOME").unwrap();
        format!("{}/{}", home_directory, ".worktimers.json")
    }
}

pub fn start_cli() {
    let matches = command!()
        .about("A command line tool to manage working hours.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("prints all intervals"))
        .subcommand(Command::new("start").about("start working"))
        .subcommand(Command::new("stop").about("stop working"))
        .get_matches();

    let config = Config::read_config(Config::get_config_path().as_str());

    if let Some(_matches) = matches.subcommand_matches("list") {
        list_command(config);
    } else if let Some(_matches) = matches.subcommand_matches("start") {
        let mut intervals = read_intervals(config.absolute_persistence_path.as_str());
        start_command(&mut intervals);
        save_to_file(&intervals, config.absolute_persistence_path.clone());
        list_command(config);
    } else if let Some(_matches) = matches.subcommand_matches("stop") {
        let mut intervals = read_intervals(config.absolute_persistence_path.as_str());
        let result = stop_command(&mut intervals);
        match result {
            Ok(_content) => {
                save_to_file(&intervals, config.absolute_persistence_path.clone());
                list_command(config);
            }
            Err(_message) => {}
        }
    }
}
fn start_command(intervals: &mut Vec<Interval>) {
    intervals.push(Interval {
        start: domain::now(),
        stop: None,
    });
}

fn stop_command(intervals: &mut Vec<Interval>) -> Result<(), String> {
    if let Some(interval) = intervals.pop() {
        let result = interval.end_iteration();
        match result {
            Ok(interval) => {
                intervals.push(interval);
                Ok(())
            }
            Err(message) => {
                println!("Please fix the current interval. It didn't start today.");
                Err(message)
            }
        }
    } else {
        println!("No intervals found. Check config, or start an interval first.");
        Err("no_intervals_found".to_string())
    }
}

fn list_command(config: Config) {
    let intervals = read_intervals(config.absolute_persistence_path.as_str());
    DisplayInterval::print_intervals(
        intervals,
        Some(Duration::minutes(config.lunch_break_in_mins)),
    );
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDateTime};

    use crate::domain::{self, Interval};
    use std::ops::{Add, Sub};

    use super::*;

    #[test]
    fn prints_intervals() {
        let sample_intervals = vec![
            Interval {
                start: NaiveDateTime::default(),
                stop: Some(NaiveDateTime::default().add(Duration::hours(2))),
            },
            Interval {
                start: domain::now().sub(Duration::hours(8)),
                stop: None,
            },
        ];

        DisplayInterval::print_intervals(sample_intervals, Some(Duration::minutes(30)));
    }
}
