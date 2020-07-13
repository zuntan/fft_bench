#!/bin/sh

LEN=180
BA=0

if [ -n "$2" ]; then
	LEN=$2
fi

if [ "$1" = "build_arm" -o "$1" = "BA" ]; then
	BA=1
fi

param=(
	" -l ${LEN} -s"
	" -l ${LEN} -s -w"
	" -l ${LEN}"
	" -l ${LEN} -w"
	" -l ${LEN} -6"
	" -l ${LEN} -6 -w"
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
echo "*** cargo build --release ***"
time cargo build --release

if [ ${BA} = "1" ]; then
	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf ***"
	time cargo build --target armv7-unknown-linux-gnueabihf

	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf --release ***"
	time cargo build --target armv7-unknown-linux-gnueabihf --release
fi

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
