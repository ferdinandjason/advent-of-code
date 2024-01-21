#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<unordered_map>
#include<map>
#include<cassert>

using namespace std;

namespace {
    void roll_north(vector<string>& maps) {
        int n = maps.size(), m = maps[0].size();

        for(int i = 0; i < m; i++) {
            int count_o = 0, cube_rock_idx = -1;
            for(int j = 0; j < n; j++) {
                if (maps[j][i] == '#') {
                    if (count_o != 0)
                        for(int x = cube_rock_idx + 1; x < j; x++) {
                            if (count_o > 0) {
                                maps[x][i] = 'O';
                                count_o--;
                            } else maps[x][i] = '.';
                        }
                    
                    cube_rock_idx = j;
                } else if (maps[j][i] == 'O') count_o++;
            }

            if (count_o != 0) {
                for(int x = cube_rock_idx + 1; x < n; x++) {
                    if (count_o > 0) {
                        maps[x][i] = 'O';
                        count_o--;
                    } else maps[x][i] = '.';
                }
            }
        }
    }

    void roll_south(vector<string> &maps) {
        int n = maps.size(), m = maps[0].size();

        for(int i = 0; i < m; i++) {
            int count_o = 0, cube_rock_idx = n;
            for(int j = n - 1; j >= 0; j--) {
                if (maps[j][i] == '#') {
                    if (count_o != 0) 
                        for(int x = cube_rock_idx - 1; x > j; x--) {
                            if (count_o > 0) {
                                maps[x][i] = 'O';
                                count_o--;
                            } else maps[x][i] = '.';
                        }

                    cube_rock_idx = j;
                } else if (maps[j][i] == 'O') count_o++;
            }
            if (count_o != 0) {
                for(int x = cube_rock_idx - 1; x >= 0; x--) {
                    if (count_o > 0) {
                        maps[x][i] = 'O';
                        count_o--;
                    } else maps[x][i] = '.';
                }
            }
        }
    }

    void roll_west(vector<string> &maps) {
        int n = maps.size(), m = maps[0].size();

        for(int i = 0; i < n; i++) {
            int count_o = 0, cube_rock_idx = -1;
            for(int j = 0; j < m; j++) {
                if (maps[i][j] == '#') {
                    if (count_o != 0)
                        for(int x = cube_rock_idx + 1; x < j; x++) {
                            if (count_o > 0) {
                                maps[i][x] = 'O';
                                count_o--;
                            } else  maps[i][x] = '.';
                        }

                    cube_rock_idx = j;
                } else if (maps[i][j] == 'O') count_o++;
            }
            if (count_o != 0) {
                for(int x = cube_rock_idx + 1; x < m; x++) {
                    if (count_o > 0) {
                        maps[i][x] = 'O';
                        count_o--;
                    } else maps[i][x] = '.';
                }
            }
        }
    }

    void roll_east(vector<string> &maps) {
        int n = maps.size(), m = maps[0].size();

        for(int i = 0; i < n; i++) {
            int count_o = 0, cube_rock_idx = m;
            for(int j = n - 1; j >= 0; j--) {
                if (maps[i][j] == '#') {
                    if (count_o != 0)
                        for(int x = cube_rock_idx - 1; x > j; x--) {
                            if (count_o > 0) {
                                maps[i][x] = 'O';
                                count_o--;
                            } else maps[i][x] = '.';
                        }

                    cube_rock_idx = j;
                } else if (maps[i][j] == 'O') count_o++;
            }
            if (count_o != 0) {
                for(int x = cube_rock_idx - 1; x > -1; x--) {
                    if (count_o > 0) {
                        maps[i][x] = 'O';
                        count_o--;
                    } else maps[i][x] = '.';
                }
            }
        }
    }

    size_t hash_map(vector<string>& map) {
        size_t seed = map.size();
        for(auto& i : map) {
            aoc::hash_combine(seed, hash<string>{}(i));
        }

        return seed;
    }

    void roll(vector<string>& maps) {
        roll_north(maps);
        roll_west(maps);
        roll_south(maps);
        roll_east(maps);
    }

    void roll_n_times(vector<string>& maps, int n) {
        int counter = 0, start_loop_idx = -1, end_loop_idx = -1;
        unordered_map<size_t, int> hash_map_counter;

        while(true) {
            roll(maps); counter++;
            auto val = hash_map(maps);

            if (auto it = hash_map_counter.find(val); it != hash_map_counter.end()) {
                start_loop_idx = it->second;
                end_loop_idx = counter;
                break;
            }

            hash_map_counter[val] = counter;
        }

        int mod = end_loop_idx - start_loop_idx;
        int remaining_roll = (n - start_loop_idx) % mod;

        while(remaining_roll--) {
            roll(maps);
        }
    }
}

void aoc::day_14(vector<string>& input) {
    roll_north(input);

    auto part1 = r::fold_left(
        input | rv::enumerate | rv::transform([&](auto&& row_with_index) {
            auto [index, row] = row_with_index;
            auto count_o = r::count_if(row, [](auto&& chr) { return chr == 'O'; } );
            return count_o * (input.size() - index);
        }), 0, plus<>()
    );

    roll_n_times(input, 1000000000);

    auto part2 = r::fold_left(
        input | rv::enumerate | rv::transform([&](auto&& row_with_index) {
            auto [index, row] = row_with_index;
            auto count_o = r::count_if(row, [](auto&& chr) { return chr == 'O'; } );

            return count_o * (input.size() - index);
        }), 0, plus<>()
    );

    assert(part1 == 103614);
    assert(part2 == 83790);

    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
