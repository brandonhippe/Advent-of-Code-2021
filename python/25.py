import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 25 Part 1

    >>> part1(['v...>>.vv>', '.vv>>.vv..', '>>.>v>...v', '>>v>>.>.v.', 'v>v.vv.v..', '>.>>..v...', '.vv..>.>v.', 'v.v..>>v.v', '....v..v.>'])
    58
    """

    lines = data[::-1]
    for i, line in enumerate(lines):
        lines[i] = line[::-1]

    for i, line in enumerate(lines):
        newStr = ""
        for _, l in enumerate(line):
            if l == ">":
                newStr += "<"
            elif l == "v":
                newStr += "^"
            else:
                newStr += "."

        lines[i] = list(newStr)
    
    day = 0
    while True:
        prevStr = []
        for line in lines:
            arr = line[:]
            prevStr.append(arr)
        
        lines = increment(lines)
        day += 1

        done = True
        for i in range(len(lines)):
            for j in range(len(lines[i])):
                if lines[i][j] != prevStr[i][j]:
                    done = False
                    break

            if not done:
                break

        if done:
            break

    return day


def part2(data):
    """ 2021 Day 25 Part 2
    """

    return "Christmas has been saved!"


def increment(lines):
    newLines = []
    for line in lines:
        arr = line[:]
        newLines.append(arr)
    
    for (i, line) in enumerate(lines):
        for (j, l) in enumerate(line):
            if l == "<" and lines[i][j - 1] == ".":
                newLines[i][j] = "."
                newLines[i][j - 1] = "<"

    lines = []
    for line in newLines:
        arr = line[:]
        lines.append(arr)

    for (i, line) in enumerate(lines):
        for (j, l) in enumerate(line):
            if l == "^" and lines[i - 1][j] == ".":
                newLines[i][j] = "."
                newLines[i - 1][j] = "^"

    return newLines


def printCucumbers(lines):
    for i in range(len(lines) - 1, -1, -1):
        for j in range(len(lines[i]) - 1, -1, -1):
            c = "."
            if lines[i][j] == "<":
                c = ">"
            if lines [i][j] == "^":
                c = "v"

            print(c,end="")

        print(" ")


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
        print(f"\nPart 1:\nFirst step where no sea cucumbers move: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)