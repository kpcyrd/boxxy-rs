extern void boxxy_run();

extern void* boxxy_init();
extern void boxxy_with(void*, char*, int (*f)(int, char**));
extern void boxxy_run_at(void*);
extern int boxxy_exec_once(void*, char*);
extern void boxxy_free(void*);
