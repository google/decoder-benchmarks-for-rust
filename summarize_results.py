#!/usr/bin/env python3

# Copyright 2021 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import json
import os

def find_result_files(base_path):
    results = []
    for path, directories, files in os.walk(base_path):
        for file in files:
            if not path.endswith('new') or not file == 'estimates.json':
                continue
            results += [os.path.join(path, file)]
    return results

def parse_results_path(path):
    """Returns a pair of (benchmark group, benchmark) by parsing the path to
    the results file."""
    parts = path.split(os.path.sep)

    # Example path:
    #   target/criterion/Images/turbo/images_venice-500x750.jpg/new/estimates.json
    #   0      1         2      3     4                         5   6
    return (parts[3], parts[4])

def extract_result(path):
    with open(path) as file:
        results = json.load(file)
        return results['mean']['point_estimate']

def extract_results(base_path):
    result_files = find_result_files(base_path)

    results = {}
    for file in result_files:
        (group, test) = parse_results_path(file)
        
        if test not in results:
            results[test] = {}

        results[test][group] = extract_result(file)

    print_results(results, 'jpeg_decoder', 'turbo', 'jpeg-decoder', 'turbojpeg')
    print_results(results, 'png', 'spng', 'png', 'spng')

    print("Don't forget to update lines.svg")
    print("  cp target/criterion/jpegs/report/lines.svg .")

def print_results(results, group1, group2, group1name, group2name):
    # Markdown table header.
    print("| Test case | {0} (ms) | {1} (ms) | {0} / {1} |"
            .format(group1name, group2name))
    print("| :--- | ---: | ---: | ---: |")

    for test_name in results:
        if group1 not in results[test_name]:
            continue
        if group2 not in results[test_name]:
            continue

        result1 = results[test_name][group1]
        result2 = results[test_name][group2]
        print("| {} | {:.4} | {:.4} | {}% |".format(
            test_name,
            result1 / 1000000,
            result2 / 1000000,
            round(result1 * 100 / result2)))

    print()

if __name__ == '__main__':
    extract_results('target/criterion')
