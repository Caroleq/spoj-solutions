/// Solution to Maximum Self-Matching Problem (https://www.spoj.com/problems/MAXMATCH/)

#include <iostream>
#include <cmath>
#include <iomanip>
#include <vector>
#include <complex>
#include <algorithm>
#include <map>

using namespace std::complex_literals;
using complex_vec = std::vector<std::complex<double>>;

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

uint32_t compute_polynomial_size(size_t data_length)
{
    size_t pow = 2;
    while (pow < data_length * 2)
    {
        pow = pow << 1;
    }
    return pow;
}

complex_vec create_polynomial_coeffs_from_data(const std::string &data, const std::map<char, uint8_t> &coding, size_t polynomial_size)
{
    complex_vec complex_coeffs(polynomial_size);
    for (size_t idx = 0; idx < data.size(); ++idx)
    {
        complex_coeffs[idx] = coding.at(data[idx]);
    }
    return complex_coeffs;
}

std::map<char, uint8_t> create_mapping_for_bases(const char character)
{
    std::map<char, uint8_t> mapping {
        {'a', 0},
        {'b', 0},
        {'c', 0},
    };
    mapping[character] = 1;
    return mapping;
}

std::map<char, uint8_t> create_inverse_mapping_for_bases(const char character)
{
    std::map<char, uint8_t> mapping {
        {'a', 1},
        {'b', 1},
        {'c', 1},
    };
    mapping[character] = 0;
    return mapping;
}

complex_vec multiply_two_polynomials(complex_vec &poly1, complex_vec &poly2)
{
    fft(poly1, false);
    fft(poly2, false);

    complex_vec mul_values(poly2.size());
    for (size_t idx = 0; idx < poly2.size(); idx++)
    {
        mul_values[idx] = poly1[idx] * poly2[idx];
    }

    fft(mul_values, true);
    return mul_values;
}

complex_vec compute_mismatch_count_for_char(const std::string &text, const std::string &rev_text, const char character)
{
    const auto polynomial_size = compute_polynomial_size(text.size());

    const auto text_coding = create_mapping_for_bases(character);
    auto text_polynomial = create_polynomial_coeffs_from_data(text, text_coding, polynomial_size);

    const auto rev_text_coding = create_inverse_mapping_for_bases(character);
    auto rev_text_polynomial = create_polynomial_coeffs_from_data(rev_text, rev_text_coding, polynomial_size);

    return multiply_two_polynomials(text_polynomial, rev_text_polynomial);
}

std::vector<uint32_t> compute_maximum_self_matching(const std::string &text)
{
    std::string rev_text = text;
    std::reverse(rev_text.begin(), rev_text.end());

    const auto mismatch_count_a = compute_mismatch_count_for_char(text, rev_text, 'a');
    const auto mismatch_count_b = compute_mismatch_count_for_char(text, rev_text, 'b');
    const auto mismatch_count_c = compute_mismatch_count_for_char(text, rev_text, 'c');

    std::vector<uint32_t> result; 
    for (size_t idx = rev_text.size(); idx < text.size() + rev_text.size() - 1; ++idx)
    {
        const uint32_t levenshtein_distance = mismatch_count_a[idx].real() + mismatch_count_b[idx].real() + 
                                              mismatch_count_c[idx].real() + 0.1;
        result.push_back(2 * text.size() - idx -  levenshtein_distance - 1);
    }   
    return result;
}

int main()
{
    std::ios::sync_with_stdio(false);
    
    std::string text;
    std::cin >> text;

    const auto result = compute_maximum_self_matching(text);
    const auto max_value = *std::max_element(result.begin(), result.end());

    std::cout << max_value << std::endl;
    for (int idx = 0; idx < result.size(); ++idx)
    {
        if (result[idx] == max_value)
        {
            std::cout << idx + 1 << " ";
        }
    }
    std::cout << std::endl;

    return 0;
}
