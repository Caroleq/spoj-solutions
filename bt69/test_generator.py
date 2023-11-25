

test_cases = []
responses = []
for N in range(1, 100):
    for M in range(N + 1, 100):
        test_cases.append((str(N), str(M)))

        K = 0
        while N ^ K <= M:
            K += 1
        responses.append(str(K))

with open("small.in", "w") as in_file:
    in_file.write(str(len(test_cases)) + "\n")

    for test_case in test_cases:
        in_file.write(test_case[0] + " " + test_case[1] + "\n")

with open("small.out", "w") as out_file:

    for result in responses:
        out_file.write(result + "\n")