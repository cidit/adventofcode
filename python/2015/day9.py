from pathlib import Path
from typing import Iterable, NamedTuple


class Travel(NamedTuple):
    locations: list[str]
    distance: int


def get_input():
    with open(Path(__file__).parent / "input.data", "r") as input_file:
        return [line.strip() for line in input_file.readlines()]


def parse_input(input: list[str]):
    tokens = (line.split() for line in input)
    return [
        Travel(locations=[token[0], token[2]], distance=int(token[4]))
        for token in tokens
    ]


def find_locations(travels: list[Travel]):
    return {
        location
        for locations in map(lambda it: it.locations, travels)
        for location in locations
    }


def find_best_choices(location: str, travels: Iterable[Travel]):
    choices = {}
    for travel in travels:
        (locations, distance) = travel
        if location in locations:
            other = locations[0] if locations[0] != location else locations[1]
            choices[distance] = other
    return sorted(choices, reverse=True)


def part1(input: list[str]):
    print("\n~~~ Part 1 ~~~\n")
    data = parse_input(input)
    locations = find_locations(data)
    for location in locations:
        print(find_best_choices(location, data))


def part2(input: list[str]):
    print("\n~~~ Part 2 ~~~\n")


if __name__ == "__main__":
    data = get_input()
    part1(input=data)
    part2(input=data)
