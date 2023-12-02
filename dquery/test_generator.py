# test generator for dquery spoj problem

from random import randint

sequence_size = 20
max_element_value = 40
queries_count = 40

sequence = []
for _ in range(sequence_size):
    number = randint(1, max_element_value)
    sequence.append(number)


queries = []
results = []
for _ in range(queries_count):
    left_idx = randint(1, sequence_size)
    right_idx = randint(left_idx, sequence_size)
    queries.append((left_idx, right_idx))
    results.append(len(set(sequence[left_idx - 1: right_idx])))


with open("small.input.in", "w") as ifile:
    ifile.write(str(sequence_size) + "\n")
    ifile.write(" ".join([str(x) for x in sequence]) + "\n")
    ifile.write(str(queries_count) + "\n")
    for query in queries:
        ifile.write(str(query[0]) + " " + str(query[1]) + "\n")

with open("small.output.out", "w") as ofile:
    for result in results:
        ofile.write(str(result) + "\n")