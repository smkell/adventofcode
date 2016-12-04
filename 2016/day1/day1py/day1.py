"""Solution of Day 1 of the `Advent of Code <http://adventofcode.com/2016/day/1>`
"""
import sys

def dist(initial, final):
    x_i, y_i, _ = initial
    x_f, y_f, _ = final

    return abs(x_i - x_f) + abs(y_i - y_f)

def main():
    """Main entry point for the script."""
    raw_directions = sys.stdin.read()
    directions = raw_directions.split(',')

    pos = (0, 0, 0)
    prev_pos = [(0,0)]
    first_intersect = None
    for direction in directions:
        direction = direction.strip()
        turn = direction[0]
        d = int(direction[1:])

        cur_x, cur_y, cur_face = pos
        if turn == 'R':
            new_face = cur_face + 90
        else:
            new_face = cur_face - 90

        if new_face >= 360:
            new_face = 0
        if new_face < 0:
            new_face = 360 + new_face

        new_x, new_y = cur_x, cur_y
        for i in range(d):
            if new_face == 0:
                new_x = new_x
                new_y = new_y + 1
            elif new_face == 90:
                new_x = new_x + 1
                new_y = new_y
            elif new_face == 180:
                new_x = new_x
                new_y = new_y - 1
            elif new_face == 270:
                new_x = new_x - 1
                new_y = new_y
            
            if (new_x, new_y) in prev_pos:
                if first_intersect is None:
                    first_intersect = (new_x, new_y, None)
            prev_pos.append((new_x, new_y))

        new_pos = (new_x, new_y, new_face)
        pos = new_pos

    print('Distance to destination: {0}'.format(dist((0, 0, 0), pos)))
    print('First intersect: {0}'.format(first_intersect))
    print('Distance to first intersect: {0}'.format(dist((0,0,0), first_intersect)))


if __name__ == '__main__':
    main()
