# worktimers

It's a cli tool to manage work times.

<!-- TOC -->
- [worktimers](#worktimers)
  - [Usage](#usage)
    - [start working](#start-working)
    - [stop working](#stop-working)
    - [list worked intervals](#list-worked-intervals)
  - [Installation](#installation)
    - [install release](#install-release)
    - [install from source](#install-from-source)
  - [install local with nix](#install-local-with-nix)
  - [install via nix home-manager](#install-via-nix-home-manager)
    - [add config](#add-config)
  - [Development](#development)
    - [create a release](#create-a-release)
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

## install local with nix
```shell
nix build
```

## install via nix home-manager
```bash
# move to home-manager config. e.g.:
cd ~/.config/home-manager

# add this as input;
    actpkg = {
      url = "github:sejoharp/act";
    };

# optional: update index
nix flake lock --update-input actpkg

# add this to packages:
inputs.reposyncpkg.packages.${pkgs.stdenv.system}.default

# build generation
nh home build .

# switch generation
nh home switch .
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
```bash
# bump version (patch by default)
make version-update

# create a git commit
git add ...
git commit ...

# tag the commit
make tag-release

# push commit and tag
make push-release
```