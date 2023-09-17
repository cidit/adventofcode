from hashlib import md5


def validate(hash: str, num_zeros: int):
    return hash.find("0" * num_zeros) == 0


def make_hash(secret_key: str, number: int):
    return md5(f"{secret_key}{number}".encode())


def find_salt(secret_key, num_zeros):
    salt = 0
    hash_valid = lambda: validate(
        make_hash(secret_key, salt).hexdigest(), num_zeros)
    while not hash_valid() and salt is not -1:
        salt += 1
    return salt


def part1():
    print("\n~~~ Part 1 ~~~\n")
    with open("input.data", "r") as input_file:
        secret_key = input_file.read()
        number = find_salt(secret_key, num_zeros=5)
        print(f"the number is {number}")


def part2():
    print("\n~~~ Part 2 ~~~\n")
    with open("input.data", "r") as input_file:
        secret_key = input_file.read()
        number = find_salt(secret_key, num_zeros=6)
        print(f"the number is {number}")


if __name__ == "__main__":
    part1()
    part2()
