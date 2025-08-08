# Solution to Totient Extreme (https://www.spoj.com/problems/DCEPCA03/)

import sys


# Implementation based on totient computing from https://www.geeksforgeeks.org/dsa/summation-gcd-pairs-n/ 
def compute_totient(max_totient):

    phi = [0 for _ in range(max_totient + 1)]
    phi[1] = 1

    for idx1 in range(2, max_totient + 1):

        if phi[idx1] == 0:
            phi[idx1] = idx1 - 1

            idx2 = idx1 << 1
            while idx2 <= max_totient:
                if phi[idx2] == 0:
                    phi[idx2] = idx2

                phi[idx2] = (phi[idx2] // idx1) * (idx1 - 1)
                idx2 += idx1

    return phi


def compute_sums(totients):
    sums = [0 for _ in range(len(totients))]
    
    for idx in range(1, len(totients)):
        sums[idx] = sums[idx - 1] + totients[idx] 

    return sums


def main():
    totients = compute_totient(10 ** 4)
    sums = compute_sums(totients)

    case_cnt = int(sys.stdin.readline().strip())
    result = ""
    for _ in range(case_cnt):
        test_case = int(sys.stdin.readline().strip())
        result += str(sums[test_case] ** 2) + "\n"

    print(result, end="")

if __name__ == "__main__":
    main()