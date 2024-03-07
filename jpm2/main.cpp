// Solution to Just Primes II Problem (https://www.spoj.com/problems/JPM2/)

#include <iostream>
#include <cmath>
#include <vector>
#include <complex>

using namespace std::complex_literals;

using complex_vec = std::vector<std::complex<double>>;


complex_vec create_prime_coeffs(uint64_t degree, const std::vector<uint64_t> &primes)
{
    complex_vec complex_coeffs(degree);
    for (const auto prime: primes)
    {
        complex_coeffs[prime] = 1;
    }
    return complex_coeffs;
}

complex_vec create_prime_x2_coeffs(uint64_t degree, const std::vector<uint64_t> &primes)
{
    complex_vec complex_coeffs(degree);
    for (const auto prime: primes)
    {
        if (2 * prime >= degree)
        {
            break;
        }
        complex_coeffs[2 * prime] = 1;
    }
    return complex_coeffs;
}

std::vector<uint64_t> generate_primes_up_to(uint64_t max_number)
{
    std::vector<uint64_t> primes;
    std::vector<bool> prime_flags(max_number + 1);

    const uint64_t max_number_sqrt = sqrt(max_number) + 1;
    for (uint64_t number = 2; number < max_number_sqrt; number++)
    {
        if (prime_flags[number] == 0)
        {
            primes.push_back(number);
        }
        else
        {
            continue;
        }

        uint64_t number_multiple = number * number;
        while (number_multiple < max_number + 1)
        {
            prime_flags[number_multiple] = 1;
            number_multiple += number;
        }
    }

    for (uint32_t number = max_number_sqrt; number < max_number + 1; number++)
    {
        if (prime_flags[number] == false)
        {
            primes.push_back(number);
        }
    }

    return primes;
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

complex_vec vectors_dot_product(const complex_vec &vector1, const complex_vec &vector2)
{
    const size_t vector_size = vector1.size();
    complex_vec multiplied_vector(vector_size);
    for (size_t idx = 0; idx < vector_size; idx++)
    {
         multiplied_vector[idx] = vector1[idx] * vector2[idx];
    }
    return multiplied_vector;
}

complex_vec compute_coeffs_to_subtract(const std::vector<uint64_t> &primes, uint64_t max_number, const complex_vec &prime_poly_values)
{
    auto prime_x2_poly_coeffs = create_prime_x2_coeffs(4 * max_number, primes);

    fft(prime_x2_poly_coeffs, false);
    auto &prime_x2_poly_vals = prime_x2_poly_coeffs;

    complex_vec poly_vals = vectors_dot_product(prime_poly_values, prime_x2_poly_vals); 

    fft(poly_vals, true);
    auto &poly_to_subtract_coeffs = poly_vals;

    for (const auto p1: primes)
    {
        if (3 * p1 >= max_number)
        {
            break;
        }
        poly_to_subtract_coeffs[3 * p1].real((poly_to_subtract_coeffs[3 * p1].real() - 0.999));
    }
    return poly_to_subtract_coeffs;
}

std::vector<std::pair<uint64_t, uint64_t>> precompute(const uint64_t max_number)
{
    std::vector<std::pair<uint64_t, uint64_t>> result(max_number);

    const auto primes = generate_primes_up_to(max_number);
    auto prime_poly_coeffs = create_prime_coeffs(4 * max_number, primes);

    for (const auto prime: primes)
    {
        result[prime] = {1, 1};
    }

    fft(prime_poly_coeffs, false);
    auto &prime_poly_values = prime_poly_coeffs;

    auto square_prime_poly_vals = vectors_dot_product(prime_poly_values, prime_poly_values); 
    
    fft(square_prime_poly_vals, true);
    auto &square_prime_poly_coeffs = square_prime_poly_vals;

    for (const auto prime: primes)
    {
        if (2 * prime >= max_number)
        {
            break;
        }
        square_prime_poly_coeffs[2 * prime].real((square_prime_poly_coeffs[2 * prime].real() - 0.999));
    }

    for (size_t idx = 0; idx < max_number; idx++)
    {
        if (result[idx].first == 1)
        {
            square_prime_poly_coeffs[idx] = {0, 0};
            continue;
        }

        const int tmp = square_prime_poly_coeffs[idx].real() / 2 + 0.01;

        square_prime_poly_coeffs[idx] = {(double)(tmp), 0};

        if (tmp > 0)
        {
            result[idx] = {2, tmp};
        }
    }  

    fft(square_prime_poly_coeffs, false);
    auto &modified_square_prime_poly_vals = square_prime_poly_coeffs;

    auto modified_exp3_prime_poly_vals = vectors_dot_product(prime_poly_values, modified_square_prime_poly_vals);

    fft(modified_exp3_prime_poly_vals, true);
    auto coeffs_to_subtract = compute_coeffs_to_subtract(primes, max_number, prime_poly_values);

    for (size_t idx = 0; idx < max_number; idx++)
    {
         modified_exp3_prime_poly_vals[idx].real(modified_exp3_prime_poly_vals[idx].real() - coeffs_to_subtract[idx].real());
    }

    for (size_t idx = 0; idx < max_number; idx++)
    {
        if (result[idx].first == 0 && modified_exp3_prime_poly_vals[idx].real() > 0.1)
        {
            result[idx] = {3, (modified_exp3_prime_poly_vals[idx].real() + 0.1) / 3};
        }
    }   

    return result;
}

int main()
{
    std::ios::sync_with_stdio(false);
    size_t cases_count;
    std::cin >> cases_count;

    const uint64_t max_number = 524288;
    const auto result = precompute(max_number);

    for (size_t idx = 0; idx < cases_count; idx++)
    {
        int number;
        std::cin >> number;
        
        if (number == 0 || number == 1 || number == 4 || number == 6)
        {
            std::cout << -1 << " " << -1 << std::endl;
            continue;
        }

        std::cout << result[number].first << " " << result[number].second << std::endl;
    }

    return 0;
}
