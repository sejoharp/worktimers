# worktimers

It's a cli tool to manages work times.

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
┌─────────────────────┬─────────────────────┬──────────┐
│ start               │ stop                │ duration │
├─────────────────────┼─────────────────────┼──────────┤
│ 1970-01-01 00:00:00 │ 1970-01-01 02:00:00 │ 02:00:00 │
├─────────────────────┼─────────────────────┼──────────┤
│ 2023-01-07 06:23:19 │                     │ 08:00:00 │
└─────────────────────┴─────────────────────┴──────────┘
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
