import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import numpy as np
from functools import cache
from itertools import product
from collections import defaultdict


def part1(data):
    """ 2021 Day 19 Part 1

    >>> part1(['--- scanner 0 ---', '404,-588,-901', '528,-643,409', '-838,591,734', '390,-675,-793', '-537,-823,-458', '-485,-357,347', '-345,-311,381', '-661,-816,-575', '-876,649,763', '-618,-824,-621', '553,345,-567', '474,580,667', '-447,-329,318', '-584,868,-557', '544,-627,-890', '564,392,-477', '455,729,728', '-892,524,684', '-689,845,-530', '423,-701,434', '7,-33,-71', '630,319,-379', '443,580,662', '-789,900,-551', '459,-707,401', '', '--- scanner 1 ---', '686,422,578', '605,423,415', '515,917,-361', '-336,658,858', '95,138,22', '-476,619,847', '-340,-569,-846', '567,-361,727', '-460,603,-452', '669,-402,600', '729,430,532', '-500,-761,534', '-322,571,750', '-466,-666,-811', '-429,-592,574', '-355,545,-477', '703,-491,-529', '-328,-685,520', '413,935,-424', '-391,539,-444', '586,-435,557', '-364,-763,-893', '807,-499,-711', '755,-354,-619', '553,889,-390', '', '--- scanner 2 ---', '649,640,665', '682,-795,504', '-784,533,-524', '-644,584,-595', '-588,-843,648', '-30,6,44', '-674,560,763', '500,723,-460', '609,671,-379', '-555,-800,653', '-675,-892,-343', '697,-426,-610', '578,704,681', '493,664,-388', '-671,-858,530', '-667,343,800', '571,-461,-707', '-138,-166,112', '-889,563,-600', '646,-828,498', '640,759,510', '-630,509,768', '-681,-892,-333', '673,-379,-804', '-742,-814,-386', '577,-820,562', '', '--- scanner 3 ---', '-589,542,597', '605,-692,669', '-500,565,-823', '-660,373,557', '-458,-679,-417', '-488,449,543', '-626,468,-788', '338,-750,-386', '528,-832,-391', '562,-778,733', '-938,-730,414', '543,643,-506', '-524,371,-870', '407,773,750', '-104,29,83', '378,-903,-323', '-778,-728,485', '426,699,580', '-438,-605,-362', '-469,-447,-387', '509,732,623', '647,635,-688', '-868,-804,481', '614,-800,639', '595,780,-596', '', '--- scanner 4 ---', '727,592,562', '-293,-554,779', '441,611,-461', '-714,465,-776', '-743,427,-804', '-660,-479,-426', '832,-632,460', '927,-485,-438', '408,393,-506', '466,436,-512', '110,16,151', '-258,-428,682', '-393,719,612', '-211,-452,876', '808,-476,-593', '-575,615,604', '-485,667,467', '-680,325,-822', '-627,-443,-432', '872,-547,-609', '833,512,582', '807,604,487', '839,-516,451', '891,-625,532', '-652,-548,-490', '30,-46,-14'])
    79
    """

    return len(relativeBeacons(tuple(data))[0])


def part2(data):
    """ 2021 Day 19 Part 2

    >>> part2(['--- scanner 0 ---', '404,-588,-901', '528,-643,409', '-838,591,734', '390,-675,-793', '-537,-823,-458', '-485,-357,347', '-345,-311,381', '-661,-816,-575', '-876,649,763', '-618,-824,-621', '553,345,-567', '474,580,667', '-447,-329,318', '-584,868,-557', '544,-627,-890', '564,392,-477', '455,729,728', '-892,524,684', '-689,845,-530', '423,-701,434', '7,-33,-71', '630,319,-379', '443,580,662', '-789,900,-551', '459,-707,401', '', '--- scanner 1 ---', '686,422,578', '605,423,415', '515,917,-361', '-336,658,858', '95,138,22', '-476,619,847', '-340,-569,-846', '567,-361,727', '-460,603,-452', '669,-402,600', '729,430,532', '-500,-761,534', '-322,571,750', '-466,-666,-811', '-429,-592,574', '-355,545,-477', '703,-491,-529', '-328,-685,520', '413,935,-424', '-391,539,-444', '586,-435,557', '-364,-763,-893', '807,-499,-711', '755,-354,-619', '553,889,-390', '', '--- scanner 2 ---', '649,640,665', '682,-795,504', '-784,533,-524', '-644,584,-595', '-588,-843,648', '-30,6,44', '-674,560,763', '500,723,-460', '609,671,-379', '-555,-800,653', '-675,-892,-343', '697,-426,-610', '578,704,681', '493,664,-388', '-671,-858,530', '-667,343,800', '571,-461,-707', '-138,-166,112', '-889,563,-600', '646,-828,498', '640,759,510', '-630,509,768', '-681,-892,-333', '673,-379,-804', '-742,-814,-386', '577,-820,562', '', '--- scanner 3 ---', '-589,542,597', '605,-692,669', '-500,565,-823', '-660,373,557', '-458,-679,-417', '-488,449,543', '-626,468,-788', '338,-750,-386', '528,-832,-391', '562,-778,733', '-938,-730,414', '543,643,-506', '-524,371,-870', '407,773,750', '-104,29,83', '378,-903,-323', '-778,-728,485', '426,699,580', '-438,-605,-362', '-469,-447,-387', '509,732,623', '647,635,-688', '-868,-804,481', '614,-800,639', '595,780,-596', '', '--- scanner 4 ---', '727,592,562', '-293,-554,779', '441,611,-461', '-714,465,-776', '-743,427,-804', '-660,-479,-426', '832,-632,460', '927,-485,-438', '408,393,-506', '466,436,-512', '110,16,151', '-258,-428,682', '-393,719,612', '-211,-452,876', '808,-476,-593', '-575,615,604', '-485,667,467', '-680,325,-822', '-627,-443,-432', '872,-547,-609', '833,512,582', '807,604,487', '839,-516,451', '891,-625,532', '-652,-548,-490', '30,-46,-14'])
    3621
    """
    
    scanners = relativeBeacons(tuple(data))[1]

    dist = 0
    for s1, s2 in product(scanners, repeat = 2):
        dist = max(dist, manhatDist(s1, s2))

    return dist


@cache
def relativeBeacons(data):
    scanners = []
    scannerDists = []
    betweenBeacons = []
    for line in data:
        if 'scanner' in line:
            scanners.append([])
            scannerDists.append({})
            betweenBeacons.append(defaultdict(set))
        elif len(line) != 0:
            pos = tuple(int(n) for n in re.findall('-?\d+', line))

            for other in scanners[-1]:
                d = pythagDist(pos, other)
                scannerDists[-1][d] = (pos, other)
                betweenBeacons[-1][pos].add(d)
                betweenBeacons[-1][other].add(d)

            scanners[-1].append(pos)

    scannerPos = [tuple([0] * len(scanners[0][0]))] + [None] * (len(scanners) - 1)
    while None in scannerPos:
        for i in range(len(scannerDists)):
            if scannerPos[i] is None:
                continue

            for j in range(len(scannerDists)):
                if j == i or scannerPos[j] is not None:
                    continue

                sameDists = set(scannerDists[i].keys()) & set(scannerDists[j].keys())

                absRel = defaultdict(set)
                for (absB, absSet), (relB, relSet) in product(betweenBeacons[i].items(), betweenBeacons[j].items()):
                    if len(sameDists & absSet & relSet) > 1:
                        absRel[absB].add(relB)
                
                if len(absRel) >= 2:
                    rot, absPos = findRot_abs(absRel)
                        
                    ### Apply transformation
                    newScannerDists = {}
                    newBetweenBeacons = defaultdict(set)
                    newScanners = []

                    for d, (b1, b2) in scannerDists[j].items():
                        b1 = tuple(np.matmul(rot, np.array(b1)))
                        b2 = tuple(np.matmul(rot, np.array(b2)))

                        b1 = tuple(p + o for p, o in zip(absPos, b1))
                        b2 = tuple(p + o for p, o in zip(absPos, b2))

                        newScannerDists[d] = (b1, b2)
                        newBetweenBeacons[b1].add(d)
                        newBetweenBeacons[b2].add(d)

                    for b in scanners[j]:
                        b = tuple(np.matmul(rot, np.array(b)))
                        b = tuple(p + o for p, o in zip(absPos, b))
                        newScanners.append(b)

                    ### Assign Absolute Positions
                    scanners[j] = newScanners
                    scannerDists[j] = newScannerDists
                    betweenBeacons[j] = newBetweenBeacons
                    scannerPos[j] = absPos

    allBeacons = set()
    for beacons in scanners:
        allBeacons |= set(beacons)

    return allBeacons, scannerPos


def findRot_abs(absRel):
    (abs1, rel1), (abs2, rel2) = list(absRel.items())[:2]
    rel1 = list(rel1)[0]
    rel2 = list(rel2)[0]

    rots = [np.array([
                np.array([1, 0, 0]),
                np.array([0, 1, 0]),
                np.array([0, 0, 1])
            ]),
            np.array([
                np.array([0, 0, 1]),
                np.array([0,	1, 0]),
                np.array([-1, 0,	0])
            ]),
            np.array([
                np.array([-1, 0,	0]),
                np.array([0,	1, 0]),
                np.array([0,	0, -1])
            ]),
            np.array([
                np.array([0,	0, -1]),
                np.array([0,	1, 0]),
                np.array([1,	0, 0]),
            ]),
            np.array([
                np.array([0,	-1,	0]),
                np.array([1,	0, 0]),
                np.array([0,	0, 1])
            ]),
            np.array([
                np.array([0,	0, 1]),
                np.array([1,	0, 0]),
                np.array([0,	1, 0])
            ]),
            np.array([
                np.array([0,	1, 0]),
                np.array([1,	0, 0]),
                np.array([0,	0, -1])
            ]),
            np.array([
                np.array([0,	0, -1]),
                np.array([1,	0, 0]),
                np.array([0, -1, 0])
            ]),
            np.array([
                np.array([0,	1, 0]),
                np.array([-1, 0,	0]),
                np.array([0,	0, 1])
            ]),
            np.array([
                np.array([0,	0, 1]),
                np.array([-1, 0,	0]),
                np.array([0,	-1, 0])
            ]),
            np.array([
                np.array([0,	-1,	0]),
                np.array([-1, 0,	0]),
                np.array([0, 0, -1])
            ]),
            np.array([
                np.array([0,	0, -1]),
                np.array([-1, 0,	0]),
                np.array([0,	1, 0])
            ]),
            np.array([
                np.array([1,	0, 0]),
                np.array([0,	0, -1]),
                np.array([0,	1, 0])
            ]),
            np.array([
                np.array([0,	1, 0]),
                np.array([0,	0, -1]),
                np.array([-1, 0,	0])   
            ]),
            np.array([
                np.array([-1, 0,	0]),
                np.array([0,	0, -1]),
                np.array([0,	-1,	0])
            ]),
            np.array([
                np.array([0,	-1,	0]),
                np.array([0,	0, -1]),
                np.array([1,	0, 0])
            ]),
            np.array([
                np.array([1,	0, 0]),
                np.array([0,	-1,	0]),
                np.array([0, 0, -1])
            ]),
            np.array([
                np.array([0,	0, -1]),
                np.array([0,	-1,	0]),
                np.array([-1, 0,	0])
            ]),
            np.array([
                np.array([-1, 0,	0]),
                np.array([0,	-1,	0]),
                np.array([0,	0, 1])
            ]),
            np.array([
                np.array([0,	0, 1]),
                np.array([0,	-1,	0]),
                np.array([1,	0, 0])
            ]),
            np.array([
                np.array([1,	0, 0]),
                np.array([0,	0, 1]),
                np.array([0,	-1,	0])
            ]),
            np.array([
                np.array([0,	-1, 0]),
                np.array([0,	0, 1]),
                np.array([-1, 0,	0])
            ]),
            np.array([
                np.array([-1, 0,	0]),
                np.array([0,	0, 1]),
                np.array([0,	1, 0])
            ]),
            np.array([
                np.array([0,	1, 0]),
                np.array([0,	0, 1]),
                np.array([1,	0, 0])
            ])]

    for rot in rots:
        r1 = tuple(np.matmul(rot, np.array(rel1)))
        r2 = tuple(np.matmul(rot, np.array(rel2)))

        a1 = tuple(p - o for p, o in zip(abs1, r1))
        a2 = tuple(p - o for p, o in zip(abs2, r2))

        if a1 == a2:
            return [rot, a1]


def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


def pythagDist(p1, p2):
    return sum(abs(c1 - c2) ** 2 for c1, c2 in zip(p1, p2))


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
        print(f"\nPart 1:\nNumber of Beacons: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFarthest Apart Scanners: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)