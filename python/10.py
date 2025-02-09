import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 10 Part 1

    >>> part1(['[({(<(())[]>[[{[]{<()<>>', '[(()[<>])]({[<{<<[]>>(', '{([(<{}[<>[]}>{[]{[(<()>', '(((({<>}<{<{<>}{[]{[]{}', '[[<[([]))<([[{}[[()]]]', '[{[{({}]{}}([{[{{{}}([]', '{<[[]]>}<{[{[{[]{()[[[]', '[<(<(<(<{}))><([]([]()', '<{([([[(<>()){}]>(<<{{', '<{([{{}}[<[[[<>{}]]]>[]]'])
    26397
    """

    chunks = []
    for l in data:
        chunks.append([chunk(l[0], 0)])

        for c in l[1:]:
            if chunks[-1][-1].closed:
                chunks[-1].append(chunk(c, 0))
            else :
                chunks[-1][-1].addChar(c)

        for c in chunks[-1]:
            c.setIncomplete()

    count = 0
    for group in chunks:
        for c in group:
            if c.corrupted:
                first = c.corruptedChar()

                if first == ')': 
                    count += 3
                elif first == ']': 
                    count += 57
                elif first == '}':
                    count += 1197
                elif first == '>':
                    count += 25137
                
                break

    return count


def part2(data):
    """ 2021 Day 10 Part 2

    >>> part2(['[({(<(())[]>[[{[]{<()<>>', '[(()[<>])]({[<{<<[]>>(', '{([(<{}[<>[]}>{[]{[(<()>', '(((({<>}<{<{<>}{[]{[]{}', '[[<[([]))<([[{}[[()]]]', '[{[{({}]{}}([{[{{{}}([]', '{<[[]]>}<{[{[{[]{()[[[]', '[<(<(<(<{}))><([]([]()', '<{([([[(<>()){}]>(<<{{', '<{([{{}}[<[[[<>{}]]]>[]]'])
    288957
    """

    chunks = []
    for l in data:
        chunks.append([chunk(l[0], 0)])

        for c in l[1:]:
            if chunks[-1][-1].closed:
                chunks[-1].append(chunk(c, 0))
            else :
                chunks[-1][-1].addChar(c)

        for c in chunks[-1]:
            c.setIncomplete()

    scores = []
    for group in chunks:
        scores.append(0)
        for c in group:
            if c.incomplete and not c.corrupted:
                scores[-1] += score(c.autoComplete())

        if scores[-1] == 0:
            scores.pop()

    scores.sort()

    return scores[int((len(scores) - 1) / 2)]


class chunk:
    def __init__(self, o, parentChunk):
        self.parent = parentChunk
        self.opening = o
        self.closing = ''
        self.childs = []
        self.closed = False
        self.corrupted = False
        self.incomplete = True

    def addChar(self, c):
        added = False

        for child in self.childs:
            if not child.closed:
                added = True
                child.addChar(c)
                break

        if not added:
            if c == '(' or c == '{' or c == '[' or c == '<':
                self.childs.append(chunk(c, self))
            else:
                self.closing = c
                self.closed = True
                if not self.corrupted:
                    if self.opening == '[':
                        self.corrupted = self.closing != ']'
                    elif self.opening == '{':
                        self.corrupted = self.closing != '}'
                    elif self.opening =='(':
                        self.corrupted = self.closing != ')'
                    elif self.opening == '<':
                        self.corrupted = self.closing != '>'

        if self.corrupted and self.parent:
            self.parent.corrupted = True

    def corruptedChar(self):
        c = ''
        for child in self.childs:
            if child.corrupted:
                c += child.corruptedChar()

        if self.closed:
            c += self.closing

        return c[0]

    def autoComplete(self):
        s = ''
        for child in self.childs:
            if child.incomplete:
                s += child.autoComplete()

        if self.incomplete:
            if self.opening == '(':
                s += ')'
            elif self.opening == '{':
                s += '}'
            elif self.opening == '[':
                s += ']'
            elif self.opening == '<':
                s += '>'

        return s

    def setIncomplete(self):
        for c in self.childs:
            c.setIncomplete()
        
        self.incomplete = not self.closed

                
def score(s):
    total = 0
    for l in s:
        total *= 5
        if l == ')':
            total += 1
        elif l == ']':
            total += 2
        elif l == '}':
            total += 3
        elif l == '>':
            total += 4

    return total


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
        print(f"\nPart 1:\nSyntax Error Score: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMedian Score: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)