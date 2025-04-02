#! /bin/bash
valgrind --tool=callgrind mu-sys -q "$1" 2>&1 || true
callgrind_annotate --auto=yes callgrind.out.*
rm callgrind.out.*
