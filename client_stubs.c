#include "shared.h" 
int add_c(int a,int b){
    struct add_args params=(struct add_args){a,b};
    //TODO: ensure proper allignment
    void * mem = calloc(args_nobjs,args_objsz);
    size_t memsz = args_nobjs*args_objsz;
    void * shm = shm_bm_create_add_params(mem,memsz);
    shm_bm_init_add_params(shm);
    shm_bm_objid_t objid;
    void * obj;
    obj = shm_bm_alloc_add_params(shm,&objid);
    *(struct add_args*)obj = params;
    struct shm_bm * ret_shm = add_s(shm,objid);
    return *(int*)shm_bm_take_add_return(ret_shm->shm,ret_shm->objid);
}
