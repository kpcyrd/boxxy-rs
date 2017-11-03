#include <stdio.h>
#include "boxxy.h"

int ohai(int argc, char** argv) {
    printf("[+] ohai from C again!\n");

    for(int i=0; i<argc; i++) {
        printf("\t%d => %s\n", i, argv[i]);
    }

    return 0;
}

int main() {

    // if you don't need function pointers, just drop into a shell with
    // printf("[*] calling boxxy_run (%p) ...\n", (void *) boxxy_run);
    // boxxy_run();

    void* boxxy = boxxy_init();
    boxxy_with(boxxy, "ohai", ohai);
    printf("[*] calling boxxy (%p) ...\n", (void *) boxxy);

    // execute commands automatically:
    // boxxy_exec_once_at(boxxy, "id\n");

    boxxy_run_at(boxxy);
    boxxy_free(boxxy);

    printf("[+] done!\n");

    return 0;
}
