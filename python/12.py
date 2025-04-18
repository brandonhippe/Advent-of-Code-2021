import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2021 Day 12 Part 1

    >>> part1(['start-A', 'start-b', 'A-c', 'A-b', 'b-d', 'A-end', 'b-end'])
    10
    """

    caves = defaultdict(lambda: set())
    for line in data:
        line = line.split('-')
        
        caves[line[0]].add(line[1])
        caves[line[1]].add(line[0])

    return findPaths('start', caves, defaultdict(lambda: 0), False)


def part2(data):
    """ 2021 Day 12 Part 2

    >>> part2(['start-A', 'start-b', 'A-c', 'A-b', 'b-d', 'A-end', 'b-end'])
    36
    """

    caves = defaultdict(lambda: set())
    for line in data:
        line = line.split('-')
        
        caves[line[0]].add(line[1])
        caves[line[1]].add(line[0])

    return findPaths('start', caves, defaultdict(lambda: 0), True)


def findPaths(curr, caves, visited, p2):
    if curr == "end":
        return 1

    if curr.islower():
        visited[curr] += 1

    paths = 0
    for n in list(caves[curr]):
        if n == "start":
            continue

        if visited[n] == 0 or (p2 and max(visited.values()) == 1):
            paths += findPaths(n, caves, visited, p2)

    if curr.islower():
        visited[curr] -= 1

    return paths


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
        print(f"\nPart 1:\nNumber of paths visiting small caves once: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of paths visiting small caves twice: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)