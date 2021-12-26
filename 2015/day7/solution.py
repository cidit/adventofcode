


"""
map = {key: hash(name), node: {name: str, action:str, deps: list[str]}}
"""

class Node:
    def __init__(self, name: str, action: str, deps: list[str]) -> None:
        self.deps = deps
        self.action = action
        self.name = name


def parse_node(unparsed: str) -> Node:
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
    pass


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        pass


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        pass


if __name__ == "__main__":
    part1()
    part2()
