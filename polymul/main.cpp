/// Solution to Polynomial Multiplication Problem (https://www.spoj.com/problems/POLYMUL/)
/// Similar solution in Rust was getting TLE

#include <iostream>
#include <cmath>
#include <iomanip>
#include <vector>
#include <complex>
#include <algorithm>

using namespace std::complex_literals;

std::vector<int> read_polynomial(int degree)
{
    std::vector<int> polynomial(degree + 1);
    for (size_t idx = 0; idx < degree + 1; idx++)
    {
        int coefficient;
        std::cin >> coefficient;
        polynomial[idx] = coefficient;
    }
    return polynomial;
}

using complex_vec = std::vector<std::complex<double>>;

complex_vec create_lookup(size_t limit)
{
    const double pi = std::acos(-1);
    std::complex<double> mult =  std::exp(1i * pi * 2.0 / (double)limit);
    
    complex_vec lookup(limit);
    std::complex<double> current_lookup_value = 1;
    for (size_t idx = 0; idx < limit; idx++)
    {
        lookup[idx] = current_lookup_value;
        current_lookup_value *= mult;
    }
    return lookup;
}

complex_vec create_complex_coeffs(const std::vector<int> &coeffs)
{
    size_t n = 1;
    while (n < 2 * (coeffs.size()))
    {
        n <<= 1;
    }

    complex_vec complex_coeffs(n);

    for (size_t idx = 0; idx < coeffs.size(); idx++)
    {
        complex_coeffs[idx] = coeffs[idx];
    }

    return complex_coeffs;
}

complex_vec fft(const complex_vec &polynomial, const bool invert, const complex_vec &lut)
{
    const size_t plength = polynomial.size();
    if (plength == 1) 
    {
        return polynomial;
    }

    complex_vec even_deg_coeffs(plength/2);
    complex_vec odd_deg_coeffs(plength/2);

    for (size_t idx = 0; idx < plength / 2; idx++)
    {
        even_deg_coeffs[idx] = polynomial[idx * 2];
        odd_deg_coeffs[idx] =  polynomial[idx * 2 + 1];
    }

    const auto even_values = fft(even_deg_coeffs, invert, lut);
    const auto odd_values = fft(odd_deg_coeffs, invert,lut);

    complex_vec result(plength);

    for (size_t idx = 0; idx < (plength / 2); idx++)
    {
        const auto product = lut[idx * lut.size() / plength] * odd_values[idx];
        result[idx] = even_values[idx] + product;
        result[idx + plength/2] = even_values[idx] - product;
        if (invert)
        {
            result[idx] /= 2;
            result[idx + plength/2] /= 2;
        }
    }

    return result;
}

int main()
{
    std::ios::sync_with_stdio(false);
    size_t cases_count;
    std::cin >> cases_count;
 
    const auto lookup_table = create_lookup(32768);
    auto reversed_lookup_table = lookup_table;
    reversed_lookup_table.push_back(1.0);
    std::reverse(reversed_lookup_table.begin(), reversed_lookup_table.end());
    reversed_lookup_table.pop_back();
    
    for (size_t idx = 0; idx < cases_count; idx++)
    {
        int degree;
        std::cin >> degree;
        const auto poly1 = read_polynomial(degree);
        const auto poly2 = read_polynomial(degree);
        
        const auto complex_poly1 = create_complex_coeffs(poly1);
        const auto complex_poly2 = create_complex_coeffs(poly2);

        const auto values1 = fft(complex_poly1, false,lookup_table);
        const auto values2 = fft(complex_poly2, false,lookup_table);

        complex_vec mul_values(values1.size());
        for (size_t idx = 0; idx < values1.size(); idx++)
        {
            mul_values[idx] = values1[idx] * values2[idx];
        }

        const auto multiplied_poly = fft(mul_values, true, reversed_lookup_table);
    
        std::vector<long long int> result(multiplied_poly.size());
        for (size_t idx = 0; idx < multiplied_poly.size(); idx++)
        {
            result[idx] = multiplied_poly[idx].real() + 0.1;
        }

        for (int i = 0; i < 2 * degree + 1; i++)
        {
           std::cout << result[i] << " ";
        }
        std::cout << std::endl;
    }

    return 0;
}