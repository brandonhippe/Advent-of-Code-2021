import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq
from collections import defaultdict


def part1(data):
    """ 2021 Day 15 Part 1

    >>> part1(['1163751742', '1381373672', '2136511328', '3694931569', '7463417111', '1319128137', '1359912421', '3125421639', '1293138521', '2311944581'])
    40
    """

    nums = [[int(x) for x in line] for line in data]

    area = {}
    for y, line in enumerate(nums):
        for x, l in enumerate(line):
            area[tuple([x, y])] = l

    return aStar((0, 0), (len(nums[0]) - 1, len(nums) - 1), area)


def part2(data):
    """ 2021 Day 15 Part 2

    >>> part2(['1163751742', '1381373672', '2136511328', '3694931569', '7463417111', '1319128137', '1359912421', '3125421639', '1293138521', '2311944581'])
    315
    """

    nums = [[int(x) for x in line] for line in data]

    areaP2 = {}
    for y in range(len(nums) * 5):
        for x in range(len(nums[0]) * 5):
            n = nums[y % len(nums)][x % len(nums[0])]
            increment = manhatDist([x, y], [x % len(nums[0]), y % len(nums)]) // len(nums)
            areaP2[tuple([x, y])] = ((n - 1 + increment) % 9) + 1

    return aStar((0, 0), (len(nums[0]) * 5 - 1, len(nums) * 5 - 1), areaP2)


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


def aStar(start, end, area):
    openList = [[manhatDist(start, end), 0, start]]
    gScores = defaultdict(lambda: float('inf'))
    closedList = set()

    while len(openList) > 0:
        _, qG, qPos = heapq.heappop(openList)

        if qPos == end:
            return qG

        if qPos in closedList:
            continue

        for nOff in [[0, 1], [0, -1], [1, 0], [-1, 0]]:
            nPos = tuple(c1 + c2 for c1, c2 in zip(qPos, nOff))

            if nPos not in area or nPos in closedList:
                continue

            nG = qG + area[nPos]

            if nG < gScores[nPos]:
                heapq.heappush(openList, [nG + manhatDist(nPos, end), nG, nPos])
                gScores[nPos] = nG

        closedList.add(qPos)

    return -1


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
        print(f"\nPart 1:\nSafest path risk: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSafest path risk: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)