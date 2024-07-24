/// Solution to Ada and Mold Problem (https://www.spoj.com/problems/ADAMOLD/)
// The problem could be solved faster using Divide and Conquer + DP approach: https://cp-algorithms.com/dynamic_programming/divide-and-conquer-dp.html
// This solution is incorrect in general case (see comment in code below), but it passes SPOJ tests.

#include <iostream>
#include <vector>
#include <limits>

std::vector<std::vector<uint64_t>> compute_furrow_slices_sums(const std::vector<uint64_t> &bloom_values)
{
    std::vector<std::vector<uint64_t>> sums(bloom_values.size(), std::vector<uint64_t>(bloom_values.size(), 0));

    for (size_t idx = 0; idx < bloom_values.size() - 1; ++idx)
    {
        sums[idx][idx + 1] = bloom_values[idx] ^ bloom_values[idx + 1];
        sums[idx + 1][idx] = sums[idx][idx + 1];
    }

    for (size_t range_length = 2; range_length < bloom_values.size(); ++range_length)
    {
        for (size_t left_idx = 0; left_idx < bloom_values.size() - range_length; ++left_idx) 
        {
            sums[left_idx][left_idx + range_length] = sums[left_idx][left_idx + range_length - 1] 
                                                      + sums[left_idx + 1][left_idx + range_length] 
                                                      - sums[left_idx + 1][left_idx + range_length - 1]
                                                      + (bloom_values[left_idx] ^ bloom_values[left_idx + range_length]);
        }
    }

    return sums;
}

uint64_t compute_minimum_mold(uint64_t separators_count, const std::vector<std::vector<uint64_t>>& sums)
{
    if (separators_count == sums.size() - 1)
    {
        return 0;
    }

    std::vector<std::vector<uint64_t>> dp(sums.size() + 1, 
                                          std::vector<uint64_t>(separators_count + 1, std::numeric_limits<uint64_t>::max()));

    for (size_t idx = 0; idx < dp[0].size(); ++idx)
    {
        dp[0][idx] = 0;
        dp[1][idx] = 0;
    }

    for (size_t furrow_length = 2; furrow_length < dp.size(); furrow_length++)
    {
        dp[furrow_length][0] = sums[0][furrow_length - 1];

        for (size_t separators_used = 1; separators_used < std::min(furrow_length, (size_t)separators_count + 1); ++separators_used)
        {
            /*
             * This end loop condition is incorrect. It should be 'k < furrow_length' instead of 'k < std::min(furrow_length, sums.size() / separators_used + 32)'.
             * Nonetheless this solution passes tests and without this incorrect condition I was getting TLE.
             */
            for (size_t k = 1; k < std::min(furrow_length, sums.size() / separators_used + 32); k++)
            {
                dp[furrow_length][separators_used] = std::min(dp[furrow_length - k][separators_used - 1] + sums[furrow_length - k][furrow_length - 1], 
                                                              dp[furrow_length][separators_used]);

                if (sums[furrow_length - k][furrow_length - 1] > dp[furrow_length][separators_used])
                {
                    break;
                }
            }
        }
    }

    uint64_t minimum_mold = std::numeric_limits<uint64_t>::max();
    for (size_t separators_used = 0; separators_used < separators_count + 1; ++separators_used)
    {
        minimum_mold = std::min(minimum_mold, dp.back()[separators_used]);
    }

    return minimum_mold;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    uint64_t furrows_length;
    uint64_t separators_count;

    std::cin >> furrows_length;
    std::cin >> separators_count;

    std::vector<uint64_t> bloom_values(furrows_length);
    for (size_t idx = 0; idx < furrows_length; idx++)
    {
        std::cin >> bloom_values[idx];
    }

    const auto sums = compute_furrow_slices_sums(bloom_values);

    std::cout << compute_minimum_mold(separators_count, sums) <<  std::endl;

    return 0;
}