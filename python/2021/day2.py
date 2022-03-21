


def parse_instruction(unparsed: str):
    (instruction, value) = tuple(unparsed.split(' '))
    return (instruction, int(value))

def parse_input(unparsed: list[str]):
    return list(map(parse_instruction, unparsed))

def calculate_position(instructions: list[tuple[str, int]], include_aim=False):
    depth = 0
    horizontal_pos = 0
    aim = 0
    for instruction, value in instructions:
        if instruction == "forward":
            if include_aim:
                horizontal_pos += value
                depth += aim * value
            else:
                horizontal_pos += value
        elif instruction == "down":
            if include_aim:
                aim += value
            else:
                depth += value
        elif instruction == "up":
            if include_aim:
                aim -= value
            else:
                depth -= value
    return (depth, horizontal_pos)

def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        (depth, horizontal_pos) = calculate_position(instructions=parse_input(input_file.readlines()))
        print(f"the depth is {depth}, the horizontal position is {horizontal_pos} and their product is {depth*horizontal_pos}")
        


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        (depth, horizontal_pos) = calculate_position(instructions=parse_input(input_file.readlines()), include_aim=True)
        print(f"the depth is {depth}, the horizontal position is {horizontal_pos} and their product is {depth*horizontal_pos}")


if __name__ == "__main__":
    part1()
    part2()
