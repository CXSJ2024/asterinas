#!/bin/sh

test_read_performance() {
    filename=$1
    filesize=$2
    blocksize=$3

    total_time=0
    max_time=0
    min_time=1000000000

    echo "Testing file size: $filesize, block size: $blocksize"

    for i in 1 2 3 4 5 6 7 8 9 10; do
        output=$(./read_test "$filename" "$filesize" "$blocksize")
        echo "$output"
        read_time=$(echo "$output" | grep "Read duration" | awk '{print $3}')
        total_time=$((total_time + read_time))
        sleep 1
    done

    avg_time=$(( total_time / 10 ))
    echo "Average read time for file size $filesize with block size $blocksize: $avg_time microseconds"
    echo "========"
}

combinations="16M 64K
16M 128K
16M 256K
16M 512K
16M 1M
16M 2M
16M 4M
16M 8M
1M 1M
36M 1M
56M 1M
64M 1M
"

echo "$combinations" | while IFS=" " read -r filesize blocksize; do
    if [ -n "$filesize" ] && [ -n "$blocksize" ]; then
        test_read_performance "random_file_${filesize}" "$filesize" "$blocksize"
        # echo "random_file_${filesize}"
        # echo "${blocksize}"
    fi
done
