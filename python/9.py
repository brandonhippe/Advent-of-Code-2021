import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 9 Part 1

    >>> part1(['2199943210', '3987894921', '9856789892', '8767896789', '9899965678'])
    15
    """

    lines = [[int(x) for x in line] for line in data]

    count = 0
    for (i, line) in enumerate(lines):
        for (j, p) in enumerate(line):
            neighbors = neighborCells(lines, j, i)
            low = True
            for n in neighbors:
                if p >= n[0]:
                    low = False

            if low:
                count += p + 1

    return count


def part2(data):
    """ 2021 Day 9 Part 2

    >>> part2(['2199943210', '3987894921', '9856789892', '8767896789', '9899965678'])
    1134
    """

    lines = [[int(x) for x in line] for line in data]

    basins = []
    template = [False] * len(lines[0])
    visited = []
    for i in range(len(lines)):
        visited.append(template[:])
    
    for (i, line) in enumerate(lines):
        for (j, p) in enumerate(line):
            if p == 9:
                visited[i][j] = True

    for (i, line) in enumerate(visited):
        for (j, p) in enumerate(line):
            if not p:
                basins.append(fillRegion(lines, visited, j, i))

    basins.sort(reverse=True)
    product = 1
    for (i, b) in enumerate(basins[:3]):
        product *= b

    return product


def neighborCells(data, x, y):
    neighbors = []
    
    for yOff in range(-1, 2):
        for xOff in range(-1, 2):
            if not (yOff == 0 or xOff == 0):
                continue
                
            ny = y + yOff
            nx = x + xOff
            if 0 <= ny < len(data) and 0 <= nx < len(data[0]) and [x, y] != [nx, ny]:
                neighbors.append([data[ny][nx], [nx, ny]])

    neighbors.sort(key=neighborSort)
    return neighbors


def neighborSort(e):
    return e[0]


def fillRegion(data, visited, x, y):
    visited[y][x] = True
    size = 1

    neighbors = neighborCells(data, x, y)

    for n in neighbors:
        if not visited[n[1][1]][n[1][0]]:
            size += fillRegion(data, visited, n[1][0], n[1][1])

    return size


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
        print(f"\nPart 1:\nSum of risk levels of low points: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct of the size of the 3 largest basins: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)