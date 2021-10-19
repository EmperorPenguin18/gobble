#!/bin/sh

echo "Compiling program"
cargo build
echo "Compile complete"
echo

echo "Testing graphical program"
./target/debug/gobble $TERM
echo "Test complete"
echo

echo "Testing cli program"
./target/debug/gobble echo testing
echo "Test complete"
echo

echo "Testing incorrect arguments"
./target/debug/gobble ls doesnotexist
echo "Test complete"
echo

echo "Testing non-existent command"
./target/debug/gobble ehco testing
echo "Test complete"
echo

echo "Testing no arguments"
./target/debug/gobble
echo "Test complete"
echo

echo "Testing simulated Wayland"
WAYLAND_DISPLAY=test ./target/debug/gobble $TERM
echo "Test complete"
echo

echo "Performance test"
echo "Gobble:"
time for I in {0..1000}; do ./target/debug/gobble printf ' ' 2>/dev/null; done
echo "Devour:"
time for I in {0..1000}; do devour printf ' ' 2>/dev/null; done
echo "Test complete"
echo

echo "Overlap mode test"
./target/debug/gobble -o $TERM
echo "Test complete"
echo

echo "Testing multiple flags + wrong flags"
./target/debug/gobble -v -l
echo "Test complete"
echo

echo "Testing version flag"
./target/debug/gobble -v
echo "Test complete"
echo

echo "Testing flag in the wrong spot"
./target/debug/gobble echo -h
echo "Test complete"
echo

echo "All tests completed"
exit 0
