#!/bin/sh

TIME_CMD=`which time`
export TIME="\nTIME R:%e S:%S U:%U P:%P CMD:%C"

echo ""
echo "*** args ***"
echo $0 $@

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
	" -l ${LEN} -s -w "
	" -l ${LEN}"
	" -l ${LEN} -b"
	" -l ${LEN} -w"
	" -l ${LEN} -w -b -x"
	" -l ${LEN} -6"
	" -l ${LEN} -6 -b"
	" -l ${LEN} -6 -w"
	" -l ${LEN} -6 -w -b -x"
)

param_r=(
	" -- "
	" --release -- "
)

echo ""
echo "*** uname -a ***"
uname -a


echo ""
echo "*** cargo clean ***"
${TIME_CMD} cargo clean

echo ""
echo "*** cargo check 1 ***"
${TIME_CMD} cargo check

echo ""
echo "*** cargo check 2 ***"
touch src/main.rs
${TIME_CMD} cargo check

echo ""
echo "*** cargo build 1 ***"
${TIME_CMD} cargo build

echo ""
echo "*** cargo build 2 ***"
touch src/main.rs
${TIME_CMD} cargo build

echo ""
echo "*** cargo build --release 1 ***"
${TIME_CMD} cargo build --release

echo ""
echo "*** cargo build --release 2 ***"
touch src/main.rs
${TIME_CMD} cargo build --release

if [ ${BA} = "1" ]; then
	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf 1 ***"
	${TIME_CMD} cargo build --target armv7-unknown-linux-gnueabihf

	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf 2 ***"
	touch src/main.rs
	${TIME_CMD} cargo build --target armv7-unknown-linux-gnueabihf

	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf --release 1 ***"
	${TIME_CMD} cargo build --target armv7-unknown-linux-gnueabihf --release

	echo ""
	echo "*** cargo build --target armv7-unknown-linux-gnueabihf --release 2 ***"
	touch src/main.rs
	${TIME_CMD} cargo build --target armv7-unknown-linux-gnueabihf --release
fi

export RUST_LOG=info

for r in ${!param_r[@]}
do
	for p in ${!param[@]}
	do
		echo ""
		echo "*** cargo run " ${param_r[$r]} ${param[$p]} " ***"
		${TIME_CMD} cargo run ${param_r[$r]} ${param[$p]}
	done
done

echo ""
echo "*** cpu ***"
cat /proc/cpuinfo

echo ""
echo "*** mem ***"
free -h
echo ""
cat /proc/meminfo

