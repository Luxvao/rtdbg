#include <stdio.h>
#include <unistd.h>
 
int main(void) {
    int i = 0;
    while (1) {
        printf("tick %d (pid %d)\n", i++, getpid());
        fflush(stdout);
        sleep(5);
    }
    return 0;
}
