#ifndef SHARED_H
#define SHARED_H
#include "shm_bm.h"
static int ret_nobjs = 16;
static size_t ret_objsz = sizeof(int);
struct shm_bm{
    void * shm;
    shm_bm_objid_t objid;
};
SHM_BM_INTERFACE_CREATE(add_return, ret_objsz, ret_nobjs);
static int args_nobjs = 16;
struct add_args{
    int a;
	int b;
};
static size_t args_objsz = sizeof(struct add_args);
SHM_BM_INTERFACE_CREATE(add_params, args_objsz, args_nobjs);
struct shm_bm *add_s(void * shm, shm_bm_objid_t objid);
int add_c(int a,int b);
#endif