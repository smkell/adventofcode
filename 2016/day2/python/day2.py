"""Solution of Day 2 of the `Advent of Code <http://adventofcode.com/2016/day/2>`
"""

import argparse
import sys
import StringIO

def indx_from_dir(keypad, cur_indx, direction):
    row, col = cur_indx
    if direction == 'U':
        if (row == 0) or (keypad[row-1][col] == ' '):
            return cur_indx
        else:
            return (row-1, col)
    if direction == 'D':
        if (row == len(keypad)-1) or (keypad[row+1][col] == ' '):
            return cur_indx
        else:
            return (row+1, col)
    if direction == 'L':
        if (col == 0) or (keypad[row][col-1] == ' '):
            return cur_indx
        else:
            return (row, col-1)
    if direction == 'R':
        if (col == len(keypad[row])-1) or (keypad[row][col+1] == ' '):
            return cur_indx
        else:
            return (row, col+1)
    return None

def part1(lines):
    keypad = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9']
    ]

    cur_index = (1, 2)
    code = []
    for line in lines:
        for direction in line.strip()[:]:
            cur_index = indx_from_dir(keypad, cur_index, direction)

        row, col = cur_index
        code.append(keypad[row][col])
    print('Code: {0}'.format(code))

def part2(lines):
    keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' ']
    ]

    cur_index = (2, 0)
    code = []
    for line in lines:
        for direction in line.strip()[:]:
            cur_index = indx_from_dir(keypad, cur_index, direction)

        row, col = cur_index
        code.append(keypad[row][col])

    print('Code: {0}'.format(code))


def main(args):
    """Main entry point.
    """
    print('Day 2')
    raw_directions = sys.stdin.read()

    lines = StringIO.StringIO(raw_directions).readlines()
    if args.part1:
        part1(lines)
    elif args.part2:
        part2(lines)


parser = argparse.ArgumentParser(description='Calculate bathroom code.')
group = parser.add_mutually_exclusive_group()
group.add_argument('--part1', action='store_true')
group.add_argument('--part2', action='store_true')
args = parser.parse_args()

if __name__ == '__main__':
    main(args)
