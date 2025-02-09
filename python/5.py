import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 5 Part 1

    >>> part1(['0,9 -> 5,9', '8,0 -> 0,8', '9,4 -> 3,4', '2,2 -> 2,1', '7,0 -> 7,4', '6,4 -> 2,0', '0,9 -> 2,9', '3,4 -> 1,4', '0,0 -> 8,8', '5,5 -> 8,2'])
    5
    """

    hvpoints = set()
    hvcounted = set()

    for line in data:
        ps = line.split(" -> ")
        p1, p2 = [[int(c) for c in p.split(',')] for p in ps]

        slope = []
        for c1, c2 in zip(p1, p2):
            slope.append(0 if c1 == c2 else (1 if c2 > c1 else -1))

        p = p1
        while p != p2:
            if 0 in slope:
                if tuple(p) in hvpoints:
                    hvpoints.remove(tuple(p))
                    hvcounted.add(tuple(p))
                elif tuple(p) not in hvcounted:
                    hvpoints.add(tuple(p))

            for i in range(len(slope)):
                p[i] += slope[i]

        if 0 in slope:
            if tuple(p) in hvpoints:
                hvpoints.remove(tuple(p))
                hvcounted.add(tuple(p))
            elif tuple(p) not in hvcounted:
                hvpoints.add(tuple(p))

    return len(hvcounted)


def part2(data):
    """ 2021 Day 5 Part 2

    >>> part2(['0,9 -> 5,9', '8,0 -> 0,8', '9,4 -> 3,4', '2,2 -> 2,1', '7,0 -> 7,4', '6,4 -> 2,0', '0,9 -> 2,9', '3,4 -> 1,4', '0,0 -> 8,8', '5,5 -> 8,2'])
    12
    """

    allpoints = set()
    allcounted = set()

    for line in data:
        ps = line.split(" -> ")
        p1, p2 = [[int(c) for c in p.split(',')] for p in ps]

        slope = []
        for c1, c2 in zip(p1, p2):
            slope.append(0 if c1 == c2 else (1 if c2 > c1 else -1))

        p = p1
        while p != p2:
            if tuple(p) in allpoints:
                allpoints.remove(tuple(p))
                allcounted.add(tuple(p))
            elif tuple(p) not in allcounted:
                allpoints.add(tuple(p))

            for i in range(len(slope)):
                p[i] += slope[i]

        if tuple(p) in allpoints:
            allpoints.remove(tuple(p))
            allcounted.add(tuple(p))
        elif tuple(p) not in allcounted:
            allpoints.add(tuple(p))

    return len(allcounted)


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nDangerous Points: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nDangerous Points: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)