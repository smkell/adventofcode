"""Python Solution of Day 4 of the `Advent of Code <http://adventofcode.com/2016/day/4>`
"""

import sys
import hashlib

def part1(line):
    """Calculates the door password using the requirements for part1."""
    print('Part 1')
    print('------')

    password = ''

    i = 0
    while len(password) < 8:
        digest = hashlib.md5("{0}{1}".format(line, i)).hexdigest()
        if digest[0:5] == '00000':
            password = password + digest[5]
        i = i + 1

    print(password)

def part2(line):
    """Calculates the door password using the requirements for part2."""
    print('Part 2')
    print('------')

    password = ['-', '-', '-', '-', '-', '-', '-', '-']

    i = 0
    while '-' in password:
        digest = hashlib.md5("{0}{1}".format(line, i)).hexdigest()
        if digest[0:5] == '00000':
            position = int(digest[5], 16)
            if (position < len(password)) and (password[position] == '-'):
                password[position] = digest[6]
                print(password)
        i = i + 1

    print(''.join(map(str, password)))

def main():
    """Process the lines according to the requirements for part 1."""

    print('Day 5')
    line = sys.stdin.readline()

    part1(line)
    part2(line)



if __name__ == '__main__':
    main()
