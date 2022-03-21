def track_houses(input_sequence):
    houses = {}
    x = y = 0
    houses[f"{x}:{y}"] = 1
    for direction in input_sequence:
        if direction == '^': y += 1
        elif direction == '>': x += 1
        elif direction == 'v': y -= 1
        elif direction == '<': x -= 1

        coord = f"{x}:{y}"
        if coord in houses:
            houses[coord] += 1
        else:
            houses[coord] = 1
    return houses


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        houses = track_houses(input_file.read())
        print(f"the number of houses with at least one gift is: {len(houses)}")


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        complete_sequence = input_file.read()
        sequence1 = []
        sequence2 = []
        for index, instruction in enumerate(complete_sequence):
            if index % 2 == 0: sequence1.append(instruction)
            else: sequence2.append(instruction)
        number_of_houses = len(track_houses(sequence1) | track_houses(sequence2))
        print(f"the number of houses with at least one gift is: {number_of_houses}")


if __name__ == "__main__":
    part1()
    part2()