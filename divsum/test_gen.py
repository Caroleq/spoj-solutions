

# https://stackoverflow.com/questions/70635382/fastest-way-to-produce-a-list-of-all-divisors-of-a-number

import numpy as np

def npDivs(N):
    if N == 0:
        return []
    divs = np.arange(1,int(N**0.5)+1)  # potential divisors up to √N
    divs = divs[N % divs==0]           # divisors
    comp = N//divs[::-1]               # complement quotients
    return np.concatenate((divs,comp[divs[-1]==comp[0]:])) # combined

test_case_cnt = 1000
with open("i2.in", "w") as obj_out:
    obj_out.write(str(test_case_cnt) + "\n")

    for i in range(test_case_cnt):
        obj_out.write(str(i) + "\n")

with open("i2.out", "w") as obj_out:
    obj_out.write(str(0) + "\n")
    for i in range(1, test_case_cnt):
        divs_sum = sum(npDivs(i)) - i
        obj_out.write(str(divs_sum) + "\n")