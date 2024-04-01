#include "shm_bm.h"
#include "header.h" 
int nobjs = 16;
struct add_args{
    int a;
	int b;
};
size_t objsz = sizeof(struct add_args);
SHM_BM_INTERFACE_CREATE(addobj, objsz, nobjs);

int add_s(void * shm, shm_bm_objid_t objid){
    struct add_args params = *(struct add_args*)shm_bm_take_addobj(shm,objid);
    shm_bm_free_addobj(shm);
    return add(params.a,params.b);
}
int add_c(int a,int b){
    struct add_args params=(struct add_args){a,b};
    //TODO: ensure proper allignment
    void * mem = calloc(nobjs,objsz);
    size_t memsz = nobjs*objsz;
    void * shm = shm_bm_create_addobj(mem,memsz);
    shm_bm_init_addobj(shm);
    shm_bm_objid_t objid;
    void * obj;
    obj = shm_bm_alloc_addobj(shm,&objid);
    *(struct add_args*)obj = params;
    return add_s(shm,objid);
}