""" Day one of Advent of code."""

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


def part1(lines):
    """Day 1 part 1 resolution."""
    return sum(compute_line(line, False) for line in lines)


def part2(lines):
    """Day 1 part 2 resolution."""
    return sum(compute_line(line, True) for line in lines)
