def is_nice_string(string: str):
    def has_invalid_string():
        for invalid in ["ab", "cd", "pq", "xy"]:
            if string.find(invalid) >= 0:
                return True
        return False

    def has_enough_vowels():
        vowel_count = 0
        for letter in string:
            if "aeiou".find(letter) >= 0:
                vowel_count += 1
        return vowel_count >= 3

    def has_doubled_letter():
        for letter in string:
            if string.find(letter * 2) >= 0:
                return True
        return False

    return not has_invalid_string() and has_enough_vowels(
    ) and has_doubled_letter()


def is_really_nice_string(string: str):
    def has_doubled_two_letters():
        for index in range(len(string)-1):
            sample = string[index:index + 2]
            (before_sample, _, after_sample) = string.partition(sample)
            if before_sample.find(sample) >= 0 or after_sample.find(sample) >= 0:
                return True
        return False

    def has_separated_repeating_letter():
        for index in range(len(string)-2):
            if string[index] == string[index + 2]:
                return True
        return False

    return has_doubled_two_letters() and has_separated_repeating_letter()


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        nice_string_count = 0
        for string in input_file.readlines():
            if is_nice_string(string):
                nice_string_count += 1
        print(f"the number of nice strings is {nice_string_count}")


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        nice_string_count = 0
        for string in input_file.readlines():
            if is_really_nice_string(string):
                nice_string_count += 1
        print(f"the number of nice strings is {nice_string_count}")


if __name__ == "__main__":
    part1()
    part2()
