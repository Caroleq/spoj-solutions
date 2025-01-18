//Solution to Feynman Problem (https://www.spoj.com/problems/SAMER08F/)

#include <iostream>

int main()
{
    while (true)
    {
        int squares_cnt_per_grid;
        std::cin >> squares_cnt_per_grid;
        if (squares_cnt_per_grid == 0)
        {
            return 0;
        }

        const int result = squares_cnt_per_grid * (squares_cnt_per_grid + 1) * (2 * squares_cnt_per_grid + 1) / 6;
        std::cout << result << std::endl;
    } 
}