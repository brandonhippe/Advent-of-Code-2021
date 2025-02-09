import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 18 Part 1

    >>> part1(['[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]', '[[[5,[2,8]],4],[5,[[9,9],0]]]', '[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]', '[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]', '[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]', '[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]', '[[[[5,4],[7,7]],8],[[8,3],8]]', '[[9,3],[[9,9],[6,[4,9]]]]', '[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]', '[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]'])
    4140
    """

    numbers = []
    for line in data:
        numbers.append(snailFishNumber(line))
        numbers[-1].reduction()

    numbers.reverse()
    while len(numbers) != 1:
        nums = [numbers.pop(), numbers.pop()]
        numbers.append(snailFishNumber('[' + nums[0].numberString() + ',' + nums[1].numberString() + ']'))
        numbers[-1].reduction()

    return numbers[0].magnitude()


def part2(data):
    """ 2021 Day 18 Part 2

    >>> part2(['[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]', '[[[5,[2,8]],4],[5,[[9,9],0]]]', '[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]', '[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]', '[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]', '[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]', '[[[[5,4],[7,7]],8],[[8,3],8]]', '[[9,3],[[9,9],[6,[4,9]]]]', '[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]', '[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]'])
    3993
    """

    numbers = []
    for line in data:
        numbers.append(snailFishNumber(line))
        numbers[-1].reduction()

    maximum = float('-inf')
    for i in range(len(numbers)):
        for j in range(len(numbers)):
            if i == j:
                continue
                
            tempNum = snailFishNumber('[' + numbers[i].numberString() + ',' + numbers[j].numberString() + ']')
            tempNum.reduction()
            maximum = max(maximum, tempNum.magnitude())

    return maximum


class snailFishNumber:
    def __init__(self, text, parent=0):
        self.regularNumber = False
        self.parent = parent

        if text[0] != '[':
            self.regularNumber = True
            self.value = int(text)
        else:
            tempText = text[1:-1]
            
            lText = ''
            i = 0
            opened = 0
            while True:
                if tempText[i] == '[':
                    opened += 1
                elif tempText[i] == ']':
                    opened -= 1
                elif tempText[i] == ',' and opened == 0:
                    break

                lText += tempText[i]
                i += 1

            rText = tempText[i + 1:]

            self.children = [snailFishNumber(lText, self), snailFishNumber(rText, self)]
    
    def reduction(self):
        changed = True
        while changed:
            changed = False
            changed = self.explode(0)
            if changed:
                continue
            changed = self.split()

    def explode(self, nested):
        if self.regularNumber:
            return False
        elif nested == 4:
            self.parent.insertLeft(self.children[0].value, self)
            self.parent.insertRight(self.children[1].value, self)
            return True
        
        for (i, child) in enumerate(self.children):
            tempReturn = child.explode(nested + 1)
            if tempReturn:
                if nested == 3:
                    self.children[i] = snailFishNumber('0', self)

                return True
        
        return False

    def insertLeft(self, num, child=0):
        if self.regularNumber:
            self.value += num
            return
        
        if child == 0:
            self.children[1].insertLeft(num)
        else:
            if self.children[0] == child:
                if self.parent != 0:
                    self.parent.insertLeft(num, self)
            else:
                self.children[0].insertLeft(num)

    def insertRight(self, num, child=0):
        if self.regularNumber:
            self.value += num
            return
        
        if child == 0:
            self.children[0].insertRight(num)
        else:
            if self.children[1] == child:
                if self.parent != 0:
                    self.parent.insertRight(num, self)
            else:
                self.children[1].insertRight(num)

    def split(self):
        if self.regularNumber:
            if self.value >= 10:
                self.regularNumber = False
                self.children = [snailFishNumber(str(self.value // 2), self), snailFishNumber(str((self.value + 1) // 2), self)]
                return True
        else:
            for child in self.children:
                tempReturn = child.split()
                if tempReturn:
                    return True
            
        return False

    def numberString(self):
        if self.regularNumber:
            return str(self.value)
        else:
            return '[' + self.children[0].numberString() + ',' + self.children[1].numberString() + ']'

    def magnitude(self):
        if self.regularNumber:
            return self.value
        else:
            return (3 * self.children[0].magnitude()) + (2 * self.children[1].magnitude())


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
        print(f"\nPart 1:\nMagnitude of Result: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLargest Magnitude: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)