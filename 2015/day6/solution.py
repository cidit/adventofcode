
from enum import Enum


class Action(Enum):
    TOGGLE = 0
    TURN_ON = 1
    TURN_OFF = 2

class Instruction:
    action: Action
    start: tuple[int, int]
    end: tuple[int, int]
    def __repr__(self) -> str:
        return f"{self.action.name} {self.start} {self.end}"


def parse_coordinate(unparsed: str):
    return tuple(map(int, unparsed.split(',')))


def parse_instruction(unparsed: str):
    instruction = Instruction()
    rest = None
    if unparsed.startswith('toggle'):
        instruction.action = Action.TOGGLE
        rest = unparsed.removeprefix('toggle').strip()
    elif unparsed.startswith('turn on'):
        instruction.action = Action.TURN_ON
        rest = unparsed.removeprefix('turn on').strip()
    elif unparsed.startswith('turn off'):
        instruction.action = Action.TURN_OFF
        rest = unparsed.removeprefix('turn off').strip()
    (start, end) = rest.split(' through ')
    instruction.start = parse_coordinate(start)
    instruction.end = parse_coordinate(end)
    return instruction


def part1():
    print("\n~~~ Part 1 ~~~\n")
    # TODO: matrices
    with open("input.data", "r") as input_file:
        for line in input_file.readlines():
            instruction = parse_instruction(line)
            


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        pass


if __name__ == "__main__":
    part1()
    part2()
