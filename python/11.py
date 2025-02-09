import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 11 Part 1

    >>> part1(['5483143223', '2745854711', '5264556173', '6141336146', '6357385478', '4167524645', '2176841721', '6882881134', '4846848554', '5283751526'])
    1656
    """

    nums = [[int(x) for x in line] for line in data]
    count = 0
    for _ in range(100):
        for i in range(len(nums)):
            for j in range(len(nums[i])):
                count += updateOcto(nums, j, i)
        
        for i in range(len(nums)):
            for j in range(len(nums[i])):
                nums[i][j] = nums[i][j] % 10

    return count


def part2(data):
    """ 2021 Day 11 Part 2

    >>> part2(['5483143223', '2745854711', '5264556173', '6141336146', '6357385478', '4167524645', '2176841721', '6882881134', '4846848554', '5283751526'])
    195
    """

    nums = [[int(x) for x in line] for line in data]
    count = 0
    day = 0
    while count != len(nums) * len(nums[0]):
        day += 1
        count = 0

        for i in range(len(nums)):
            for j in range(len(nums[i])):
                nums[i][j] = nums[i][j] % 10

        for i in range(len(nums)):
            for j in range(len(nums[i])):
                count += updateOcto(nums, j, i) 

    return day


def neighborOctopi(data, x, y):
    neighbors = []
    
    for yOff in range(-1, 2):
        for xOff in range(-1, 2):
            if yOff == 0 and xOff == 0:
                continue
                
            ny = y + yOff
            nx = x + xOff
            if 0 <= ny < len(data) and 0 <= nx < len(data[0]) and [x, y] != [nx, ny]:
                neighbors.append([nx, ny])

    return neighbors


def updateOcto(data, x, y):
    if data[y][x] == 10:
        return 0

    count = 0
    data[y][x] += 1

    if data[y][x] == 10:
        count += 1
        nOctos = neighborOctopi(data, x, y)
        for n in nOctos:
            count += updateOcto(data, n[0], n[1])
    
    return count


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
        print(f"\nPart 1:\nNumber of Octopi flashes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst Synchronized Flash: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)