#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <time.h>
#include <string.h>
#include <errno.h>

void error(const char *msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

size_t parse_size(const char *size_str) {
    size_t size = atol(size_str);
    char unit = size_str[strlen(size_str) - 1];

    switch (unit) {
        case 'K': size *= 1024; break;
        case 'M': size *= 1024 * 1024; break;
        case 'G': size *= 1024 * 1024 * 1024; break;
    }

    return size;
}

int main(int argc, char *argv[]) {
    if (argc != 4) {
        fprintf(stderr, "Usage: %s <target_file> <file_size[K|M|G]> <block_size[K|M|G]>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    const char *target_file = argv[1];
    size_t file_size = parse_size(argv[2]);
    size_t block_size = parse_size(argv[3]);

    int fd = open(target_file, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) {
        error("open");
    }

    char *buffer = malloc(block_size);
    if (!buffer) {
        error("malloc");
    }

    // 填充随机数据
    for (size_t i = 0; i < block_size; i++) {
        buffer[i] = rand() % 256;
    }

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);

    size_t total_bytes_written = 0;
    while (total_bytes_written < file_size) {
        ssize_t bytes_written = write(fd, buffer, block_size);
        if (bytes_written == -1) {
            error("write");
        }
        total_bytes_written += bytes_written;
    }

    clock_gettime(CLOCK_MONOTONIC, &end);
    long duration = (end.tv_sec - start.tv_sec) * 1000000 + (end.tv_nsec - start.tv_nsec) / 1000;
    printf("Write duration: %ld microseconds\n", duration);

    free(buffer);
    close(fd);

    return 0;
}
