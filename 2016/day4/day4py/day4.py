"""Python Solution of Day 4 of the `Advent of Code <http://adventofcode.com/2016/day/4>`
"""

import sys

def parse_room(line):
    """Parses the components of the room entry from a line of input."""
    components = line.split('[')
    name_sector = components[0]
    checksum = components[1]
    checksum = checksum[0:len(checksum)-1]

    last_dash = name_sector.rfind('-')
    name = name_sector[0:last_dash]
    sector = name_sector[last_dash+1:]

    return (name, int(sector), checksum)

def check_room(room):
    """Verifies the checksum of the given room."""
    name, _, checksum = room

    chars = {}
    for char in name.translate(None, '-'):
        if char not in chars:
            chars[char] = 1
        else:
            chars[char] = chars[char] + 1

    sorted_dict = sorted(chars.items(), key=lambda x: (-x[1], x[0]))
    new_checksum = ''
    for char, _ in sorted_dict[:5]:
        new_checksum = new_checksum + char

    return checksum == new_checksum

def shift_char(char, shift):
    """Shifts the given character to the left alphabetically `shift` times."""
    new_char = chr((((ord(char) - 97) + shift) % 26) + 97)
    return new_char


def decrypt_name(room):
    """Descrypts the name of a room (assumes the room is valid)."""
    name, sector, _ = room

    decoded = ''
    for char in name:
        if char != '-':
            decoded = decoded + shift_char(char, sector)
        else:
            decoded = decoded + ' '

    return decoded


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

    rooms = [room for room in [parse_room(line) for line in lines] if check_room(room)]
    for room in rooms:
        decoded = decrypt_name(room)
        if 'northpole object storage' in decoded:
            print 'decoded - sector {0}'.format(room[1])


def main():
    """Main entry point."""
    print 'Day 3'

    lines = [line.strip() for line in sys.stdin.readlines()]
    part1(lines)
    part2(lines)

if __name__ == '__main__':
    main()
