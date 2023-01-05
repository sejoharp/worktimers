extern crate itertools;

use std::fs;

use chrono::{Duration, Local, NaiveDateTime, Timelike};
use clap::{arg, command, Command};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct JsonEntry {
    date: String,
    time: String,
    status: String,
}

impl JsonEntry {
    pub fn to_entry(&self) -> Entry {
        Entry {
            date: parse_date_time(self.date.as_str(), self.time.as_str()),
            status: self.status.to_owned(),
        }
    }
    fn parse_entries(json_data: String) -> Vec<JsonEntry> {
        let error_message = format!("failed to parse {}", json_data);
        let json_entries: Vec<JsonEntry> =
            serde_json::from_str(json_data.as_str()).expect(error_message.as_str());
        return json_entries;
    }
    fn read_json_entries(path: &str) -> Vec<JsonEntry> {
        let data = read_file(path);
        return JsonEntry::parse_entries(data);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Entry {
    date: NaiveDateTime,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Interval {
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
}

impl Interval {
    pub fn calculate_duration(&self) -> Duration {
        self.end.unwrap_or(now()).signed_duration_since(self.start)
    }
}

impl Interval {
    fn parse_intervals(json_data: String) -> Vec<Interval> {
        let error_message = format!("failed to parse {}", json_data);
        return serde_json::from_str(json_data.as_str()).expect(error_message.as_str());
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Tabled)]
struct CliInterval {
    start: String,
    end: String,
}

impl CliInterval {
    pub fn from_interval(interval: &Interval) -> CliInterval {
        let end = interval.end.map_or_else(|| "".to_string(), |end_date| end_date.to_string());
        CliInterval { start: interval.start.to_string(), end: end }
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
    intervals.push(Interval { start: now(), end: None });
}

fn stop_command(intervals: &mut Vec<Interval>) {
    if let Some(interval) = intervals.pop() {
        intervals.push(Interval { start: interval.start, end: Some(now()) });
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


fn parse_date_time(date: &str, time: &str) -> NaiveDateTime {
    let input = format!("{} {}", date, time);
    let error_message = format!("unable to parse: {}", input);
    NaiveDateTime::parse_from_str(input.as_str(), "%Y-%m-%d %H:%M:%S")
        .expect(error_message.as_str())
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

    use chrono::{Duration, NaiveDate, NaiveDateTime};

    use super::*;

    #[test]
    fn calculates_duation() {
        let interval = Interval {
            start: NaiveDateTime::default().add(Duration::hours(10)),
            end: Some(NaiveDateTime::default().add(Duration::hours(12))),
        };

        assert_eq!(interval.calculate_duration(), Duration::hours(3));
    }

    #[test]
    fn read_intervals_from_file() {
        let input = read_file("tests/resources/workingtimes-full-intervals.json");

        let intervals = Interval::parse_intervals(input);

        assert_eq!(intervals.len(), 220);
    }

    #[test]
    fn prints_intervals() {
        let sample_intervals = vec![
            Interval { start: NaiveDateTime::default(), end: Some(NaiveDateTime::default().add(Duration::hours(2))) },
            Interval { start: NaiveDateTime::default().add(Duration::hours(8)), end: None }];

        print_intervals(sample_intervals);
    }


    #[test]
    fn creates_entry_from_json_entry() {
        let input = JsonEntry {
            date: "2022-09-19".to_string(),
            time: "07:33:50".to_string(),
            status: "start".to_string(),
        };

        let expected = Entry {
            date: NaiveDateTime::parse_from_str("2022-09-19 07:33:50", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            status: "start".to_string(),
        };

        assert_eq!(input.to_entry(), expected);
    }

    #[test]
    fn reads_an_entry_list_from_file() {
        let entries: Vec<JsonEntry> = JsonEntry::read_json_entries("tests/resources/workingtimes.json");

        let expected_entries = vec![
            JsonEntry {
                date: "2020-03-23".to_string(),
                time: "08:10:00".to_string(),
                status: "start".to_string(),
            },
            JsonEntry {
                date: "2020-03-23".to_string(),
                time: "16:20:00".to_string(),
                status: "stop".to_string(),
            },
        ];
        assert_eq!(entries, expected_entries);
    }

    #[test]
    fn parses_date_from_string() {
        let date: &str = "2022-09-19";
        let time: &str = "08:10:00";

        let expected = NaiveDate::from_ymd(2022, 9, 19).and_hms(8, 10, 0);

        let actual: NaiveDateTime = parse_date_time(date, time);

        assert_eq!(actual, expected);
    }

    #[test]
    fn transform_to_json() {
        let entry_list: Vec<JsonEntry> = JsonEntry::read_json_entries("tests/resources/workingtimes-full.json");

        let intervals = entry_list
            .chunks(2)
            // .inspect(print_chunk)
            .filter(remove_different_start_end_date)
            .map(map_to_interval)
            .inspect(|interval| println!("{:?}", interval))
            .collect::<Vec<_>>();
        save_to_file(&intervals);
    }


    #[test]
    fn parses_an_entry_list() {
        let data = r#"
        [{
            "date": "2022-09-19",
            "time": "07:33:50",
            "status": ":start"
        }, {
            "date": "2022-09-19",
            "time": "08:33:50",
            "status": ":stop"
        }, {
            "date": "2022-09-20",
            "time": "08:33:50",
            "status": ":start"
        },{
            "date": "2022-09-21",
            "time": "08:33:50",
            "status": ":start"
        }]"#;
        let expected_entry_list = vec![JsonEntry {
            date: "2022-09-19".to_string(),
            time: "07:33:50".to_string(),
            status: ":start".to_string(),
        }];

        let entry_list: Vec<JsonEntry> = JsonEntry::parse_entries(data.to_string());
        let entries = entry_list
            .chunks(2)
            // .inspect(print_chunk)
            .filter(remove_different_start_end_date)
            .map(map_to_interval)
            .inspect(|interval| println!("{:?}", interval))
            .collect::<Vec<_>>();
        // assert_eq!(entry_list, expected_entry_list);
        print!("{}", serde_json::to_string(&entries).unwrap())
    }

    fn remove_different_start_end_date(interval: &&[JsonEntry]) -> bool {
        if interval.get(0).is_some() &&
            interval.get(1).is_some() {
            let start = interval.get(0).unwrap();
            let end = interval.get(1).unwrap();
            if start.date == end.date {
                true
            } else {
                println!("DIFFERENT DATES IN INTERVAL: {:?} != {:?}", start, end);
                false
            }
        } else {
            true
        }
    }

    fn print_chunk(interval: &&[JsonEntry]) {
        print!("START ");
        if interval.get(0).is_some() {
            print!("{:?}", interval.get(0).unwrap())
        }
        if interval.get(1).is_some() {
            print!("{:?}", interval.get(1).unwrap())
        }
        println!(" END");
    }

    fn map_to_interval(chunk: &[JsonEntry]) -> Interval {
        if chunk.get(0).is_some() && chunk.get(1).is_none() {
            let start_chunk = chunk.get(0).unwrap();
            return Interval {
                start: parse_date_time(start_chunk.date.as_str(), start_chunk.time.as_str()),
                end: None,
            };
        } else {
            let start_chunk = chunk.get(0).unwrap();
            let end_chunk = chunk.get(1).unwrap();
            return Interval {
                start: parse_date_time(start_chunk.date.as_str(), start_chunk.time.as_str()),
                end: Some(parse_date_time(end_chunk.date.as_str(), end_chunk.time.as_str())),
            };
        }
    }
}