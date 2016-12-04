"""Python Solution of Day 3 of the `Advent of Code <http://adventofcode.com/2016/day/3>`
"""

import sys

def valid_triangle(sides):
    """Return true if the given sides specify a valid triangle."""
    return ((sides[0] + sides[1] > sides[2]) and
            (sides[1] + sides[2] > sides[0]) and
            (sides[0] + sides[2] > sides[1]))

def part1(lines):
    """Process the lines using the requirements of Part 1"""
    print 'Part 1'
    print '------'
    accum = 0

    for line in lines:
        sides = [int(x) for x in line.split()]
        if valid_triangle(sides):
            accum = accum + 1

    print 'Number of valid triangles: {0}'.format(accum)
    print ''

def part2(lines):
    """Process the lines using the requirements of Part 2"""
    print 'Part 2'
    print '------'

    accum = 0
    cols = [[], [], []]
    for line in lines:
        sides = [int(x) for x in line.split()]

        for i, side in enumerate(sides):
            cols[i].append(side)

    accum = 0
    for col in cols:
        for page in range(0, len(col) / 3):
            start = page * 3
            end = start + 3
            triangle = col[start:end]
            if valid_triangle(triangle):
                accum = accum + 1


    print 'Number of valid triangles: {0}'.format(accum)
    print ''

def main():
    """Main entry point."""
    lines = sys.stdin.readlines()

    part1(lines)
    part2(lines)

if __name__ == '__main__':
    main()
