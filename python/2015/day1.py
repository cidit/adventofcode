
def count_floors(input: str):
    floor = 0 
    for character in input:
        if character == '(':
            floor += 1
        elif character == ')':
            floor -=1
    return floor

def find_basement(input: str):
    floor = 0
    instruction = 0
    for character in input:
        instruction +=1
        if character == '(':
            floor += 1
        elif character == ')':
            floor -=1
        if floor == -1:
            break
    return instruction


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        print(count_floors(input_file.read()))


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        print(find_basement(input_file.read()))


if __name__ == "__main__":
    part1()
    part2()