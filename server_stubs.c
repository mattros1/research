#include "shared.h" 
#include "server.h"
struct shm_bm * add_s(void * shm, shm_bm_objid_t objid){
    struct add_args params = *(struct add_args*)shm_bm_take_add_params(shm,objid);
    shm_bm_free_add_params(shm);
    int ret = add(params.a,params.b);
    void * mem = calloc(sizeof(int),ret_nobjs);
    size_t memsz = ret_nobjs*ret_objsz;
    void * ret_shm = shm_bm_create_add_return(mem,memsz);
    shm_bm_init_add_return(ret_shm);
    shm_bm_objid_t ret_objid;
    void * obj;
    obj = shm_bm_alloc_add_return(ret_shm,&ret_objid);
    *(int*)obj = ret;
    return &((struct shm_bm ){ret_shm,ret_objid});
}