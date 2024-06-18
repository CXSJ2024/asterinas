#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/wait.h>

void get_current_time(char *label) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    printf("%s: %ld.%06ld\n", label, tv.tv_sec, tv.tv_usec);
    fflush(stdout);
}

int main() {
    const char *program = "./fibonacci";
    int num_runs = 10;

    for (int i = 0; i < num_runs; ++i) {
        pid_t pid = fork();
        if (pid == 0) { // Child process
            get_current_time("Before exec");

            execl(program, program, NULL);

            // If execl fails
            perror("execl failed");
            exit(EXIT_FAILURE);
        } else if (pid > 0) { // Parent process
            int status;
            waitpid(pid, &status, 0);
        } else { // fork failed
            perror("fork failed");
            exit(EXIT_FAILURE);
        }
    }

    return 0;
}

