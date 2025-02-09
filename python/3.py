import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 3 Part 1

    >>> part1(['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010'])
    198
    """

    gamma = ""
    epsilon = ""

    for i in range(len(data[0])):
        counts = [0] * 2
        for line in data:
            counts[int(line[i])] += 1
        
        if counts[0] > counts[1]:
            gamma = gamma + '0'
            epsilon = epsilon + '1'
        else:
            gamma = gamma + '1'
            epsilon = epsilon + '0'

    return int(gamma, 2) * int(epsilon, 2)


def part2(data):
    """ 2021 Day 3 Part 2

    >>> part2(['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010'])
    230
    """

    return oxy_rating(data, 0) * co2_rating(data, 0)


def oxy_rating(data, bit):
    if len(data) == 1:
        return int(data[0], 2)

    newData = []
    counts = [0] * 2
    for line in data:
        counts[int(line[bit])] += 1

    search = ''
    if counts[0] > counts[1]:
        search = '0'
    else:
        search = '1'
    
    for line in data:
        if line[bit] == search:
            newData.append(line)

    return oxy_rating(newData, bit + 1)


def co2_rating(data, bit):
    if len(data) == 1:
        return int(data[0], 2)

    newData = []
    counts = [0] * 2
    for line in data:
        counts[int(line[bit])] += 1

    search = ''
    if counts[0] <= counts[1]:
        search = '0'
    else:
        search = '1'
    
    for line in data:
        if line[bit] == search:
            newData.append(line)

    return co2_rating(newData, bit + 1)


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
        print(f"\nPart 1:\nPower Consumption: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLife Support Rating: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)