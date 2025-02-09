import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 8 Part 1

    >>> part1(['acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf'])
    0
    >>> part1(['be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe', 'edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc', 'fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg', 'fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb', 'aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea', 'fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb', 'dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe', 'bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef', 'egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb', 'gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce'])
    26
    """

    patterns = []
    outputs = []

    for line in data:
        temp = line.split(" | ")
        patterns.append(temp[0])
        outputs.append(temp[1])

    for (i, pattern) in enumerate(patterns):
        patterns[i] = pattern.split(' ')

    for (i, output) in enumerate(outputs):
        outputs[i] = output.split(' ')

    for output in outputs:
        for (i, o) in enumerate(output):
            if o[-1] == '\n':
                output[i] = o[0:-1]

    count = 0
    for output in outputs:
        for o in output:            
            if len(o) == 2 or len(o) == 3 or len(o) == 4 or len(o) == 7:
                count += 1

    return count


def part2(data):
    """ 2021 Day 8 Part 2

    >>> part2(['acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf'])
    5353
    >>> part2(['be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe', 'edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc', 'fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg', 'fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb', 'aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea', 'fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb', 'dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe', 'bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef', 'egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb', 'gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce'])
    61229
    """

    patterns = []
    outputs = []

    for line in data:
        temp = line.split(" | ")
        patterns.append(temp[0])
        outputs.append(temp[1])

    for (i, pattern) in enumerate(patterns):
        patterns[i] = pattern.split(' ')

    for (i, output) in enumerate(outputs):
        outputs[i] = output.split(' ')

    for output in outputs:
        for (i, o) in enumerate(output):
            if o[-1] == '\n':
                output[i] = o[0:-1]

    count = 0
    for i in range(len(patterns)):
        c = code(patterns[i], outputs[i])
        count += c.output

    return count


dispPats = ['abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf', 'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg']
occurences = {344: 'a', 204: 'b', 304: 'c', 266: 'd', 96: 'e', 396: 'f', 280: 'g'}


class code:
    def __init__(self, pattern, outputs):
        self.digitReps = [-1] * 10
        self.digitPats = [-1] * 10
        self.pattern = pattern[:]
        self.outputs = outputs[:]        
        self.pairs = {'a': '', 'b': '', 'c': '', 'd': '', 'e': '', 'f': '', 'g': ''}

        for o in range(ord('a'), ord('g') + 1):
            c = chr(o)
            freq = 0
            num = 0
            for p in pattern:
                inP = False
                for test in p:
                    if test == c:
                        inP = True
                        freq += 1
                
                if inP:
                    num += len(p)
            
            index = freq * num
            self.pairs[c] = occurences[index]
        
        self.output = 0
        for o in outputs:
            self.output *= 10
            self.output += dispPats.index(self.getStdOutput(o))

    def getStdOutput(self, output):
        new = ''
        for o in output:
            new += self.pairs[o]

        return sortStr(new)
    

def sortStr(s):
    ordArr = [0] * len(s)
    for (i, l) in enumerate(s):
        ordArr[i] = ord(l)
    
    ordArr.sort()
    ret = ''
    for o in ordArr:
        ret += chr(o)

    return ret


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
        print(f"\nPart 1:\nInstances of 1, 4, 7, and 8: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of output values: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)