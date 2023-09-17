
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

def make_int_matrix(n, m):
    matrix = [0] * n 
    for x in range(n):
        matrix[x] = [0] * m
    return matrix

def apply_instruction(lights_grid: list[list[int]], instruction: Instruction, brightness_control: bool = False):
    (x1, y1) = instruction.start
    (x2, y2) = instruction.end
    for x in range(x1, x2+1):
        for y in range(y1, y2+1):
            if instruction.action == Action.TOGGLE:
                if brightness_control:
                    lights_grid[x][y] += 2
                else:
                    lights_grid[x][y] = 1 if lights_grid[x][y] == 0 else 0
            elif instruction.action == Action.TURN_ON:
                if brightness_control:
                    lights_grid[x][y] += 1
                else: 
                    lights_grid[x][y] = 1
            elif instruction.action == Action.TURN_OFF:
                if brightness_control:
                    lights_grid[x][y] = lights_grid[x][y] - 1 if lights_grid[x][y] > 0 else 0
                else:
                    lights_grid[x][y] = 0

def count_lights_on(lights_grid: list[list[int]]):
    flatened_lights_grid = [light for lights in lights_grid for light in lights]
    counter = 0
    for light in flatened_lights_grid:
        if light == True:
            counter += 1
    return counter

def count_brightness(lights_grid: list[list[int]]):
    flatened_lights_grid = [light for lights in lights_grid for light in lights]
    total_brightness = 0
    for brightnesses in flatened_lights_grid:
        total_brightness += brightnesses
    return total_brightness


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        matrix = make_int_matrix(1000, 1000)
        for line in input_file.readlines():
            instruction = parse_instruction(line)
            apply_instruction(matrix, instruction, brightness_control=False)
        print(f"the number of lights on is {count_lights_on(matrix)}")
            


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        matrix = make_int_matrix(1000, 1000)
        for line in input_file.readlines():
            instruction = parse_instruction(line)
            apply_instruction(matrix, instruction, brightness_control=True)
        print(f"the number of lights on is {count_brightness(matrix)}")


if __name__ == "__main__":
    part1()
    part2()
