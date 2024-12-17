# My inputs
A = 22817223
B = 0
C = 0

program = [2, 4, 1, 2, 7, 5, 4, 5, 0, 3, 1, 7, 5, 5, 3, 0]


def run(A, B, C, program):
    def combo_operand(operand):
        if operand < 4:
            return operand
        elif operand == 4:
            return A
        elif operand == 5:
            return B
        elif operand == 6:
            return C
        raise ValueError("Invalid operand, you are out of 3 bit operands")

    pointer = 0
    output = []

    while pointer < len(program):
        opcode = program[pointer]
        operand = program[pointer + 1]
        # Evaluate instructions:
        if opcode == 0:  # adv (division)
            A >>= combo_operand(operand)
        elif opcode == 1:  # bxl (xor)
            B ^= operand  # TODO: check
        elif opcode == 2:  # bst (combo)
            B = combo_operand(operand) % 8
        elif opcode == 3:  # jnz
            if A != 0:
                pointer = operand
                continue
        elif opcode == 4:  # bxc
            B ^= C
        elif opcode == 5:  # out
            output.append(combo_operand(operand) % 8)
        elif opcode == 6:  # bdv
            B = A >> combo_operand(operand)
        elif opcode == 7:  #
            C = A >> combo_operand(operand)
        pointer += 2

        print(A, B, C)

    return ",".join(map(str, output))


print(run(A, B, C, program))
