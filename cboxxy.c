#include <stdio.h>
#include "boxxy.h"

extern void run_boxxy();

int main() {
    printf("[*] calling run_boxxy (%p) ...\n", (void *) run_boxxy);
    run_boxxy();
    printf("[+] done!\n");

    return 0;
}
