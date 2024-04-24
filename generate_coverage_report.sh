#!/bin/bash

# just run cargo kcov instead

# this script is used to generate the coverage report
# the entire output of the testing step will be saved in test.output
# it is recommended to only run this script after all tests are passing

# run the tests
cargo test &> test.output

# get list of possible binary names
binary_name=$(grep -o 'target/debug/deps/\(.*\)' test.output | cut -d ')' -f 1)

# print the first binary name
echo $binary_name

# generate the coverage report
# kcov --exclude-pattern='/.cargo/' --exclude-path='/usr/include/' ./coverage/ ./$binary_name
kcov --exclude-path='/usr/include/' ./coverage/ ./$binary_name