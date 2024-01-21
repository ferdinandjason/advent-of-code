# Advent Of Code 2023

Complete solution for AOC 2023 written in Modern C++ 23

## Requirement
* C++ 23
* CMake (https://cmake.org/download/)


## Installation
1. Copy `CMakeLists.txt.sample` to `CMakeLists.txt` and update the config that suits you.
    ```bash
    cp CMakeLists.txt.sample CMakeLists.txt
    ```
1. Build the project
    ```bash
    cmake build .
    ```
1. Place your input in the `input` directory with format `day_XX.txt`
1. Run the project
    * To run on the respective day 
        ```bash
        ./aoc <day>
        ```
    * To run the benchmark
        ```bash
        ./aoc benchmark <loop-count>
        ```


## Benchmark
Benchmark was run on MacBook Pro M1 (14-inch, 2021).

| Day | Benchmark AVG (ms) | Benchmark Min (ms) | Benchmark Stdev (ms) |
| --- | :-: | :-: | :-: |
|  1 | `0.042` | `0.040` | `0.004` |
|  2 | `0.016` | `0.014` | `0.002` |
|  3 | `0.080` | `0.077` | `0.003` |
|  4 | `0.035` | `0.034` | `0.002` |
|  5 | `0.042` | `0.037` | `0.027` |
|  6 | `0.001` | `0.000` | `0.002` |
|  7 | `0.860` | `0.805` | `0.053` |
|  8 | `1.675` | `1.631` | `0.038` |
|  9 | `0.072` | `0.068` | `0.005` |
| 10 | `2.763` | `2.350` | `0.795` |
| 11 | `0.160` | `0.152` | `0.009` |
| 12 | `79.169` | `75.726` | `2.098` |
| 13 | `0.675` | `0.648` | `0.027` |
| 14 | `17.191` | `14.506` | `1.171` |
| 15 | `0.389` | `0.377` | `0.012` |
| 16 | `286.379` | `276.866` | `5.312` |
| 17 | `459.892` | `379.458` | `55.495` |
| 18 | `0.136` | `0.121` | `0.019` |
| 19 | `0.572` | `0.557` | `0.015` |
| 20 | `23.982` | `23.479` | `0.514` |
| 21 | `8.757` | `8.375` | `0.360` |
| 22 | `4.608` | `4.373` | `0.228` |
| 23 | `71.725` | `69.077` | `3.123` |
| 24 | `0.580` | `0.547` | `0.027` |
| 25 | `9.280` | `9.086` | `0.132` |
| **Total** | `969.078` | `868.404` | - |

