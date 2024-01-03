#!/usr/bin/env bash
# Download all input files for each day of a year.

YEAR=$1
DAY=$2

set -e

DIR="$YEAR/rust"

mkdir -p "$DIR/input"

if [ "$DAY" -eq 1 ]
then
  {
    cd "$DIR"
    cargo update
    cd -
  }
  printf "\n" >> "$DIR/src/lib.rs"
fi

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
