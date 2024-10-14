/********************************************************************************
*   Program: Print
*
*   Purpose: Formats files for printing
*   Usage:  print [options] file(s)
*   Options:
*       -v          Produces verbose messages
*       -o<file>    Send output to a file (default=print.out)
*       -l<lines>   Sets number of lines per page (Default = 66)
****************************************************************************/
#include <stdio.h>
#include <stdlib.h>

int verbose=0;
char *out_file = "print.out";
char *program_name;
int line_max = 66;

/***********************************************************************
*   do_file -- dummy routine to handle a file
*   Parameter -- name -- Name of the file to print
*
************************************************************************/
void do_file(char *name){
    printf("Verbose %d Lines %d Input %s Output %s \n",verbose,line_max,name,out_file);
}

/*********************************************************************** 
*   Usage -- Tells the user how to use this program and exit
***********************************************************************/

void usage(void)
{
    fprintf(stderr, "Usage is %s [options] [file-list]\n",program_name);
    fprintf(stderr,"-v          Verbose\n");
    fprintf(stderr,"-l<number>  Number of lines\n");
    fprintf(stderr,"-o<name>    Set output filename\n");
    exit(8);
}
/**************************************************************************
*   example_13_10 -- basic parse flags passed in from command line
*   Parameters  argv -- number of arguments passed in
*               argc -- values passed in from the command line
**************************************************************************/
void example_13_12(int argc,char *argv[]){
program_name=argv[0]; /* program name */
/* Loop for each option */
while ((argc>1)&& (argv[1][0]=='-')){
switch(argv[1][1]){ /* argv[1][1] is the actual option character */
case 'v':   verbose=1;
break;
case 'o': out_file=&argv[1][2];
break;
case 'l': line_max=atoi(&argv[1][2]);
break;
default: fprintf(stderr,"Bad option %s \n",argv[1]);
usage();

}
++argv;
--argc;
}
/* at this point all the options have been processed.*/
if (argc==1) {

    do_file("print.in");
}else{
    while (argc>1){
        do_file(argv[1]);
        ++argv;
        --argc;
    }
}
} 
