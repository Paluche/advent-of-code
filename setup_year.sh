#!/usr/bin/env bash
# Download all input files for each day of a year.

YEAR=$1

set -e

DIR="$YEAR/rust"

git checkout -b "$YEAR"
mkdir -p "$DIR/input"

printf "\n" >> "$DIR/src/lib.rs"

for DAY in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
do
  printf "#[aoc(day%s part1)]\nfn part1(input: &str) -> usize {\n    0\n}\n\n//#[aoc(day%s, part2)]\n//fn part2(input: &str) -> usize {\n//    0\n//}\n"\
    "$DAY" "$DAY" > "$DIR/src/day$DAY.rs"

  curl --cookie ./cookies.txt "https://adventofcode.com/$YEAR/day/$DAY/input" > "$DIR/input/day$DAY.txt"

  echo "pub mod day$DAY;" >> "$DIR/src/lib.rs"
  {
    cd "$DIR"
    cargo fmt
    cd -
  }
  git add "$YEAR/"
  git commit -m "$YEAR: day $DAY"
done
