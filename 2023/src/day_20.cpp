#include"aoc.h"
#include<ranges>
#include<algorithm>
#include<iostream>
#include<cmath>
#include<sstream>
#include<numeric>
#include<queue>
#include<memory>
#include<map>
#include<unordered_map>
#include<unordered_set>
#include<functional>
#include<cassert>

using namespace std;

namespace {
    enum class module_type {
        flip_flop,
        conjugation,
        broadcaster,
        none,
    };

    struct signal_module {
        string from, to;
        int signal;
    };

    class base_module {
        public:
        string name;
        vector<string> dests;
    
        base_module(string& name, vector<string>& dests) : name(name), dests(dests) {}

        virtual vector<signal_module> receive_signal(const signal_module& mod) {
            return {};
        }

        virtual void add_input(string& input) {}
    };

    class flip_flop: public base_module {
        int toggle = 0;

        public:
        flip_flop(string& name, vector<string>& dests) : base_module(name, dests) {}

        vector<signal_module> receive_signal(const signal_module& mod) override {
            if (mod.signal == 1) {
                return {};
            }

            toggle = toggle ^ 1;
            return dests | rv::transform([&](auto&& dest) -> signal_module {
                return {name, dest, toggle};
            }) | to<vector<signal_module>>();
        }
    };

    class conjugation: public base_module {
        public:
        unordered_map<string, int> state;

        conjugation(string& name, vector<string>& dests) : base_module(name, dests) {}

        vector<signal_module> receive_signal(const signal_module& mod) override {
            state[mod.from] = mod.signal;
            int value = 0;
            for(auto [from, val]: state) {
                if (val == 0) {
                    value = 1;
                    break;
                }
            }

            return dests | rv::transform([&](auto&& dest) -> signal_module {
                return {name, dest, value};
            }) | to<vector<signal_module>>();
        }

        void add_input(string& input) override {
            state[input] = 0;
        }
    };

    class broadcaster: public base_module {
        public:
        broadcaster(string& name, vector<string>& dests) : base_module(name, dests) {}

        vector<signal_module> receive_signal(const signal_module& mod) override {
            return dests | rv::transform([&](auto&& dest) -> signal_module {
                return {name, dest, 0};
            }) | to<vector<signal_module>>();
        }
    };

    unique_ptr<base_module> make_module(module_type type, string& name, vector<string>& dests) {
        using factory_fn = function<unique_ptr<base_module>(string&, vector<string>&)>;
        static const unordered_map<module_type, factory_fn> factory = {
            {
                module_type::broadcaster, 
                [](string& name, vector<string>& dests) { return make_unique<broadcaster>(name, dests); }
            }, {
                module_type::flip_flop,
                [](string& name, vector<string>& dests) { return make_unique<flip_flop>(name, dests); }
            }, {
                module_type::conjugation,
                [](string& name, vector<string>& dests) { return make_unique<conjugation>(name, dests); }
            }, {
                module_type::none,
                [](string& name, vector<string>& dests) { return make_unique<base_module>(name, dests); }
            }
        };

        return factory.at(type)(name, dests);
    }

    pair<module_type, string> parse_module_type(string name) {
        if (name.front() == '%') {
            return {module_type::flip_flop, name.substr(1)};
        }

        if (name.front() == '&') {
            return {module_type::conjugation, name.substr(1)};
        }

        if (name == "broadcaster") {
            return {module_type::broadcaster, name};
        }

        return {module_type::none, name};
    }

    using modules = unordered_map<string, unique_ptr<base_module>>;
    modules parse_input(vector<string>& input) {
        unordered_map<string, vector<string>> conjugation_input;

        auto mod = input | 
            rv::transform([&](auto&& line) -> modules::value_type {
                auto parts = aoc::split(line, "->");
                auto lhs = parts[0], rhs = parts[1];
                auto [type, name] = parse_module_type(lhs.substr(0, lhs.size() - 1));
                auto dests = aoc::split(rhs.substr(1), ", ");

                if (type == module_type::flip_flop) {
                    for(auto dest: dests) {
                        conjugation_input[dest].push_back(name);
                    }
                }

                return {name, make_module(type, name, dests)};
            }) | 
            to<modules>();

        for(auto [name, inputs]: conjugation_input) {
            for(auto input: inputs) {
                mod[name]->add_input(input);
            }
        }

        return mod;
    }

    i64 solve_part1(modules& mod) {
        int count = 1000;
        i64 count_low_global = 0;
        i64 count_high_global = 0;

        while(count--) {
            queue<signal_module> q;
            q.push({"button", "broadcaster", 0});

            int count_low = 0;
            int count_high = 0;

            while(!q.empty()) {
                auto signal = q.front(); q.pop();

                if (signal.signal == 0) count_low++;
                else count_high++;

                auto emitted_signal = mod[signal.to]->receive_signal(signal);
                for(auto& new_signal : emitted_signal) {
                    q.push(new_signal);
                }
            }

            count_low_global += count_low;
            count_high_global += count_high;
        }

        return count_low_global * count_high_global;
    }

    i64 solve_part2(modules& mod) {
        // 2910 cycle is enough
        int count = 2910, i = 0;
        unordered_map<string, int> counter;
        while(count--) {
            i++;

            queue<signal_module> q;
            q.push({"button", "broadcaster", 0});

            int count_low = 0;
            int count_high = 0;

            while(!q.empty()) {
                auto signal = q.front(); q.pop();

                if (signal.to == "bb" && signal.signal == 1) {
                    counter[signal.from] = i;
                    if (counter.size() == 4) {
                        count = 0;
                        break;
                    }
                }

                auto emitted_signal = mod[signal.to]->receive_signal(signal);
                for(auto& new_signal : emitted_signal) {
                    q.push(new_signal);
                }
            }
        }

        auto add_1000 = [](i64 cycle) {
            return cycle + 1000; // from part1
        };

        return *r::fold_left_first(counter | rv::values | rv::transform(add_1000), [](i64 acc, i64 now) {
            return lcm(acc, now);
        });
    }

    void add_none_module(modules& mod, string name) {
        vector<string> empty;
        
        mod[name] = make_module(module_type::none, name, empty);
    }
}

void aoc::day_20(vector<string>& input) {
    auto modules = parse_input(input);
    add_none_module(modules, "rx");

    auto part1 = solve_part1(modules);
    auto part2 = solve_part2(modules);

    assert(part1 == 883726240);
    assert(part2 == 211712400442661);
    
    // cout << "PART I  : " << part1 << endl;
    // cout << "PART II : " << part2 << endl;
}
