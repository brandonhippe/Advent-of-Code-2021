import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 4 Part 1

    >>> part1(['7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1', '', '22 13 17 11  0', ' 8  2 23  4 24', '21  9 14 16  7', ' 6 10  3 18  5', ' 1 12 20 15 19', '', ' 3 15  0  2 22', ' 9 18 13 17  5', '19  8  7 25 23', '20 11 10 24  4', '14 21 16 12  6', '', '14 21 17 24  4', '10 16 15  9 19', '18  8 23 26 20', '22 11 13  6  5', ' 2  0 12  3  7'])
    4512
    """

    calls = data[0].split(',')

    for (i, num) in enumerate(calls):
        calls[i] = int(num)

    boards = []
    values = []
    for line in data[2:]:
        if len(line) == 0:
            boards.append(bingoBoard(values))
            values = []
        else:
            temp = line.split(' ')
            for i in range(len(temp) - 1, -1, -1):
                if len(temp[i]) == 0:
                    temp.pop(i)
                else:
                    temp[i] = int(temp[i])

            values.append(temp)

    if len(values):
        boards.append(bingoBoard(values))

    for num in calls:
        for board in boards:
            if board.mark(num):
                return board.score(num)

    return -1


def part2(data):
    """ 2021 Day 4 Part 2

    >>> part2(['7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1', '', '22 13 17 11  0', ' 8  2 23  4 24', '21  9 14 16  7', ' 6 10  3 18  5', ' 1 12 20 15 19', '', ' 3 15  0  2 22', ' 9 18 13 17  5', '19  8  7 25 23', '20 11 10 24  4', '14 21 16 12  6', '', '14 21 17 24  4', '10 16 15  9 19', '18  8 23 26 20', '22 11 13  6  5', ' 2  0 12  3  7'])
    1924
    """

    calls = data[0].split(',')

    for (i, num) in enumerate(calls):
        calls[i] = int(num)

    boards = []
    values = []
    for line in data[2:]:
        if len(line) == 0:
            boards.append(bingoBoard(values))
            values = []
        else:
            temp = line.split(' ')
            for i in range(len(temp) - 1, -1, -1):
                if len(temp[i]) == 0:
                    temp.pop(i)
                else:
                    temp[i] = int(temp[i])

            values.append(temp)

    if len(values):
        boards.append(bingoBoard(values))

    lastScore = -1
    for num in calls:
        for i in range(len(boards) - 1, -1, -1):
            won = boards[i].mark(num)
            if won == 1:
                if len(boards) == 1:
                    lastScore = boards[i].score(num)
                
                boards.pop(i)
        
        if len(boards) == 0:
            break

    return lastScore


class bingoBoard:
    def __init__(self, values):
        self.board = values[:]
        template = [0] * 5
        self.checked = []
        for _ in range (5):
            self.checked.append(template[:])

    def mark(self, call):
        for (i, row) in enumerate(self.board):
            for (j, val) in enumerate(row):
                if val == call:
                    self.checked[i][j] += 1

        for i in range(len(self.board)):
            counts = [0] * 2
            for j in range(len(self.board)):
                if self.checked[i][j] > 0:
                    counts[0] += 1
                if self.checked[j][i] > 0:
                    counts[1] += 1

            if 5 in counts:
                return 1

        return 0

    def score(self, call):
        count = 0
        for (i, row) in enumerate(self.checked):
            for (j, val) in enumerate(row):
                if val == 0:
                    count += self.board[i][j] * call

        return count

    def printBoard(self):
        for row in self.board:
            print(row)


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
        print(f"\nPart 1:\nWinning board score: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLast board score: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)