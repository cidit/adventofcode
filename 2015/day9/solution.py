from pathlib import Path
from typing import NamedTuple

class Travel(NamedTuple):
    locations: list[str]
    distance: int

def get_input():
    with open(Path(__file__).parent / "input.data", "r") as input_file:
        return [line.strip() for line in input_file.readlines()]

def parse_input(input: list[str]):
    tokens = (line.split() for line in input)
    return [Travel(locations=[token[0], token[2]], distance=int(token[4])) for token in tokens]

def part1(input: list[str]):
    print("\n~~~ Part 1 ~~~\n")
    data = parse_input(input)
    for d in data:
        print(d)
    

def part2(input: list[str]):
    print("\n~~~ Part 2 ~~~\n")


if __name__ == "__main__":
    data = get_input()
    part1(input=data)
    part2(input=data)
