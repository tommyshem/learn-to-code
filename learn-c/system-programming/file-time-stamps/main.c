// simple use of stat to get the time stamp
// compile with clang -o listfile ./main.c
// you need file called 'test_file' in the directory you are running the program to see the permissions 

#include <stdio.h>
#include <sys/stat.h>
#include <time.h>
#include <errno.h>
#include <stdlib.h>

int main(int argc, char * argv[]){  // get args passed in
    struct stat sb; // the stat buffer
        // from the standard library 
        //        struct stat {
        //        dev_t     st_dev;         /* ID of device containing file */
        //        ino_t     st_ino;         /* Inode number */
        //        mode_t    st_mode;        /* File type and mode */
        //        nlink_t   st_nlink;       /* Number of hard links */
        //        uid_t     st_uid;         /* User ID of owner */
        //        gid_t     st_gid;         /* Group ID of owner */
        //        dev_t     st_rdev;        /* Device ID (if special file) */
        //        off_t     st_size;        /* Total size, in bytes */
        //        blksize_t st_blksize;     /* Block size for filesystem I/O */
        //        blkcnt_t  st_blocks;      /* Number of 512B blocks allocated */

        //        /* Since Linux 2.6, the kernel supports nanosecond
        //           precision for the following timestamp fields.
        //           For the details before Linux 2.6, see NOTES. */

        //        struct timespec st_atim;  /* Time of last access */
        //        struct timespec st_mtim;  /* Time of last modification */
        //        struct timespec st_ctim;  /* Time of last status change */

        //    #define st_atime st_atim.tv_sec      /* Backward compatibility */
        //    #define st_mtime st_mtim.tv_sec
        //    #define st_ctime st_ctim.tv_sec
        //    };
    // check only 1 argument is passed in
    if (argc != 2){ // args count must be 2 - first arg the the name of the command and the second is the first arg passed in
    fprintf(stderr, "usage: listfile <filename>\n");
    exit(1);
    }
    // get info from filename or directory passsed in
    if (stat(argv[1],&sb) < 0){
    // error checking 
        perror(argv[1]); // print error
        exit(2); // exit program
    }
    // file time
    printf("Time Stamp\n");
    printf("last time accessed: %s",ctime(&sb.st_atime));
    printf("last time modified: %s",ctime(&sb.st_mtime));
    printf("last time changed: %s",ctime(&sb.st_ctime));
    // file permissions
    // used for getting the device type
    char *filetype[] = {"unknown","FIFO","character device","unknown","directory","unknown","block device","unknown","file","unknown","symlink","unknown","socket"};
    printf("file type: %s\n",filetype[(sb.st_mode >> 12) & 017]); //bit masking to get mode
    // print permissions out
    printf("permissions: \nUser: %c%c%c \nGroup: %c%c%c\nOther users: %c%c%c\n",
    (sb.st_mode & S_IRUSR)  ? 'r' : '-', // turnery operator is the ? true : false
    (sb.st_mode & S_IWUSR)  ? 'w' : '-', // bit wise and masking if true print r else false print -
    (sb.st_mode & S_IXUSR)  ? 'x' : '-', 
    (sb.st_mode & S_IRGRP)  ? 'r' : '-',
    (sb.st_mode & S_IWGRP)  ? 'w' : '-',
    (sb.st_mode & S_IXGRP)  ? 'x' : '-',
    (sb.st_mode & S_IROTH)  ? 'r' : '-',
    (sb.st_mode & S_IWOTH)  ? 'w' : '-',
    (sb.st_mode & S_IWOTH)  ? 'x' : '-'
    );

}