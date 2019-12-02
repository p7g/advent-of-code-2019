from enum import Enum
from typing import List


class OpCode(Enum):
    ADD = 1
    MUL = 2
    HALT = 99


def get_input() -> List[int]:
    with open("input.txt", "r") as f:
        return list(map(int, f.read().split(",")))


def evaluate(code: List[int]) -> int:
    l = len(code)
    ip = 0

    def next() -> int:
        nonlocal ip
        instruction = code[ip]
        ip += 1
        return instruction

    while ip < l:
        instruction = OpCode(next())

        if instruction in (OpCode.ADD, OpCode.MUL):
            left = code[next()]
            right = code[next()]

            code[next()] = left + right if instruction == OpCode.ADD else left * right
        elif instruction == OpCode.HALT:
            return code[0]

    raise Exception("Did not encounter HALT")


def main():
    input = get_input()
    input[1] = 12
    input[2] = 2

    print(evaluate(input))


if __name__ == "__main__":
    main()
