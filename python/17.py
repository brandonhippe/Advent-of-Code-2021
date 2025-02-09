import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 17 Part 1

    >>> part1(['target area: x=20..30, y=-10..-5'])
    45
    """

    lines = data[0].split("target area: x=")
    lines = lines[1:]
    lines = lines[0].split(", y=")
    lines = [line.split("..") for line in lines]

    nums = []
    for line in lines:
        for l in line:
            nums.append(int(l))

    yMax = float('-inf')

    xVel = 0
    
    while triangle(xVel) < nums[0]:
        xVel += 1

    yVel = 0
    
    while yVel < 500:
        pos = [0, 0]
        vel = [xVel, yVel]
        maxY = 0

        while True:
            if pos[1] > maxY:
                maxY = pos[1]

            for (i, (p, v)) in enumerate(zip(pos, vel)):
                pos[i] = p + v
            
            if vel[0] != 0:
                vel[0] -= vel[0] // abs(vel[0])

            vel[1] -= 1

            if pos[0] > nums[1] or pos[1] < nums[2]:
                landed = False
                break

            if nums[0] <= pos[0] <= nums[1] and nums[2] <= pos[1] <= nums[3]:
                landed = True
                break

        if landed:
            if maxY > yMax:
                yMax = maxY

        yVel += 1

    return yMax


def part2(data):
    """ 2021 Day 17 Part 2

    >>> part2(['target area: x=20..30, y=-10..-5'])
    112
    """

    lines = data[0].split("target area: x=")
    lines = lines[1:]
    lines = lines[0].split(", y=")
    lines = [line.split("..") for line in lines]

    nums = []
    for line in lines:
        for l in line:
            nums.append(int(l))

    yMax = float('-inf')

    xVel = 0
    
    while triangle(xVel) < nums[0]:
        xVel += 1

    yVel = 0
    
    while yVel < 500:
        pos = [0, 0]
        vel = [xVel, yVel]
        maxY = 0

        while True:
            if pos[1] > maxY:
                maxY = pos[1]

            for (i, (p, v)) in enumerate(zip(pos, vel)):
                pos[i] = p + v
            
            if vel[0] != 0:
                vel[0] -= vel[0] // abs(vel[0])

            vel[1] -= 1

            if pos[0] > nums[1] or pos[1] < nums[2]:
                landed = False
                break

            if nums[0] <= pos[0] <= nums[1] and nums[2] <= pos[1] <= nums[3]:
                landed = True
                break

        if landed:
            if maxY > yMax:
                yMax = maxY

            yVelMax = yVel

        yVel += 1

    trajectories = []
    xVelMin = xVel

    for yVel in range(nums[2], yVelMax + 1):
        for xVel in range(xVelMin, nums[1] + 1):
            pos = [0, 0]
            vel = [xVel, yVel]
            maxY = 0

            while True:
                if pos[1] > maxY:
                    maxY = pos[1]

                for (i, (p, v)) in enumerate(zip(pos, vel)):
                    pos[i] = p + v
                
                if vel[0] != 0:
                    vel[0] -= vel[0] // abs(vel[0])

                vel[1] -= 1

                if pos[0] > nums[1] or pos[1] < nums[2]:
                    landed = False
                    break

                if nums[0] <= pos[0] <= nums[1] and nums[2] <= pos[1] <= nums[3]:
                    landed = True
                    break

            if landed:
                trajectories.append([xVel, yVel])

    return len(trajectories)


def triangle(n):
    return n * (n + 1) // 2


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
        print(f"\nPart 1:\nMaximum Y Position on Landing Trajectory: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of Landing Trajectories: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)