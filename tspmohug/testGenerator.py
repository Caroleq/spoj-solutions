from random import randint


problem_size = 15

cards = []
for _ in range(problem_size):
    power = randint(0, 10)
    cooldown = randint(0, power)
    cards.append((power, cooldown))

with open("test.in", "w") as test_obj:
    test_obj.write(str(problem_size) + "\n")
    for (power, cooldown) in cards:
        test_obj.write(str(power) + " " + str(cooldown) + "\n")