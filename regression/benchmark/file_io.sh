#!/bin/sh

# 参数设置
TEST_FILE="testfile"
BLOCK_SIZES="128K 256K 512K 1M 2M 4M"  # 单文件块大小集合
FILE_SIZE="128M"   # 测试文件大小
WRITE_COUNT=10   # 写入次数
READ_COUNT=10    # 读取次数

# 创建测试文件
echo "Creating test file of size $FILE_SIZE..."
dd if=/dev/zero of=$TEST_FILE bs=$FILE_SIZE count=1

# 运行不同单文件块大小的测试
for BLOCK_SIZE in $BLOCK_SIZES; do
  echo "Testing with block size $BLOCK_SIZE..."

  # 写入测试
  echo "Running write test..."
  START_TIME=$(date +%s%N)
  for i in $(seq 1 $WRITE_COUNT); do
    dd if=/dev/zero of=$TEST_FILE bs=$BLOCK_SIZE count=$(echo $FILE_SIZE | sed 's/K//;s/M//;s/G//' | awk '{print $1 / '$BLOCK_SIZE'}') oflag=direct 2>/dev/null
  done
  END_TIME=$(date +%s%N)
  WRITE_DURATION=$(( (END_TIME - START_TIME) / 1000000 ))
  echo "Write duration: $WRITE_DURATION ms"

  # 读取测试
  echo "Running read test..."
  START_TIME=$(date +%s%N)
  for i in $(seq 1 $READ_COUNT); do
    dd if=$TEST_FILE of=/dev/null bs=$BLOCK_SIZE count=$(echo $FILE_SIZE | sed 's/K//;s/M//;s/G//' | awk '{print $1 / '$BLOCK_SIZE'}') iflag=direct 2>/dev/null
  done
  END_TIME=$(date +%s%N)
  READ_DURATION=$(( (END_TIME - START_TIME) / 1000000 ))
  echo "Read duration: $READ_DURATION ms"

  echo "Results for block size $BLOCK_SIZE:"
  echo "Write duration: $WRITE_DURATION ms"
  echo "Read duration: $READ_DURATION ms"
  echo "-----------------------------"
done

# 清理测试文件
echo "Cleaning up..."
rm -f $TEST_FILE

echo "Test completed."
