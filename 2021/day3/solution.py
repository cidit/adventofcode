def transform_input(untransformed: list[str]):
    return list(map(lambda it: it.strip(), untransformed))


def find_most_common(sample: iter):
    """ returns a set of items by order of frequency in the list array"""
    items_list = list(sample)
    item_counts = {}
    for value in items_list:
        item_counts[id(value)] = items_list.count(value)
    items_list.sort(key=lambda value: item_counts[id(value)], reverse=True)
    return list(dict.fromkeys(items_list))


def calc_gamma_epsilon(sample: list[str]):
    gamma_string = ""
    epsilon_string = ""
    shortest_length = min(map(lambda it: len(it), sample))
    for index in range(shortest_length):
        most_common = find_most_common(map(lambda it: it[index], sample))
        gamma_string += most_common[0]
        epsilon_string += most_common[1]

    return (int(gamma_string, base=2), int(epsilon_string, base=2))


def calc_o2_co2(sample: list[str]):
    pass    


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        (gamma, epsilon) = calc_gamma_epsilon(input_file.readlines())
        print(f"gamma is {gamma}, epsilon is {epsilon} and consumption rate is {gamma * epsilon}")


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        pass


if __name__ == "__main__":
    part1()
    part2()
