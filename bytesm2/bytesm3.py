# Solution to BYTESM2 Problem (https://www.spoj.com/problems/BYTESM2/)

import sys

def parse_board_from_input(numbers, idx):
    h, w = numbers[idx], numbers[idx + 1]
    
    board = []
    idx += 2
    for _ in range(h):
        board.append(numbers[idx:idx + w])
        idx += w

    return board, idx

def compute_max_stone_count(board):

    h = len(board)
    w = len(board[0])

    dp = [[0 for _ in range(w)] for _ in range(h)]

    for j in range(w):
        dp[0][j] = board[0][j]

    for i in range(1, h):
        for j in range(w):
            dp[i][j] = dp[i - 1][j]

            if j > 0:
                dp[i][j] = max(dp[i][j], dp[i - 1][j - 1])

            if j < w - 1:
                dp[i][j] = max(dp[i][j], dp[i - 1][j + 1])

            dp[i][j] += board[i][j]

    
    if len(dp[0]) == 1:
        return dp[h - 1][0]
        
    return max(*dp[h - 1])

def main():
    # Probably there is some issue with newlines in test cases, so I decided to read all data at once,
    # otherwise the program crashed. 
    numbers = [int(n) for n in sys.stdin.read().split()]

    case_number = numbers[0]
    idx = 1
    for _ in range(case_number):
        board, idx = parse_board_from_input(numbers, idx)
        print (compute_max_stone_count(board))


if __name__ == "__main__":
    main()