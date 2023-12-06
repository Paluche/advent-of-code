#! /usr/bin/env python3

""" Day one of Advent of code."""

from argparse import ArgumentParser, FileType

DIGITS_STR = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight',
              'nine']


def get_digits(line, part_two):
    """Return the digits encountered in a given line."""
    ret = []
    for i, x in enumerate(line):
        if '0' <= x <= '9':
            ret.append(ord(x) - ord('0'))
        elif part_two:
            for j, y in enumerate(DIGITS_STR):
                if line[i:].startswith(y):
                    ret.append(j + 1)

    return ret


def compute_line(line, part_two):
    """Process a line extracting first and last digit."""
    digits = get_digits(line, part_two)
    return digits[0] * 10 + digits[-1]


def __main__():
    parser = ArgumentParser(description=__doc__)

    parser.add_argument(dest='input', type=FileType(encoding='utf-8'))
    parser.add_argument('--part-two', action='store_true')

    args = parser.parse_args()

    print(sum(compute_line(line, args.part_two) for line in args.input))


if __name__ == '__main__':
    __main__()
