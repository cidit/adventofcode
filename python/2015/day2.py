def parse(unparsed: str):
    return [int(splited) for splited in unparsed.split('x')]

def mesure_wrapping(l, w, h):
    sizes = (l * w, w * h, h * l)
    return 2 * sum(sizes) + min(sizes)


def mesure_ribbon(l, w, h):
    shortest = min([l + w, w + h, h + l])
    return 2 * shortest + (l * w * h)


def part1():
    print("\n~~~ Part 1 ~~~\n")

    totalArea = 0
    with open("input.data", "r") as input_file:
        for line in input_file.readlines():
            totalArea += mesure_wrapping(*parse(line))

    print(f"total area of wrapping paper needed: {totalArea}")


def part2():
    print("\n~~~ Part 2 ~~~\n")

    totalLength = 0
    with open("input.data", "r") as input_file:
        for line in input_file.readlines():
            totalLength += mesure_ribbon(*parse(line))

    print(f"total length of ribbon needed: {totalLength}")


if __name__ == "__main__":
    part1()
    part2()