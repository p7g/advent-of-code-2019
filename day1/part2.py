from functools import reduce

from part1 import get_input, calculate_fuel


def account_for_fuel_mass(total: int) -> int:
    current = total

    while 0 < (extra := calculate_fuel(current)):
        total += extra
        current = extra

    return total


def main():
    total = 0
    for module in get_input():
        fuel = calculate_fuel(module)
        total += account_for_fuel_mass(fuel)
    return total


if __name__ == '__main__':
    test_cases = [
        (14, 2),
        (1969, 966),
        (100756, 50346),
    ]

    for input, expected in test_cases:
        assert account_for_fuel_mass(calculate_fuel(input)) == expected, \
            f'Expected mass of {input} to require {expected} fuel'

    print(main())
