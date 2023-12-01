#! /usr/bin/env python3

""" Day one of Advent of code."""

from argparse import ArgumentParser, FileType

DIGITS = [chr(x) for x in range(ord('0'), ord('9') + 1)]


def get_first_digit(line):
    """Return the first digit we encounter in a given line."""
    for x in line:
        if x in DIGITS:
            return x

    raise RuntimeError('No digit in line')


def compute_line(line):
    """Process a line extracting first and last digit."""
    first_digit = get_first_digit(line)
    last_digit = get_first_digit(reversed(line))
    return int(first_digit + last_digit, base=10)


def __main__():
    parser = ArgumentParser(description=__doc__)

    parser.add_argument(dest='input', type=FileType(encoding='utf-8'))

    args = parser.parse_args()

    print(sum(compute_line(line) for line in args.input))


if __name__ == '__main__':
    __main__()
