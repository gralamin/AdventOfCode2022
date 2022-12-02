# AdventOfCode2022
Advent of Code for 2022 - https://adventofcode.com/2022

## Creating a new date executable

Per day, remember to:
```
export day=day03
cargo new $day
cp day01/Makefile $day/
touch $day/README.md
touch $day/input
touch $day/src/lib.rs
git add $day
git commit -m "$day: Added template"
```

By convention for this repo, so I can ignore it, all programs will be called `<foldername>.day` eg `day01.day`.

To format code, call:

```
make format
```

## Dependencies

To make a new lib:

```
cargo new --lib foo
```

Then you can refer to that lib in the Cargo.toml:

```
[dependencies.my_lib]
path = "../my_lib"
```

And in the code use
```
extern crate my_lib;
```

*Note*: Libs use a slightly different Makefile (no copy)

## Lib list

* `filelib` - A library for common file operations needed in advent of code. Most notably `load_as_ints`, which is used to load input that is just numbers per line.