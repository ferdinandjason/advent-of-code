#ifndef _UTIL_H_
#define _UTIL_H_

#include<vector>
#include<string>
#include<ranges>
#include<numeric>
#include<algorithm>

#pragma once

using namespace std;

using u32 = uint32_t;
using i64 = int64_t;
using u64 = uint64_t;

namespace r = ranges;
namespace rv = ranges::views;

namespace aoc {
    template <typename T> inline size_t hash_value(T val) {
        const int size_t_bits = numeric_limits<size_t>::digits;
        const int length = (numeric_limits<T>::digits - 1) / size_t_bits;
        size_t seed = 0;
        T positive = val < 0 ? -1 - val : val;
        
        for(unsigned int i = length * size_t_bits; i > 0; i -= size_t_bits){
            seed ^= (size_t) (positive >> i) + (seed<<6) + (seed>>2);
        }
        seed ^= (size_t) val + (seed<<6) + (seed>>2);
        return seed;
    }

    template <typename T> void hash_combine(size_t& seed, T v) {
        seed ^= hash_value(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2); 
    }
    
    template <typename T> vector<T> extract_numbers(string& str) {
        string_view buffer{str};

        int i = 0;

        auto const extract_number = [&]() {
            bool neg = false;
            if (buffer[i] == '-') {
                neg = true; i++;
            }

            auto count = 0;
            auto n = r::fold_left(
                buffer | 
                    rv::drop(i) |
                    rv::take_while([&](const char c) {
                        return isdigit(c);
                    }),
                static_cast<i64>(0),
                [&](i64 acc, char now) {
                    count++;
                    return acc * 10 + (now - '0');
                }
            );

            if (neg) n *= -1;

            i += count;
            return static_cast<T>(n);
        };

        vector<T> ret;
        while(i < buffer.size()) {
            while(!isdigit(buffer[i]) && buffer[i] != '-') i++;
            ret.push_back(extract_number());
        }

        return ret;
    }

    vector<string> split(string str, string delims);
}

// my compiler doesn't have ranges::to()
namespace detail {
    template <typename C>
    struct to_helper {
    };
    
    template <typename Container, ranges::range R>
    requires convertible_to<ranges::range_value_t<R>, typename Container::value_type>
    Container operator|(R&& r, to_helper<Container>) {
        return Container{r.begin(), r.end()};
    }
}


template <ranges::range Container>
requires (!ranges::view<Container>)
auto to() {
    return detail::to_helper<Container>{};
}

#endif