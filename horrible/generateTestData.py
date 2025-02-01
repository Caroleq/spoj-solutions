
from random import randint

max_number = 4000
max_value = 100
insert_queries_cnt = 400
sum_queries_cnt = 400

numbers = [0 for _ in range(max_number + 1)]
queries = []

for _ in range(insert_queries_cnt):
    one = randint(1, max_number)
    two = randint(1, max_number)

    smaller = min(one, two)
    larger = max(one, two)
    value = randint(1, max_value)

    for idx in range(smaller, larger + 1):
        numbers[idx] += value

    queries.append([0, smaller, larger, value])

results = []
for _ in range(sum_queries_cnt):
    one = randint(1, max_number)
    two = randint(1, max_number)

    smaller = min(one, two)
    larger = max(one, two)

    queries.append([1, smaller, larger])
    results.append(sum(numbers[smaller: larger + 1]))

with open("test.in", "w") as input_obj:
    input_obj.write("1\n")
    input_obj.write(str(max_number) + " " + str(insert_queries_cnt + sum_queries_cnt) + "\n")
    for query in queries:
        input_obj.write(" ".join([str(x) for x in query]) + "\n")
    

with open("test.out", "w") as output_obj:
    for result in results:
        output_obj.write(str(result) + "\n")