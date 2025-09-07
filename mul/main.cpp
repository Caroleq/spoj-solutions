/// Solution to Fast Multiplication Problem (https://www.spoj.com/problems/MUL/)
/// Similar solution in Rust was getting TLE

#include <iostream>
#include <cmath>
#include <iomanip>
#include <vector>
#include <complex>
#include <algorithm>

using namespace std::complex_literals;

std::pair<std::vector<int>, bool> read_polynomial()
{
    std::string string_number;
    std::cin >> string_number;

    std::vector<int> number_vector;

    bool negative = false;
    size_t start = 0;
    if (string_number[0] == '-')
    {
        start = 1;
        negative = true;
    }

    for (size_t idx = start; idx < string_number.size(); idx++)
    {
        number_vector.push_back(string_number[idx] - '0');
    }
    return {number_vector, negative};
}

using complex_vec = std::vector<std::complex<double>>;

complex_vec create_complex_coeffs(const std::vector<int> &coeffs, size_t degree)
{
    size_t n = 1;
    while (n < 2 * (degree + 1))
    {
        n <<= 1;
    }

    complex_vec complex_coeffs(n);

    for (size_t idx = 0; idx < coeffs.size(); idx++)
    {
        complex_coeffs[n - coeffs.size() + idx] = coeffs[idx];
    }
    std::reverse(complex_coeffs.begin(), complex_coeffs.end());

    return complex_coeffs;
}

void fft(complex_vec &polynomial, bool invert) 
{
    const size_t polyLength = polynomial.size();

    for (int i = 1, j = 0; i < polyLength; i++) 
    {
        int number = polyLength >> 1;
        for (; j & number; number >>= 1)
        {
            j ^= number;
        }
        j ^= number;

        if (i < j)
        {
            swap(polynomial[i], polynomial[j]);
        }
    }

    const static double pi = std::acos(-1);

    for (int len = 2; len <= polyLength; len <<= 1) 
    {
        std::complex<double> multiplier =  std::exp(1i * pi * 2.0 / (double)len);
        if (invert)
        {
            multiplier =  std::exp(-1i * pi * 2.0 / (double)len);
        }
        for (int i = 0; i < polyLength; i += len) 
        {
            std::complex<double> current_w(1);
            for (int j = 0; j < len / 2; j++) 
            {
                const auto product = polynomial[i + j + len/2] * current_w;
                
                polynomial[i + j + len/2] = polynomial[i + j] - product;
                polynomial[i + j] = polynomial[i + j] + product;
                current_w *= multiplier;
            }
        }
    }

    if (invert) 
    {
        for (auto &coeff : polynomial)
        {
            coeff /= polyLength;
        }
    }
}

std::vector<int> normalize_polynomial( std::vector<long long int> &polynomial)
{
    std::vector<int> result;

    int carry_elem = 0;
    int idx = polynomial.size();

    for (;idx > 0; idx--)
    {
        carry_elem += polynomial[idx - 1];

        result.push_back(carry_elem % 10);
        carry_elem /= 10;
    }

    while (carry_elem > 0)
    {
        result.push_back(carry_elem % 10);
        carry_elem /= 10;
    }
    
    std::reverse(result.begin(), result.end());

    return result;
}

std::string vector_to_str_number(const std::vector<int> &polynomial)
{
    std::string result;

    int idx = 0;
    while (idx < polynomial.size()) 
    {
        if (polynomial[idx] != 0) 
        {
            break;
        }
        idx += 1;
    }
    
    if (idx == polynomial.size())
    {
        return "0";
    }

    while (idx < polynomial.size()) 
    {
        result += std::to_string(polynomial[idx]);
        idx += 1;
    }

    return result; 
}

int main()
{
    std::ios::sync_with_stdio(false);
    size_t cases_count;
    std::cin >> cases_count;
    
    for (size_t idx = 0; idx < cases_count; idx++)
    {
        const auto poly1 = read_polynomial();
        const auto poly2 = read_polynomial();

        size_t max_length = std::max(poly1.first.size(), poly2.first.size()); 
        
        auto complex_poly1 = create_complex_coeffs(poly1.first, max_length);
        auto complex_poly2 = create_complex_coeffs(poly2.first, max_length);

        fft(complex_poly1, false);
        fft(complex_poly2, false);

        complex_vec mul_values(complex_poly1.size());
        for (size_t idx = 0; idx < complex_poly1.size(); idx++)
        {
            mul_values[idx] = complex_poly1[idx] * complex_poly2[idx];
        }

        fft(mul_values, true);

        std::vector<long long int> result(mul_values.size());
        for (size_t idx = 0; idx < mul_values.size(); idx++)
        {
            result[idx] = mul_values[idx].real() + 0.3;
        }

        std::reverse(result.begin(), result.end());

        const auto res = normalize_polynomial(result);
        const auto result_str = vector_to_str_number(res);

        if (poly1.second && poly2.second || 
            !poly1.second && !poly2.second)
        {
            std::cout << result_str << std::endl;
        }
        else 
        {
            std::cout << '-' << result_str << std::endl;
        }
    }

    return 0;
}