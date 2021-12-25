def parse_input(unparsed: list[str]):
    return list(map(int, unparsed))


def make_windows(sample: list[int], window_size=3):
    windows = []
    index_of_last_window = len(sample) - (window_size - 1)
    for i in range(index_of_last_window):
        window = [sample[i + j] for j in range(window_size)]
        windows.append(window)
    return windows


def sum_windows(windows: list[list[int]]):
    return list(map(sum, windows))


def count_increases(sample: list[int]):
    increases = 0
    last_mesurement = -1
    for mesurement in sample:
        if last_mesurement != -1:
            if mesurement > last_mesurement:
                increases += 1
        last_mesurement = mesurement
    return increases


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        increases = count_increases(sample=parse_input(input_file.readlines()))
        print(
            f"the number of mesurements bigger than the previous one is {increases}"
        )


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        windows = make_windows(parse_input(input_file.readlines()))
        summed_windows = sum_windows(windows)
        increases = count_increases(summed_windows)
        print(
            f"the number of summed windows bigger than the previous one is {increases}"
        )


if __name__ == "__main__":
    part1()
    part2()
