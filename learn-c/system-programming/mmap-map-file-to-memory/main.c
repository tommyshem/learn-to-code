#include <sys/mman.h>
#include <fcntl.h>
#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <sys/types.h>
#include <unistd.h>

// define a record
struct record {
    int id;
    char name[80];
};

int main (){
size_t size;
struct record *records;
int fd = open ("test_file",O_RDWR);
// error checking if file opened
if (fd < 0 ){
    perror("error");
    exit(1);
}
size = lseek(fd,0,SEEK_END); // get size of the file

records =(struct record *) mmap (NULL,size,PROT_READ | PROT_WRITE,MAP_PRIVATE,fd,0);

records[1].id = 99; // update record 1

msync(records,size,MS_SYNC); // sync back to the file

return 0;
}