
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

def make_matrix(n, m):
    matrix = [False] * n 
    for x in range(n):
        matrix[x] = [False] * m
    return matrix

def apply_instruction(lights_grid: list[list[bool]], instruction: Instruction):
    (x1, y1) = instruction.start
    (x2, y2) = instruction.end
    for x in range(x1, x2+1):
        for y in range(y1, y2+1):
            if instruction.action == Action.TOGGLE:
                lights_grid[x][y] = not lights_grid[x][y]
            elif instruction.action == Action.TURN_ON:
                lights_grid[x][y] = True
            elif instruction.action == Action.TURN_OFF:
                lights_grid[x][y] = False

def count_lights_on(lights_grid: list[list[bool]]):
    flatened_lights_grid = [light for lights in lights_grid for light in lights]
    counter = 0
    for light in flatened_lights_grid:
        if light == True:
            counter += 1
    return counter



def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        matrix = make_matrix(1000, 1000)
        for line in input_file.readlines():
            instruction = parse_instruction(line)
            apply_instruction(matrix, instruction)
        print(f"the number of lights on is {count_lights_on(matrix)}")
            


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        pass


if __name__ == "__main__":
    part1()
    part2()
