# Solution to TABY problem (https://www.spoj.com/problems/TABY/)

import sys

def compute_minimum_shifts_cnt(starting_pos, destination_pos):
    if len(starting_pos) == 0:
        return 0
    
    dp = [0 for _ in range(len(starting_pos))]
    dp[0] = abs(starting_pos[0] - destination_pos[0])

    for idx in range(1, len(starting_pos)):

        dist_directed = destination_pos[idx] - starting_pos[idx]
        dist_prev_directed = destination_pos[idx - 1] - starting_pos[idx - 1]

        if dist_directed == 0:
            dp[idx] = dp[idx - 1]
            continue

        same_direction = dist_prev_directed < 0 and dist_directed < 0 or dist_prev_directed > 0 and dist_directed > 0
        if same_direction:
            if abs(dist_prev_directed) > abs(dist_directed):
                dp[idx] = dp[idx - 1]
            else:
                 dp[idx] = dp[idx - 1] + abs(dist_directed) - abs(dist_prev_directed)
        else:
             dp[idx] = dp[idx - 1] + abs(dist_directed)
    
    return dp[-1]

def main():
    test_case_cnt = int(sys.stdin.readline().rstrip())

    for _ in range(test_case_cnt):
        sys.stdin.readline()
        start_pos = sys.stdin.readline().rstrip().split()
        start_pos = [int(x) for x in start_pos]

        dest_pos = sys.stdin.readline().rstrip().split()
        dest_pos = [int(x) for x in dest_pos]

        result = compute_minimum_shifts_cnt(start_pos, dest_pos)
        print(result)


if __name__ == "__main__":
    main()