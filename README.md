# worktimers

It's a cli tool to manage work times.

## Usage

### start working

It uses the current timestamp as start date and [lists worked intervals](#list-worked-intervals).

```shell
worktimers start
```

### stop working

It closes the current interval with the current date and [lists worked intervals](#list-worked-intervals).

```shell
worktimers stop
```

### list worked intervals

```shell
worktimers list
```

The output will look like this:

```
┌─────────────────────┬─────────────────────┬──────────┬───────────────────────────┐
│ start               │ stop                │ duration │ duration_with_lunch_break │
├─────────────────────┼─────────────────────┼──────────┼───────────────────────────┤
│ 1970-01-01 00:00:00 │ 1970-01-01 02:00:00 │ 02:00:00 │ 01:30:00                  │
├─────────────────────┼─────────────────────┼──────────┼───────────────────────────┤
│ 2023-01-09 10:25:39 │                     │ 08:00:00 │ 07:30:00                  │
└─────────────────────┴─────────────────────┴──────────┴───────────────────────────┘
```

## Installation

### install rust

https://www.rust-lang.org/tools/install

or

```shell
brew install rustup-init
```

### install worktimers

```shell
make install
```

### add config

add `.worktimers.json` to your home directory and adjust the following content:

```json
{
  "absolute_persistence_path": "/Users/joscha/workingtimes.json",
  "lunch_break_in_mins": 50
}
```

## Development

### create a release

1. bump the version in [Cargo.toml](Cargo.toml)
1. create a commit with all changes
1. tag the commit `git tag 0.2.0`
1. git push && git push --tags
