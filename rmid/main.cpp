/// Solution to Running Median Problem (https://www.spoj.com/problems/RMID/)

#include<bits/stdc++.h>


void add_item_to_median_structure(
    const long number,
    std::priority_queue<long, std::vector<long>, std::less<long> > &less_or_eq_median,
    std::priority_queue<long, std::vector<long>, std::greater<long> > &above_median
)
{
    if (less_or_eq_median.empty())
    {
        less_or_eq_median.push(number);
    }  
    else if (above_median.empty())
    {
        less_or_eq_median.push(number);
        above_median.push(less_or_eq_median.top());
        less_or_eq_median.pop();
        return;
    }
    else if (number <= above_median.top())
    {
        less_or_eq_median.push(number);
        if (less_or_eq_median.size() > above_median.size() + 1)
        {
            above_median.push(less_or_eq_median.top());
            less_or_eq_median.pop();
        }
    }
    else
    {
        above_median.push(number);
        if (less_or_eq_median.size() < above_median.size())
        {
            less_or_eq_median.push(above_median.top());
            above_median.pop();
        }
    }
}

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    while (true)
    {
        std::priority_queue<long, std::vector<long>, std::less<long> > less_or_eq_median;
        std::priority_queue<long, std::vector<long>, std::greater<long> > above_median; 

        while (true)
        {
            long number;
            std::cin >> number;
            if (!std::cin)
            {
                return 0;
            }

            if (number == 0) {
                std::cout << '\n';
                break;
            }

            if (number == -1)
            {
                long result = less_or_eq_median.top();
                less_or_eq_median.pop();
                std::cout << result << '\n';

                if (less_or_eq_median.size() < above_median.size())
                {
                    long a = above_median.top();
                    above_median.pop();
                    less_or_eq_median.push(a);
                }
            }
            else 
            {
                add_item_to_median_structure(number, less_or_eq_median, above_median);
            }
        }
        
    }
}