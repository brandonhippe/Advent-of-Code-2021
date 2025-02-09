import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 13 Part 1

    >>> part1(['6,10', '0,14', '9,10', '0,3', '10,4', '4,11', '6,0', '6,12', '4,1', '0,13', '10,12', '3,4', '3,0', '8,4', '1,10', '2,14', '8,10', '9,0', '', 'fold along y=7', 'fold along x=5'])
    17
    """

    points = []
    for line in data:
        if len(line) == 0:
            continue

        if line[0] == 'f':
            info = line.split("fold along ")
            info = info[1:]
            info = info[0].split('=')
            info[-1] = int(info[-1])

            if info[0] == 'x':
                newPoints = []
                for point in points:
                    newPoint = [info[1] - abs(point[0] - info[1]), point[1]]
                    if not newPoint in newPoints:
                        newPoints.append(newPoint)

                points = newPoints[:]
            else:
                newPoints = []
                for point in points:
                    newPoint = [point[0], info[1] - abs(point[1] - info[1])]
                    if not newPoint in newPoints:
                        newPoints.append(newPoint)

                points = newPoints[:]

            break 
        else:
            digits = [int(num) for num in line.split(',')]
            if not digits in points:
                points.append(digits)

    return len(points)  


def part2(data):
    """ 2021 Day 13 Part 2
    """

    points = []
    for line in data:
        if len(line) == 0:
            continue

        if line[0] == 'f':
            info = line.split("fold along ")
            info = info[1:]
            info = info[0].split('=')
            info[-1] = int(info[-1])

            if info[0] == 'x':
                newPoints = []
                for point in points:
                    newPoint = [info[1] - abs(point[0] - info[1]), point[1]]
                    if not newPoint in newPoints:
                        newPoints.append(newPoint)

                points = newPoints[:]
            else:
                newPoints = []
                for point in points:
                    newPoint = [point[0], info[1] - abs(point[1] - info[1])]
                    if not newPoint in newPoints:
                        newPoints.append(newPoint)

                points = newPoints[:]       
        else:
            digits = [int(num) for num in line.split(',')]
            if not digits in points:
                points.append(digits)

    mins = [float('inf')] * 2
    maxs = [float('-inf')] * 2
    for point in points:
        for (i, j) in enumerate(point):
            if j > maxs[i]:
                maxs[i] = j
            
            if j < mins[i]:
                mins[i] = j
    
    string = ''
    for y in range(mins[1], maxs[1] + 1):
        string += '\n'
        for x in range(mins[0], maxs[0] + 1):
            string += '█' if [x, y] in points else ' '

    return string


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
        print(f"\nPart 1:\nNumber of dots visible after first fold: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCode: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)