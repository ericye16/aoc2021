processes=(
    ./target/release/d1
    ./target/release/d2
    ./target/release/d3
    ./target/release/d4
    ./target/release/d5
    ./target/release/d6
    ./target/release/d7
    ./target/release/d8
    ./target/release/d9
    ./target/release/d10
    ./target/release/d11
    ./target/release/d12
    ./target/release/d13
    ./target/release/d14
    ./target/release/d15
    ./target/release/d16
    ./target/release/d17
    ./target/release/d18
    ./target/release/d19v2
    ./target/release/d20
    ./target/release/d21
    ./bazel-bin/cpp/p22
    ./bazel-bin/cpp/p23
    ./target/release/d24
    ./target/release/d25
)

TIMEFMT=$'%uU'

for process in ${processes[@]}; do
    # warm-up run
    ${process} > /dev/null
    printf "Running ${process}, "
    time ${process} > /dev/null
done
