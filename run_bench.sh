#!/bin/sh

param=(
	" -l 180 -s"
	" -l 180 -s -w"
	" -l 180"
	" -l 180 -w"
	" -l 180 -6"
	" -l 180 -6 -w"
)

param_r=(
	" -- "
	" --release -- "
)

echo ""
echo "*** cargo clean ***"
time cargo clean

echo ""
echo "*** cargo check ***"
time cargo check

echo ""
echo "*** cargo build ***"
time cargo build

echo ""
echo "*** cargo build ***"
time cargo build --release

export RUST_LOG=info

for r in ${!param_r[@]}
do
	for p in ${!param[@]}
	do
		echo ""
		echo " *** cargo run " ${param_r[$r]} ${param[$p]} " ***"
		time cargo run ${param_r[$r]} ${param[$p]}
	done
done
