pub type thrd_t = ::c_ulong;
pub type once_flag = ::c_int;
pub type tss_t = ::c_uint;

s! {
    pub struct cnd_t {
        pub _c_head: *mut ::c_void,
        pub _c_clock: ::c_int,
        pub _c_tail: *mut ::c_void,
        pub _c_lock: ::c_int
    }

    pub struct mtx_t {
        pub __i: [::c_int; 1],
    }
}

pub const thrd_success: ::c_int = 0;
pub const thrd_busy : ::c_int = 1;
pub const thrd_error : ::c_int = 2;
pub const thrd_nomem : ::c_int = 3;
pub const thrd_timedout : ::c_int = 4;

pub const mtx_plain: ::c_int = 0;
pub const mtx_recursive: ::c_int = 1;
pub const mtx_timed: ::c_int = 2;

pub const MTX_INIT : ::mtx_t = ::mtx_t {
    __i: [ 0 ],
};

pub const CND_INIT : ::cnd_t = ::cnd_t {
    _c_head: 0usize as *mut _,
    _c_clock: 0,
    _c_tail: 0usize as *mut _,
    _c_lock: 0,
};

pub const ONCE_FLAG_INIT : ::once_flag = 0;

extern {
    pub fn thrd_create(thr: *mut ::thrd_t,
                       start: extern fn(*mut ::c_void) -> ::c_int,
                       arg: *mut ::c_void) -> ::c_int;
    pub fn thrd_create_with_name(thr: *mut ::thrd_t,
                                 start: extern fn(*mut ::c_void) -> ::c_int,
                                 arg: *mut ::c_void,
                                 name: *const ::c_char) -> ::c_int;
    pub fn thrd_exit(status: ::c_int);

    pub fn thrd_detach(thr: ::thrd_t) -> ::c_int;
    pub fn thrd_join(thr: ::thrd_t, res: *mut ::c_int) -> ::c_int;

    pub fn thrd_sleep(until: *const ::timespec, rem: *mut ::timespec) -> ::c_int;
    pub fn thrd_yield();

    pub fn thrd_current() -> ::thrd_t;

    pub fn call_once(flag: *mut ::once_flag, func: extern fn());

    pub fn mtx_init(mtx: *mut ::mtx_t, mtx_type: ::c_int) -> ::c_int;
    pub fn mtx_destroy(mtx: *mut ::mtx_t);

    pub fn mtx_lock(mtx: *mut ::mtx_t) -> ::c_int;
    pub fn mtx_timedlock(mtx: *mut ::mtx_t, until: *const ::timespec) -> ::c_int;
    pub fn mtx_trylock(mtx: *mut ::mtx_t) -> ::c_int;
    pub fn mtx_unlock(mtx: *mut ::mtx_t) -> ::c_int;

    pub fn cnd_init(cond: *mut ::cnd_t) -> ::c_int;
    pub fn cnd_destroy(cond: *mut ::cnd_t);

    pub fn cnd_signal(cond: *mut ::cnd_t) -> ::c_int;
    pub fn cnd_broadcast(cond: *mut ::cnd_t) -> ::c_int;

    pub fn cnd_wait(cond: *mut ::cnd_t, mtx: *mut ::mtx_t) -> ::c_int;
    pub fn cnd_timedwait(cond: *mut ::cnd_t,
                         mtx: *mut ::mtx_t,
                         when: *const ::timespec) -> ::c_int;

    pub fn tss_create(key: *mut ::tss_t, dtor: extern fn(*mut ::c_void)) -> ::c_int;
    pub fn tss_delete(key: ::tss_t);

    pub fn tss_get(key: ::tss_t) -> *mut ::c_void;
    pub fn tss_set(key: ::tss_t, val: *mut ::c_void) -> ::c_int;
}
