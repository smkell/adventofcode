"""Solution of Day 2 of the `Advent of Code <http://adventofcode.com/2016/day/2>`
"""

import sys
import StringIO

def indx_from_dir(cur_indx, dir):
    if dir == 'U':
        if cur_indx < 3:
            return cur_indx
        else:
            return cur_indx - 3
    if dir == 'D':
        if cur_indx > 5:
            return cur_indx
        else:
            return cur_indx + 3
    if dir == 'L':
        if cur_indx in [0, 3, 6]:
            return cur_indx
        else:
            return cur_indx - 1
    if dir == 'R':
        if cur_indx in [2, 5, 8]:
            return cur_indx
        else:
            return cur_indx + 1
    return None

def main():
    """Main entry point.
    """
    print('Day 2')
    raw_directions = sys.stdin.read()

    keypad = [1, 2, 3,
              4, 5, 6,
              7, 8, 9]

    cur_index = 4 # start at "5" button
    code = []
    for line in StringIO.StringIO(raw_directions).readlines():
        for direction in line.strip()[:]:
            cur_index = indx_from_dir(cur_index, direction)
        code.append(keypad[cur_index])

    print('Code: {0}'.format(code))
if __name__ == '__main__':
    main()
