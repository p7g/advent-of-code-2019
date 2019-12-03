from itertools import permutations
from multiprocessing import Pool
from typing import Tuple

from part1 import evaluate, get_input


def try_permutation(values: Tuple[int, ...]):
    i, j = values

    code = code_input.copy()
    code[1] = i
    code[2] = j

    result = evaluate(code)

    if result == 19690720:
        return 100 * i + j


def main():
    with Pool(8) as pool:
        result = [
            r
            for r in pool.map(try_permutation, permutations(range(100), 2))
            if r is not None
        ]

    if len(result) == 1:
        return result[0]

    raise ValueError(result)


code_input = get_input()


if __name__ == "__main__":
    print(main())
