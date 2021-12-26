class UnlinkedNode():
    def __init__(self, name: str, action: str, dependents: list[str]) -> None:
        self.dependents = dependents
        self.action = action
        self.name = name


def parse_node(unparsed: str) -> UnlinkedNode:
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
