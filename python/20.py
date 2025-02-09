import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 20 Part 1

    >>> part1(['..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#', '', '#..#.', '#....', '##..#', '..#..', '..###'])
    35
    """

    enhancement = data[0]
    imgInput = data[2:]

    image = conwayCells()

    for (i, line) in enumerate(imgInput):
        for (j, l) in enumerate(line):
            if l == '#':
                image.addCell([j, i])

    image = image.iterate(enhancement, 0)
    return len(image.iterate(enhancement, 1).cells)


def part2(data):
    """ 2021 Day 20 Part 2

    >>> part2(['..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#', '', '#..#.', '#....', '##..#', '..#..', '..###'])
    3351
    """

    enhancement = data[0]
    imgInput = data[2:]

    image = conwayCells()

    for (i, line) in enumerate(imgInput):
        for (j, l) in enumerate(line):
            if l == '#':
                image.addCell([j, i])

    for day in range(50):
        image = image.iterate(enhancement, day)

    return len(image.cells)


class conwayCell:
    def __init__(self, locArr, dictString):
        self.loc = locArr[:]
        self.index = dictString


class conwayCells:
    def __init__(self):
        self.cells = {}
        self.min = float('inf')
        self.max = float('-inf')
        self.mins = 0
        self.maxs = 0
        self.side = 0

    def addCell(self, loc):
        if self.mins == 0:
            self.mins = [float('inf')] * len(loc)

        if self.maxs == 0:
            self.maxs = [float('-inf')] * len(loc)
        
        for i in range(len(loc)):
            if loc[i] > self.maxs[i]:
                self.maxs[i] = loc[i]

            if loc[i] < self.mins[i]:
                self.mins[i] = loc[i]

        self.updateMinMax()

        string = self.dictString(loc)
        self.cells[string] = conwayCell(loc, string)

    def updateMinMax(self):
        for m in self.maxs:
            if m >= self.max:
                self.max = m + 1

        for m in self.mins:
            if m <= self.min:
                self.min = m - 1

        self.side = self.max - self.min + 1
    
    def dictString(self, loc):
        s = str(loc[0])
        for l in loc[1:]:
            s += ',' + str(l)

        return s

    def onBorder(self, loc):
        for i in range(len(loc)):
            if loc[i] == self.mins[i] or loc[i] == self.maxs[i]:
                return True

        return False

    def inRange(self, loc):
        for i in range(len(loc)):
            if not (self.mins[i] <= loc[i] <= self.maxs[i]):
                return False

        return True

    def getNeighbors(self, loc, dist):
        sideLen = dist * 2 + 1

        neighbors = []
        for i in range(sideLen ** len(loc)):
            temp = [-dist] * len(loc)
            num = i
            for j in range(len(loc)):
                temp[j] += num % sideLen
                num = int(num / sideLen)
            
            neighbors.append(temp)

        output = []
        for neighbor in neighbors:
            temp = []
            for (l, n) in zip(loc, neighbor):
                temp.append(l + n)

            output.append(temp)

        return output 

    def iterate(self, enhancementImg, day):
        dictItem = self.cells.popitem()
        dim = len(dictItem[1].loc)
        self.addCell(dictItem[1].loc)

        nextCells = conwayCells()

        for i in range(self.side ** dim):
            cell = [self.min] * dim
            num = i
            for j in range(dim):
                cell[j] += num % self.side
                num = int(num / self.side)

            if enhancementImg[enchancementIndex(self, cell, enhancementImg, day)] == '#':
                nextCells.addCell(cell)

        return nextCells

    def printCells(self):
        for i in range(self.min, self.max + 1):
            for j in range(self.min, self.max + 1):
                if self.dictString([j, i]) in self.cells:
                    print("#",end='')
                else:
                    print(".",end='')

            print("\n",end='')

        print("\n")


def enchancementIndex(imgInput, loc, enhancementImg, day):
    neighbors = imgInput.getNeighbors(loc, 1)
    index = 0
    for n in neighbors:
        index *= 2
        if imgInput.dictString(n) in imgInput.cells:
            index += 1
            continue

        if enhancementImg[0] == '#' and day % 2 == 1 and not imgInput.inRange(n):
            index += 1
            continue

    return index


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
        print(f"\nPart 1:\nNumber of lit pixels: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of lit pixels: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)