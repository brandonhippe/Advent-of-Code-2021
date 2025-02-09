import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 14 Part 1

    >>> part1(['NNCB', '', 'CH -> B', 'HH -> N', 'CB -> H', 'NH -> C', 'HB -> C', 'HC -> B', 'HN -> C', 'NN -> C', 'BH -> H', 'NC -> B', 'NB -> B', 'BN -> B', 'BB -> N', 'BC -> B', 'CC -> N', 'CN -> C'])
    1588
    """

    template = data[0]
    templatePairs = [template[i:i+2] for i in range(0, len(template) - 1)]
    pairs = {}
    for pair in templatePairs:
        if pair in pairs:
            pairs[pair] += 1
        else:
            pairs[pair] = 1
    
    lines = data[2:]

    occurrances = {}
    for t in template:
        if t in occurrances:
            occurrances[t] += 1
        else:
            occurrances[t] = 1

    insertions = {}
    for line in lines:
        line = line.split(" -> ")
        insertions[line[0]] = line[1]

    for _ in range(10):
        nextPairs = pairs.copy()
        for pair in pairs:
            newLetter = insertions[pair]
            if newLetter in occurrances:
                occurrances[newLetter] += pairs[pair]
            else:
                occurrances[newLetter] = pairs[pair]

            newPairs = [pair[0] + newLetter, newLetter + pair[1]]
            nextPairs[pair] -= pairs[pair]
            for nP in newPairs:
                if nP in nextPairs:
                    nextPairs[nP] += pairs[pair]
                else:
                    nextPairs[nP] = pairs[pair]

        pairs = nextPairs.copy()

    return max(occurrances[o] for o in occurrances) - min(occurrances[o] for o in occurrances)


def part2(data):
    """ 2021 Day 14 Part 2

    >>> part2(['NNCB', '', 'CH -> B', 'HH -> N', 'CB -> H', 'NH -> C', 'HB -> C', 'HC -> B', 'HN -> C', 'NN -> C', 'BH -> H', 'NC -> B', 'NB -> B', 'BN -> B', 'BB -> N', 'BC -> B', 'CC -> N', 'CN -> C'])
    2188189693529
    """

    template = data[0]
    templatePairs = [template[i:i+2] for i in range(0, len(template) - 1)]
    pairs = {}
    for pair in templatePairs:
        if pair in pairs:
            pairs[pair] += 1
        else:
            pairs[pair] = 1
    
    lines = data[2:]

    occurrances = {}
    for t in template:
        if t in occurrances:
            occurrances[t] += 1
        else:
            occurrances[t] = 1

    insertions = {}
    for line in lines:
        line = line.split(" -> ")
        insertions[line[0]] = line[1]

    for _ in range(40):
        nextPairs = pairs.copy()
        for pair in pairs:
            newLetter = insertions[pair]
            if newLetter in occurrances:
                occurrances[newLetter] += pairs[pair]
            else:
                occurrances[newLetter] = pairs[pair]

            newPairs = [pair[0] + newLetter, newLetter + pair[1]]
            nextPairs[pair] -= pairs[pair]
            for nP in newPairs:
                if nP in nextPairs:
                    nextPairs[nP] += pairs[pair]
                else:
                    nextPairs[nP] = pairs[pair]

        pairs = nextPairs.copy()

    return max(occurrances[o] for o in occurrances) - min(occurrances[o] for o in occurrances)


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
        print(f"\nPart 1:\nMost common - least common: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMost common - least common: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)