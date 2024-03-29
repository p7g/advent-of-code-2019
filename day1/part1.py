from functools import reduce
from typing import Iterable


def get_input() -> Iterable[int]:
    with open("input.txt", "r") as f:
        return map(int, f.readlines())


def calculate_fuel(mass: int) -> int:
    return (mass // 3) - 2


def sum_fuel(masses: Iterable[int]):
    return reduce(lambda sum, f: sum + calculate_fuel(f), masses)


if __name__ == "__main__":
    print(sum_fuel(get_input()))
