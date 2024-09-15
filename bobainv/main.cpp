/// Solution to Boba Inversion (https://www.spoj.com/problems/BOBAINV/)

#include <iostream>
#include <string>

long long dp[5000][5000] = {};
long long sweetness[5000];

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    size_t tea_sequence_size = 0;
    std::cin >> tea_sequence_size;

    for (size_t idx = 0; idx < tea_sequence_size; idx++)
    {
        std::cin >> sweetness[idx];
    }

    for (size_t range_len = 1; range_len < tea_sequence_size + 1; range_len++)
    {
        for (size_t left_idx = 0; left_idx < tea_sequence_size - range_len; left_idx++)
        {
            dp[left_idx][left_idx + range_len] = dp[left_idx][left_idx + range_len - 1] + 
                                                 dp[left_idx + 1][left_idx + range_len] - 
                                                 dp[left_idx + 1][left_idx + range_len - 1];
            if (sweetness[left_idx] > sweetness[left_idx + range_len])
            {
                dp[left_idx][left_idx + range_len] += 1;
            }
        }
    }

    size_t queries_cnt = 0;
    std::cin >> queries_cnt;

    std::string result;
    for (size_t query_idx = 0; query_idx < queries_cnt; query_idx++)
    {
        size_t left_idx;
        size_t right_idx;
        std::cin >> left_idx;
        std::cin >> right_idx;

        result += std::to_string(dp[left_idx - 1][right_idx - 1]) + '\n';
    }

    std::cout << result;

    return 0;
}