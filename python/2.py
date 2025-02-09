import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 2 Part 1

    >>> part1(['forward 5', 'down 5', 'forward 8', 'up 3', 'down 8', 'forward 2'])
    150
    """

    # 0 is horizontal position, 1 is depth
    pos = [0] * 2

    for line in data:
        split = line.split(' ')
        num = int(split[-1])
        if line[0] == 'f':
            pos[0] += num
        elif line[0] == 'd':
            pos[1] += num
        elif line[0] == 'u':
            pos[1] -= num

    product = 1
    for i in pos:
        product *= i

    return product


def part2(data):
    """ 2021 Day 2 Part 2

    >>> part2(['forward 5', 'down 5', 'forward 8', 'up 3', 'down 8', 'forward 2'])
    900
    """

    pos = [0] * 2
    aim = 0
    for line in data:
        split = line.split(' ')
        num = int(split[-1])
        if line[0] == 'f':
            pos[0] += num
            pos[1] += num * aim
        elif line[0] == 'd':
            aim += num
        elif line[0] == 'u':
            aim -= num

    product = 1
    for i in pos:
        product *= i

    return product


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
        print(f"\nPart 1:\nProduct: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)