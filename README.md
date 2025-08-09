# worktimers

It's a cli tool to manage work times.

<!-- TOC -->
* [worktimers](#worktimers)
  * [Usage](#usage)
    * [start working](#start-working)
    * [stop working](#stop-working)
    * [list worked intervals](#list-worked-intervals)
  * [Installation](#installation)
    * [install release](#install-release)
    * [install from source](#install-from-source)
    * [add config](#add-config)
  * [Development](#development)
    * [create a release](#create-a-release)
<!-- TOC -->

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

### install release
```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/sejoharp/worktimers/refs/heads/main/scripts/install.sh)"
```

### install from source
```shell
# install rust
brew install rustup-init

# build and install worktimers
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
1. make a commit 
2. push it
3. github actions will create a release

