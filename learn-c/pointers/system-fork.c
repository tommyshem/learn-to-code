#include <stdio.h>
#include <sys/types.h>
#include <unicase.h>

// fork() copy all program onto another proccess
int main(){
    pid_t pid1 = fork();
    printf("This pid = %d Forked 1 pid = %d\n",getpid(),pid1);
    pid_t pid2 = fork();
    printf("This pid = %d Forked 2 pid = %d\n",getpid(),pid2);
    pid_t pid3 = fork();
    printf("This pid = %d Forked 3 pid = %d\n",getpid(),pid3);
    printf("Pro End %d\n",getpid());
    wait();
    return 0;
}