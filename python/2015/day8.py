from codecs import decode
from pathlib import Path 


def get_input():
    with open(Path(__file__).parent / "input.data", "r") as input_file:
        return [line.strip() for line in input_file.readlines()]


def part1(input: list[str]):
    print("\n~~~ Part 1 ~~~\n")
    total_num_char_in_literals = 0
    total_num_char_in_memory = 0
    for line in input:
        decoded_line = decode(line, 'unicode_escape')[1:-1]
        total_num_char_in_literals += len(line)
        total_num_char_in_memory += len(decoded_line) 
    print(f"total number of characters in literals: {total_num_char_in_literals} and in memory: {total_num_char_in_memory}")
    print(f"and their difference is: {total_num_char_in_literals - total_num_char_in_memory}")


def part2(input: list[str]):
    print("\n~~~ Part 2 ~~~\n")
    escape_characters = { '\\', '"'}

    def double_encode_line(line: str):
        chars = (char for char in line)
        escaped = ["".join(['\\', char]) if char in escape_characters else char for char in chars]
        return f'"{"".join(escaped)}"'

    total_num_char_in_literals = 0
    total_num_char_in_double_encoded= 0
    for line in input:
        double_encoded = double_encode_line(line)
        total_num_char_in_literals += len(line)
        total_num_char_in_double_encoded += len(double_encoded) 
    print(f"total number of characters in literals: {total_num_char_in_literals} and in double encoded: {total_num_char_in_double_encoded}")
    print(f"and their difference is: {total_num_char_in_double_encoded - total_num_char_in_literals}")


if __name__ == "__main__":
    data = get_input()
    part1(input=data)
    part2(input=data)
