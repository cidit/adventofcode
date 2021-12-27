"""
map = {key: hash(name), node: {name: str, action:str, deps: list[str]}}
"""
from pathlib import Path
from typing import NamedTuple


class Node(NamedTuple):
    name: str = ""
    what: str = ""
    deps: list[str] = []


def get_input():
    with open(Path(__file__).parent / "input.data", "r") as input_file:
        return [line.strip() for line in input_file.readlines()]


def parse_node(unparsed: str):
    """
    dependents need to be resolved
    dependents can resolve to a scalar or another dependent
    there can be at most 2 dependents
    scalars can only be numbers
    if a dependent is a numeric string then it is a scalar
    else, if it is alphabetic it resolves to another dependent

    there are these operations:
    - RSHIFT: dependent [ACTION] scalar -> output
    - LSHIFT: dependent [ACTION] scalar -> output
    - AND: dependent [ACTION] dependent -> output
    - OR: dependent [ACTION] dependent -> output
    - NOT: [ACTION] dependent -> output
    - [NO ACTION]: dependent -> output
    """

    # TODO: refactor this maybe
    tokens = unparsed.split()
    acions_with_two_dependents = "RSHIFT LSHIFT AND OR".split()
    name = tokens.pop()
    tokens.remove("->")
    what = ""
    deps = []
    if any(action in tokens for action in acions_with_two_dependents):
        deps.append(tokens[0])
        what = tokens[1]
        deps.append(tokens[2])
    elif "NOT" in tokens:
        what = "NOT"
        deps.append(tokens[1])
    else:
        what = "NO-ACTION"
        deps.append(tokens[0])

    return Node(name=name, what=what, deps=deps)


def parse_input(input: list[str]):
    parsed_nodes = map(parse_node, input)
    return {node.name: node for node in parsed_nodes}


def resolve_node(name: str, nodes: dict[str, Node]):
    node = dict[name]
    # TODO: implement this


def part1(input: list[str]):
    print("\n~~~ Part 1 ~~~\n")


def part2(input: list[str]):
    print("\n~~~ Part 2 ~~~\n")


if __name__ == "__main__":
    data = get_input()
    part1(input=data)
    part2(input=data)
