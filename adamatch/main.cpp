/// Solution to Ada and Nucleobase Problem (https://www.spoj.com/problems/ADAMATCH/)
/// Writeup to the problem is available here: https://medium.com/@karolina.gontarek20/fft-in-competitive-programming-solving-adamatch-spoj-problem-ee98577540b2

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
        {'A', 0},
        {'C', 0},
        {'T', 0},
        {'G', 0},
    };
    mapping[character] = 1;
    return mapping;
}

std::map<char, uint8_t> create_inverse_mapping_for_bases(const char character)
{
    std::map<char, uint8_t> mapping {
        {'A', 1},
        {'C', 1},
        {'T', 1},
        {'G', 1},
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

complex_vec compute_mismatch_count_for_char(const std::string &dna_sequence, const std::string &rev_pattern, const char character)
{
    const auto polynomial_size = compute_polynomial_size(dna_sequence.size());

    const auto dna_coding = create_mapping_for_bases(character);
    auto dna_seq_polynomial = create_polynomial_coeffs_from_data(dna_sequence, dna_coding, polynomial_size);

    const auto pattern_coding = create_inverse_mapping_for_bases(character);
    auto pattern_polynomial = create_polynomial_coeffs_from_data(rev_pattern, pattern_coding, polynomial_size);

    return multiply_two_polynomials(dna_seq_polynomial, pattern_polynomial);
}

uint32_t compute_minimum_hamming_distance(const std::string &dna_sequence, const std::string &pattern)
{
    std::string rev_pattern = pattern;
    std::reverse(rev_pattern.begin(), rev_pattern.end());

    const auto mismatch_count_A = compute_mismatch_count_for_char(dna_sequence, rev_pattern, 'A');
    const auto mismatch_count_C = compute_mismatch_count_for_char(dna_sequence, rev_pattern, 'C');
    const auto mismatch_count_T = compute_mismatch_count_for_char(dna_sequence, rev_pattern, 'T');
    const auto mismatch_count_G = compute_mismatch_count_for_char(dna_sequence, rev_pattern, 'G');

    uint32_t min_hamming_dist = rev_pattern.size();
    for (size_t idx = pattern.size() - 1; idx < dna_sequence.size(); ++idx)
    {
        const uint32_t hamming_distance = mismatch_count_A[idx].real() + mismatch_count_C[idx].real() + 
                                          mismatch_count_T[idx].real() + mismatch_count_G[idx].real() + 0.1;
        min_hamming_dist = std::min(min_hamming_dist, hamming_distance);
    }   
    return min_hamming_dist;
}

int main()
{
    std::ios::sync_with_stdio(false);
    
    std::string dna_sequence;
    std::cin >> dna_sequence;

    std::string pattern;
    std::cin >> pattern;

    const auto minimum_hamming_distance = compute_minimum_hamming_distance(dna_sequence, pattern);
    std::cout << minimum_hamming_distance << std::endl;

    return 0;
}
