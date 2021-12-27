
"""
map = {key: hash(name), node: {name: str, action:str, deps: list[str]}}
"""
from typing import NamedTuple


def get_input():
    with open("input.data", "r") as input_file:
        return [line.strip() for line in input_file.readlines()]


class Node(NamedTuple):
    name: str
    what: str
    deps: list[str] = []


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

    tokens = unparsed.split()
    acions_with_two_dependents = "RSHIFT LSHIFT AND OR".split()
    node = Node()
    node.name = tokens.pop()
    if any(action in tokens for action in acions_with_two_dependents):
        node.deps.append(tokens[0])
        node.what = tokens[1]
        node.deps.append(tokens[2])
    elif "NOT" in tokens:
        node.what = "NOT"
        node.deps.append(tokens[1])
    else:
        node.what = "NO-ACTION"
        node.deps.append = tokens[0]
    return node



def part1(input: list[str]):
    print("\n~~~ Part 1 ~~~\n")


def part2(input: list[str]):
    print("\n~~~ Part 2 ~~~\n")


if __name__ == "__main__":
    data = get_input()
    part1(input=data)
    part2(input=data)
