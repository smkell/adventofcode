"""Python Solution of Day 4 of the `Advent of Code <http://adventofcode.com/2016/day/4>`
"""

import sys
import operator

def parse_room(line):
    components = line.split('[')
    name_sector = components[0]
    checksum = components[1]
    checksum = checksum[0:len(checksum)-1]

    last_dash = name_sector.rfind('-')
    name = name_sector[0:last_dash]
    sector = name_sector[last_dash+1:]

    return (name, sector, checksum)

def check_room(room):
    name, _, checksum = room

    chars = {}
    for char in name:
        if char != '-':
            if char not in chars:
                chars[char] = 1
            else:
                chars[char] = chars[char] + 1

    sorted_dict = sorted(chars.items(), key=lambda x: (-x[1], x[0]))
    new_checksum = ''
    for char, _ in sorted_dict[:5]:
        new_checksum = new_checksum + char

    print '{0} == {1} = {2}'.format(checksum, new_checksum, checksum == new_checksum)
    return checksum == new_checksum

def part1(lines):
    """Process the lines according to the requirements for part 1."""
    print 'Part 1'
    print '------'

    accum = 0
    for line in lines:
        room = parse_room(line)
        sector = room[1]
        if check_room(room):
            accum = accum + int(sector)

    print 'Sum of sector ids for real rooms: {0}'.format(accum)
    print ''

def part2(lines):
    """Process the lines according to the requirements for part 2."""
    print 'Part 2'
    print '------'

def main():
    """Main entry point."""
    print 'Day 3'

    lines = [line.strip() for line in sys.stdin.readlines()]
    part1(lines)
    part2(lines)

if __name__ == '__main__':
    main()
