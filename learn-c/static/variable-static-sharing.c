// compile with
// gcc variable-static-sharing-.c -o variable-static-sharing
#include <stdio.h>

static int eggs = 0;  // static variable only visable in this file.
int allEggs = 0; // global variable visable in all files

void up(){
    eggs += 10;
}

void down(){
    eggs -= 5;
}

int main(){
    up();
    up();
    down();
    printf("eggs = %d\n",eggs);
}
