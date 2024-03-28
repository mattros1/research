#ifndef SHM_BM_H
#define SHM_BM_H

#include <stdlib.h>
#include <string.h>
#include <limits.h>

/**
 * This is a version of the shared memory header file that operates locally
 * using malloc. It provides a slab-like memory allocator interface to allocate 
 * fixed-size blocks of memory.
 */

#define SHM_BM_ALIGN (1 << 24)

typedef void *        shm_bm_t;
typedef unsigned int  shm_bm_objid_t;
void * shm;

#define SHM_BM_REFC(shm, nobj) ((unsigned char *)((unsigned char *)shm + nobj))
#define SHM_BM_DATA(shm, nobj) ((unsigned char *)(SHM_BM_REFC(shm, nobj) + nobj))

#define SHM_BM_BITMAP_BLOCK (sizeof (unsigned int) * CHAR_BIT)


static inline shm_bm_t 
__shm_bm_create(void * mem, size_t memsz, size_t objsz, unsigned int nobj)
{
    //if ((word_t)mem % SHM_BM_ALIGN != 0) return 0;
	//if (memsz < __shm_bm_size(objsz, nobj)) return 0;
    shm=mem;
    return (shm_bm_t)mem;
}

static inline void
__shm_bm_init(shm_bm_t shm, size_t objsz, unsigned int nobj)
{
}

static inline void * 
__shm_bm_alloc(shm_bm_t shm, shm_bm_objid_t *objid, size_t objsz, unsigned int nobj)
{
    return shm; // No free space
}

static inline void *   
__shm_bm_take(shm_bm_t shm, shm_bm_objid_t objid, size_t objsz, unsigned int nobj)
{
    return shm;
}

static void
__shm_bm_ptr_free(void *ptr, size_t objsz, unsigned int nobj)
{
   free(shm);
}

static shm_bm_objid_t
__shm_bm_get_objid(void *ptr, size_t objsz, unsigned int nobj)
{
    return (shm_bm_objid_t)(((unsigned char *)ptr - SHM_BM_DATA(ptr, nobj)) / objsz);
}

#define __SHM_BM_DEFINE_FCNS(name)                                                          \                                   
    static inline shm_bm_t shm_bm_create_##name(void);                                      \
    static inline void     shm_bm_init_##name(shm_bm_t shm);                                \
    static inline void *   shm_bm_alloc_##name(shm_bm_t shm, shm_bm_objid_t *objid);        \
    static inline void *   shm_bm_take_##name(shm_bm_t shm, shm_bm_objid_t objid);          \
    static inline void *   shm_bm_borrow_##name(shm_bm_t shm, shm_bm_objid_t objid);        \
    static inline void     shm_bm_free_##name(void *ptr);                                   \
    static inline shm_bm_objid_t shm_bm_get_objid_##name(void *ptr);

#define SHM_BM_INTERFACE_CREATE(name, objsz, nobjs)                                         \
    __SHM_BM_DEFINE_FCNS(name)                                                              \                                                                                       \
    static inline shm_bm_t                                                                 \
    shm_bm_create_##name(void)                                                              \
    {                                                                                       \
        return __shm_bm_create(objsz, nobjs);                                              \
    }                                                                                       \
    static inline void                                                                     \
    shm_bm_init_##name(shm_bm_t shm)                                                       \
    {                                                                                       \
        __shm_bm_init(shm, objsz, nobjs);                                                  \
    }                                                                                       \
    static inline void *                                                                   \
    shm_bm_alloc_##name(shm_bm_t shm, shm_bm_objid_t *objid)                               \
    {                                                                                       \
        return __shm_bm_alloc(shm, objid, objsz, nobjs);                                   \
    }                                                                                       \
    static inline void *                                                                   \
    shm_bm_take_##name(shm_bm_t shm, shm_bm_objid_t objid)                                 \
    {                                                                                       \
        return __shm_bm_take(shm, objid, objsz, nobjs);                                    \
    }                                                                                       \
    static inline void *                                                                   \
    shm_bm_borrow_##name(shm_bm_t shm, shm_bm_objid_t objid)                               \
    {                                                                                       \
        return __shm_bm_take(shm, objid, objsz, nobjs);                                    \
    }                                                                                       \                                                                                     \
    static inline void                                                                     \
    shm_bm_free_##name(void *ptr)                                                          \
    {                                                                                       \
        __shm_bm_ptr_free(ptr, objsz, nobjs);                                              \
    }                                                                                       \
    static inline shm_bm_objid_t                                                           \
    shm_bm_get_objid_##name(void *ptr)                                                     \
    {                                                                                       \
        return __shm_bm_get_objid(ptr, objsz, nobjs);                                      \
    }

#endif
