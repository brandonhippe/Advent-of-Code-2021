import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 24 Part 1
    """

    arr = []
    groups = []
    for line in data:
        if line[0] == "i":
            if len(arr) != 0:
                groups.append(digitProg(arr))
        
            arr = []
        
        arr.append(line)

    groups.append(digitProg(arr))

    return numFromArr(getInputsP1(groups, 0))


def part2(data):
    """ 2021 Day 24 Part 2
    """

    arr = []
    groups = []
    for line in data:
        if line[0] == "i":
            if len(arr) != 0:
                groups.append(digitProg(arr))
        
            arr = []
        
        arr.append(line)

    groups.append(digitProg(arr))

    return numFromArr(getInputsP2(groups, 0))


class digitProg:
    def __init__(self, code):
        self.lines = code[:]
        self.decType = self.lines[4][-2:] == "26"
        if self.decType:
            self.xOffset = int(self.lines[5].split(" ")[-1])
        else:
            self.xOffset = int(self.lines[15].split(" ")[-1])
        
        if self.decType:
            self.zMod26Range = []

            for i in range(1, 10):
                self.zMod26Range.append(i - self.xOffset)
        
    def zOutput(self, w, z):
        if self.decType:
            if z % 26 + self.xOffset == w:
                return z // 26
            else:
                return -1
        else:
            return 26 * z + self.xOffset + w
        

def getInputsP1(groups, z, digitIndex=0):    
    g = groups[0]

    if g.decType:
        try:
            w = g.zMod26Range.index(z % 26) + 1
            nextZ = g.zOutput(w, z)
            if len(groups) == 1:
                if nextZ == 0:
                    return [w]
                else:
                    return []

            nextInputs = getInputsP1(groups[1:], nextZ, digitIndex + 1)

            if len(nextInputs) != 0:
                nextInputs.reverse()
                nextInputs.append(w)
                nextInputs.reverse()
                return nextInputs
        except:
            return []
    else:
        for w in range(9, 0, -1):
            nextZ = g.zOutput(w, z)
            if nextZ != -1:
                if len(groups) == 1:
                    if nextZ == 0:
                        return [w]
                    else:
                        return []

                nextInputs = getInputsP1(groups[1:], nextZ, digitIndex + 1)

                if len(nextInputs) != 0:
                    nextInputs.reverse()
                    nextInputs.append(w)
                    nextInputs.reverse()
                    return nextInputs

    return []


def getInputsP2(groups, z, digitIndex=0):    
    g = groups[0]

    if g.decType:
        try:
            w = g.zMod26Range.index(z % 26) + 1
            nextZ = g.zOutput(w, z)
            if len(groups) == 1:
                if nextZ == 0:
                    return [w]
                else:
                    return []

            nextInputs = getInputsP2(groups[1:], nextZ, digitIndex + 1)

            if len(nextInputs) != 0:
                nextInputs.reverse()
                nextInputs.append(w)
                nextInputs.reverse()
                return nextInputs
        except:
            return []
    else:
        for w in range(1, 10):
            nextZ = g.zOutput(w, z)
            if nextZ != -1:
                if len(groups) == 1:
                    if nextZ == 0:
                        return [w]
                    else:
                        return []

                nextInputs = getInputsP2(groups[1:], nextZ, digitIndex + 1)

                if len(nextInputs) != 0:
                    nextInputs.reverse()
                    nextInputs.append(w)
                    nextInputs.reverse()
                    return nextInputs

    return []


def numFromArr(inputs):
    num = 0
    for i in inputs:
        num *= 10
        num += i

    return num


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
        print(f"\nPart 1:\nLargest Valid Model Number: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSmallest Valid Model Number: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)