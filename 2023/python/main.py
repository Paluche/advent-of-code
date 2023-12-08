#! /usr/bin/env python3

"""Module script to help with advent of code puzzle resolution in python.
Defaults runs all the days and part."""

import os
from argparse import ArgumentParser
from importlib import import_module

THIS_DIR = os.path.dirname(os.path.abspath(__file__))
INPUT_DIR = os.path.join(os.path.dirname(THIS_DIR), 'rust', 'input')


def get_part(day_module, part_number):
    """Get the function that resolves a puzzle part for a day. Returns None if
    not found.
    """
    try:
        return getattr(day_module, f'part{part_number}')
    except AttributeError:
        return None


def get_input(day_number):
    """Get the input file for a specific day."""
    input_path = os.path.join(INPUT_DIR, f'day{day_number}.txt')

    with open(input_path, mode='r', encoding='utf-8') as input_file:
        return input_file.readlines()


def load_days():
    """Load the days solutions."""
    solutions_dir = os.path.join(THIS_DIR, 'solutions')

    ret = {}

    for _, _, file_names in os.walk(solutions_dir):
        for file_name in file_names:
            if not file_name.endswith('.py'):
                continue

            if not file_name.startswith('day'):
                continue

            day_number = int(file_name[len('day'):-len('.py')])

            module = import_module(f'solutions.day{day_number}')

            ret[day_number] = (
                (
                    get_part(module, 1),
                    get_part(module, 2)
                ),
                get_input(day_number)
            )
        break

    return ret


def __main__():
    parser = ArgumentParser(description=__doc__)

    parser.add_argument('-d',
                        '--day',
                        dest='day',
                        type=int,
                        help='Run a specific day (between 1 and 24 included)',
                        choices=range(25))

    parser.add_argument('-p',
                        '--part',
                        dest='part',
                        type=int,
                        help='Run a specific part',
                        choices=[1, 2])
    # TODO -t, --timing         Show timing information

    args = parser.parse_args()

    for i, (parts, lines) in load_days().items():
        if args.day and not i == args.day:
            continue

        for j, part in enumerate(parts):
            if part is None:
                continue

            if args.part and not j == args.part:
                continue

            print(f'Day {i}, part {j}: {part(lines)}')


if __name__ == '__main__':
    __main__()
