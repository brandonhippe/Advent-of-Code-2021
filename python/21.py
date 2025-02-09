import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import Counter


def part1(data):
    """ 2021 Day 21 Part 1

    >>> part1(['Player 1 starting position: 4', 'Player 2 starting position: 8'])
    739785
    """

    players = [0] * 2
    players[0] = int(data[0].split(" starting position: ")[1])
    players[1] = int(data[1].split(" starting position: ")[1])

    die = 1
    rolls = 0
    playersScores = [0] * 2
    playerIndex = 0

    while playersScores[0] < 1000 and playersScores[1] < 1000:
        for _ in range(3):
            players[playerIndex] += die
            players[playerIndex] = players[playerIndex] % 10
            if players[playerIndex] == 0:
                players[playerIndex] = 10

            die += 1
            rolls += 1
            if die > 100:
                die = die % 100

        playersScores[playerIndex] += players[playerIndex]
        
        playerIndex += 1
        playerIndex = playerIndex % 2
        
    return rolls * playersScores[playerIndex]


def part2(data):
    """ 2021 Day 21 Part 2

    >>> part2(['Player 1 starting position: 4', 'Player 2 starting position: 8'])
    444356092776315
    """

    states = {}
    states[stateString([int(data[0].split(" starting position: ")[1]), int(data[1].split(" starting position: ")[1]), 0, 0])] = 1
    playerWins = [0] * 2

    dice = list(Counter(
        i + j + k
        for i in range(1, 4)
        for j in range(1, 4)
        for k in range(1, 4)
    ).items())

    while len(states) != 0:
        newStates = {}
        for state in states:
            count = states[state]
            for (die, dieCount) in dice:
                stateArr = [int(x) for x in state.split(',')]
                stateArr[0] += die
                stateArr[0] = stateArr[0] % 10
                if stateArr[0] == 0:
                    stateArr[0] = 10

                stateArr[2] += stateArr[0]
                if stateArr[2] >= 21:
                    playerWins[0] += count * dieCount
                    continue
                
                tempState = stateArr[:]
                for (die2, die2Count) in dice:
                    stateArr = tempState[:]
                    stateArr[1] += die2
                    stateArr[1] = stateArr[1] % 10
                    if stateArr[1] == 0:
                        stateArr[1] = 10

                    stateArr[3] += stateArr[1]
                    if stateArr[3] >= 21:
                        playerWins[1] += count * dieCount * die2Count
                        continue

                    newState = stateString(stateArr)
                    if newState in newStates:
                        newStates[newState] += count * dieCount * die2Count
                    else:
                        newStates[newState] = count * dieCount * die2Count

        states = newStates

    return max(playerWins)


def stateString(stateArr):
    string = str(stateArr[0])
    for e in stateArr[1:]:
        string += ',' + str(e)

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
        print(f"\nPart 1:\nProduct of die rolls and losing score: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nThe most universes won in is: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)