import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2021 Day 16 Part 1

    >>> part1(['8A004A801A8002F478'])
    16
    >>> part1(['620080001611562C8802118E34'])
    12
    >>> part1(['C0015000016115A2E0802F182340'])
    23
    >>> part1(['A0016C880162017C3686B18A3D4780'])
    31
    """

    packetBinary = bin(int(data[0], 16))[2:]
    packetBinary = '0' * ((8 - (len(packetBinary) % 8)) % 8) + packetBinary
    packet = Packet(packetBinary)
    packet.parse()

    return packet.version + versionSums(packet.subPackets)


def part2(data):
    """ 2021 Day 16 Part 2

    >>> part2(['C200B40A82'])
    3
    >>> part2(['04005AC33890'])
    54
    >>> part2(['880086C3E88112'])
    7
    >>> part2(['CE00C43D881120'])
    9
    >>> part2(['D8005AC2A8F0'])
    1
    >>> part2(['F600BC2D8F'])
    0
    >>> part2(['9C005AC2F8F0'])
    0
    >>> part2(['9C0141080250320F1802104A08'])
    1
    """

    packetBinary = bin(int(data[0], 16))[2:]
    packetBinary = '0' * ((8 - (len(packetBinary) % 8)) % 8) + packetBinary
    packet = Packet(packetBinary)
    packet.parse()

    return packet.eval()


class Packet:
    def __init__(self, packetString):
        self.binary = packetString
        self.parsed = False
        self.subPackets = []
        self.version = 0
        self.typeID = 0
        self.lengthType = 0
        self.value = 0

    def parse(self):
        if self.parsed:
            return

        self.parsed = True
        self.version = int(self.binary[:3], 2)
        self.typeID = int(self.binary[3:6], 2)

        if self.typeID == 4:
            # Literal Packet
            return self.parseLiteral(self.binary[6:])
        else:
            # Operator Packet
            self.subPackets = []
            self.lengthType = int(self.binary[6])

            if self.lengthType == 0:
                # Total Bits
                totalBits = int(self.binary[7:22], 2)
                remainder = self.binary[22:]

                while len(self.binary) - 22 - len(remainder) < totalBits:
                    subPacket = Packet(remainder)
                    self.subPackets.append(subPacket)
                    remainder = subPacket.parse()
                
                return remainder
            else:
                # Number of packets
                numPackets = int(self.binary[7:18], 2)
                remainder = self.binary[18:]
                
                for _ in range(numPackets):
                    subPacket = Packet(remainder)
                    self.subPackets.append(subPacket)
                    remainder = subPacket.parse()

                return remainder        

    def parseLiteral(self, literalString):
        i = 0
        number = ''
        while i < len(literalString):
            number += literalString[i + 1:i + 5]
            i += 5

            if literalString[i - 5] == '0':
                break            

        self.value = int(number, 2)
        return literalString[i:]

    def eval(self):
        if self.typeID == 0:
            # Sum Packet
            total = 0
            for sP in self.subPackets:
                total += sP.eval()

            return total
        if self.typeID == 1:
            # Product Packet
            product = 1
            for sP in self.subPackets:
                product *= sP.eval()

            return product
        if self.typeID == 2:
            # Minimum Packet
            minimum = float('inf')
            for sP in self.subPackets:
                num = sP.eval()
                if num < minimum:
                    minimum = num

            return minimum
        if self.typeID == 3:
            # Maximum Packet
            maximum = float('-inf')
            for sP in self.subPackets:
                num = sP.eval()
                if num > maximum:
                    maximum = num

            return maximum
        if self.typeID == 4:
            # Literal Packet
            return self.value
        if self.typeID == 5:
            # Greater Than Packet
            return int(self.subPackets[0].eval() > self.subPackets[1].eval())
        if self.typeID == 6:
            # Less Than Packet
            return int(self.subPackets[0].eval() < self.subPackets[1].eval())
        if self.typeID == 7:
            # Equal To Packet
            return int(self.subPackets[0].eval() == self.subPackets[1].eval())
        

def versionSums(packets):
    count = 0
    for p in packets:
        if not p.parsed:
            continue

        count += p.version
        count += versionSums(p.subPackets)
    
    return count


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
        print(f"\nPart 1:\nSum of Version Numbers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nEvaulation of Packet: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)