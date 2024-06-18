#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>

// 定义大整数的结构
typedef struct {
    char *digits;
    int length;
} BigInt;

// 创建大整数
BigInt* createBigInt(int length) {
    BigInt *bigInt = (BigInt*)malloc(sizeof(BigInt));
    bigInt->digits = (char*)malloc(length + 1);
    memset(bigInt->digits, '0', length);
    bigInt->digits[length] = '\0';
    bigInt->length = length;
    return bigInt;
}

// 释放大整数
void freeBigInt(BigInt *bigInt) {
    free(bigInt->digits);
    free(bigInt);
}

// 复制大整数
void copyBigInt(BigInt *dest, BigInt *src) {
    memcpy(dest->digits, src->digits, src->length);
}

// 打印大整数
void printBigInt(BigInt *bigInt) {
    int start = 0;
    while (start < bigInt->length && bigInt->digits[start] == '0') {
        start++;
    }
    if (start == bigInt->length) {
        printf("0\n");
    } else {
        printf("%s\n", bigInt->digits + start);
    }
}

// 大整数相加
void addBigInt(BigInt *result, BigInt *a, BigInt *b) {
    int carry = 0;
    for (int i = a->length - 1; i >= 0; i--) {
        int sum = (a->digits[i] - '0') + (b->digits[i] - '0') + carry;
        result->digits[i] = (sum % 10) + '0';
        carry = sum / 10;
    }
}

void fibonacci(int n) {
    struct timeval start_time, end_time;
    gettimeofday(&start_time, NULL);
    printf("Start Fibonacci Calculation: %ld.%06ld\n", start_time.tv_sec, start_time.tv_usec);

    BigInt *a = createBigInt(100000);
    BigInt *b = createBigInt(100000);
    BigInt *temp = createBigInt(100000);

    b->digits[b->length - 1] = '1';

    for (int i = 2; i <= n; ++i) {
        addBigInt(temp, a, b);
        copyBigInt(a, b);
        copyBigInt(b, temp);
    }

    gettimeofday(&end_time, NULL);
    printf("End Fibonacci Calculation: %ld.%06ld\n", end_time.tv_sec, end_time.tv_usec);

    freeBigInt(a);
    freeBigInt(b);
    freeBigInt(temp);
}

int main() {
    int n = 100000;
    fibonacci(n);
    return 0;
}

