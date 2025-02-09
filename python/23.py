import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import math
import heapq


def part1(data):
    """ 2021 Day 23 Part 1

    >>> part1(['#############', '#...........#', '###B#C#B#D###', '  #A#D#C#A#', '  #########'])
    12521
    """

    lines = data[:]

    for i in range(len(lines)):
        lines[i] += ' ' * (len(lines[0]) - len(lines[i]))

    rooms = lines[1][1:-1]

    for i in lines[2:-1]:
        rooms += i[3:-3:2]

    return aStar(rooms)


def part2(data):
    """ 2021 Day 23 Part 2

    >>> part2(['#############', '#...........#', '###B#C#B#D###', '  #A#D#C#A#', '  #########'])
    44169
    """

    lines = data[:]

    for i in range(len(lines)):
        lines[i] += ' ' * (len(lines[0]) - len(lines[i]))

    rooms = lines[1][1:-1]

    for i in lines[2:-1]:
        rooms += i[3:-3:2]

    rooms = rooms[:15] + 'DCBADBAC' + rooms[15:]

    return aStar(rooms)


def getEnergy(s_, indexes):
    state = list(s_)
    energy = 0
    start, end = indexes

    for (i, s) in enumerate(state):
        if s == ".":
            state[i] = 0
        else:
            state[i] = 10 ** (ord(s) - ord('A'))

    if state[start] == 0:
        # Start isn't occupied
        return [0, 0]

    val = state[start]

    if state[end] != 0:
        # End is occupied
        return [0, 0]

    if end % 2 == 0 and 2 <= end <= 8:
        # End is outside a hallway
        return [0, 0]

    if end < 11:
        # End is in the hallway
        if start < 11:
            # Start is also in the hallway
            return [0, 0]
    else:
        # End is in a room
        room = (end - 11) % 4
        if val != 10 ** room:
            # Value shouldn't go into the room
            return [0, 0]

        bottom = -1
        for (i, spot) in enumerate(state[11 + room::4]):
            if spot != 0:
                if spot != val:
                    # End room is not fillable
                    return [0, 0]
            else:
                bottom = i

        if bottom >= 0 and bottom != (end - 11) // 4:
            # End isn't at bottom of room
            return [0, 0]

        if start >= 11:
            # Start is also in a room
            if (start - 11) % 4 == (end - 11) % 4:
                # Start and End are in the same room
                return [0, 0]

            room = (start - 11) % 4
            if val == 10 ** room:
                fillable = True
                for spot in state[11 + room::4]:
                    if spot != 0 and spot != val:
                        fillable = False

                if fillable:
                    # Start room is fillable. Don't remove from room
                    return [0, 0]

    sStart = start
    while start != end:
        energy += val
        if start != sStart and state[start] != 0:
            # Something in the way
            return [0, 0]

        if start < 11:
            # Start is in hallway
            if end < 11:
                # End is also in hallway
                goalIndex = end
            else:
                # End is in a room
                goalIndex = 2 + 2 * ((end - 11) % 4)

                if start == goalIndex:
                    # Enter the room
                    start = 11 + (end - 11) % 4
                    continue

            if start < goalIndex:
                start += 1
            else:
                start -= 1
        else:
            # Start is in room
            if end >= 11 and (start - 11) % 4 == (end - 11) % 4:
                # Start and end are in the same room
                start += 4
            else:
                level = (start - 11) // 4
                if level > 0:
                    # Staying in same room
                    start -= 4
                else:
                    # Moving into hallway
                    start = 2 + 2 * ((start - 11) % 4)

    state[sStart] = 0
    state[end] = val

    stateStr = ""
    for e in state:
        c = "."
        if e > 0:
            c = chr(int(math.log10(e)) + ord('A'))

        stateStr += c

    return [stateStr, energy]


def getNext(state):
    posIndexes = [i for i in range(2)]
    for i in range(3, 9, 2):
        posIndexes.append(i)

    for i in range(9, len(state)):
        posIndexes.append(i)

    nextStates = []
    for p1 in posIndexes:
        for p2 in posIndexes:
            if p1 == p2:
                continue

            energyCost = getEnergy(state, [p1, p2])
            if energyCost[1] > 0:
                nextStates.append(energyCost)

    return nextStates


def fSort(e):
    return e[1]


def heuristic(state):
    energy = 0
    counts = [(len(state) - 10) // 4] * 4
    move = [True] * len(state)
    for (i, s) in enumerate(state[-1:10:-1]):
        level = ((len(state) - 11) // 4) - (i // 4)
        room = 3 - (i % 4)

        if ord(s) - ord('A') == room and level == counts[room]:
            move[11 + 4 * (level - 1) + room] = False
            counts[room] -= 1

    for (i, s) in enumerate(state):
        if s == "." or not move[i]:
            continue

        goalRoom = ord(s) - ord('A')
        goalIndex = 2 * (goalRoom + 1)
        val = 10 ** goalRoom

        if i < 11:
            # In hallway
            energy += val * (abs(i - goalIndex) + counts[goalRoom])
        else:
            # In a room
            level = (i - 11) // 4 + 1
            room = (i - 11) % 4

            if room == goalRoom:
                # In correct room, move out of Room first, then back in
                energy += val * (level + 2 + counts[goalRoom])
            else:
                # In wrong room
                hallIndex = 2 * (room + 1)
                energy += val * (level + abs(hallIndex - goalIndex) + counts[goalRoom])

        counts[goalRoom] -= 1

    return energy


def aStar(start):
    end = '.' * 11 + 'ABCD' * ((len(start) - 11) // 4)

    openList_heap = [[heuristic(start), 0, start]]
    closedList = {}
    heuristics = {}

    heuristics[start] = heuristic(start)

    heapq.heapify(openList_heap)
    while len(openList_heap) != 0:
        qF, qG, q = heapq.heappop(openList_heap)  
        
        if q == end:
            return qG

        nextStates = getNext(q)

        for n in nextStates:
            state, nG = n
            nG += qG

            try:
                nH = heuristics[state]
            except:
                nH = heuristic(state)
                heuristics[state] = nH

            nF = nG + nH

            found = False
            for item in openList_heap:
                if item[2] == state and item[0] <= nF:
                    found = True
                    break

            if found:
                continue

            if state in closedList and closedList[state][0] <= nF:
                continue

            heapq.heappush(openList_heap, [nF, nG, state])

        closedList[q] = [qF, qG, q]


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
        print(f"\nPart 1:\nThe lowest possible energy required is: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nThe lowest possible energy required is: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)