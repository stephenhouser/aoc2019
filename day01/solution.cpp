#include <getopt.h>	 // getopt
#include <cassert>	  // assert macro
#include <chrono>	  // high resolution timer
#include <fstream>	  // ifstream (reading file)
#include <numeric>	  // max, reduce, etc.
#include <print>
#include <ranges>  // ranges and views
#include <algorithm>
#include <string>  // strings
#include <vector>  // collection

using namespace std;

/* Update with data type and result types */
using data_t = vector<size_t>;
using result_t = size_t;

/* Read the data file... */
const data_t read_data(const string& filename) {
	data_t data;

	std::ifstream ifs(filename);

	string line;
	while (getline(ifs, line)) {
		if (!line.empty()) {
			data.push_back(stoul(line));
		}
	}

	return data;
}

/* Part 1 */
result_t part1(const data_t& data) {
	return ranges::fold_left(
		data | views::transform([](size_t m) { return m / 3 - 2; }),
		0, std::plus());
}

result_t fuel_needed(result_t mass) {
    if ((mass / 3) <= 2) {
        return 0;
    }

   	result_t fuel = mass / 3 - 2;
    return fuel + fuel_needed(fuel) ;
}

result_t part2([[maybe_unused]] const data_t& data) {
	return ranges::fold_left(
		data | views::transform(fuel_needed),
		0, std::plus());
}

int main(int argc, char* argv[]) {
	bool verbose = false;

	int c;
	while ((c = getopt(argc, argv, "v")) != -1) {
		switch (c) {
			case 'v':
				verbose = !verbose;
				break;
			default:
				std::print(stderr, "ERROR: Unknown option \"{}\"\n", c);
				exit(1);
		}
	}

	argc -= optind;
	argv += optind;

	const char* input_file = argv[0];
	if (argc != 1) {
		std::print(stderr, "ERROR: No input file specified\n");
		exit(2);
	}

	auto start_time = chrono::high_resolution_clock::now();

	auto data = read_data(input_file);

	auto parse_complete = chrono::high_resolution_clock::now();
	auto parse_time = chrono::duration_cast<chrono::nanoseconds>(parse_complete - start_time);
	if (verbose) {
		print("{:>15} ({:>10} ns)\n", "parse", parse_time.count());
	}

	auto p1_start = chrono::high_resolution_clock::now();
	result_t p1_result = part1(data);
	auto p1_complete = chrono::high_resolution_clock::now();
	auto p1_time = chrono::duration_cast<chrono::nanoseconds>(p1_complete - p1_start);
	print("{:>15} ({:>10} ns){}", p1_result, p1_time.count(), verbose ? "\n" : "");

	auto p2_start = chrono::high_resolution_clock::now();
	result_t p2_result = part2(data);
	auto p2_complete = chrono::high_resolution_clock::now();
	auto p2_time = chrono::duration_cast<chrono::nanoseconds>(p2_complete - p2_start);
	print("{:>15} ({:>10} ns){}", p2_result, p2_time.count(), verbose ? "\n" : "");

	auto total_time = chrono::duration_cast<chrono::nanoseconds>(p2_complete - start_time);
	print("{:>15} ({:>10} ns)\n", "total", total_time.count());
}
