/* Input members.
     These are initialized before call to gdbm_recover.  The flags argument
     specifies which of them are initialized. */
/* Output members.
     The gdbm_recover function fills these before returning. */
/* Default settings */
/* errfun is initialized */
/* max_failed_keys is initialized */
/* max_failed_buckets is initialized */
/* max_failures is initialized */
/* Keep backup copy of the
						original database on success */
/* Force recovery by skipping the
						check pass */
/* This one was never used and will be removed in the future */

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![crate_type = "staticlib"]

extern crate c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
pub extern fn add(input: i32) -> i32 {
    input * 2
}


extern "C" {

    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn abort() -> !;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getuid() -> __uid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: off_t) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    //fn _gdbm_end_update(_: GDBM_FILE) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;  
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
  
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;

    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;

    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );

    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;

    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;

    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;

    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;    
  
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;


    
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;

    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    //fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    //fn gdbm_db_strerror(dbf: GDBM_FILE) -> *const libc::c_char;
    // fn dcgettext(
    //     __domainname: *const libc::c_char,
    //     __msgid: *const libc::c_char,
    //     __category: libc::c_int,
    // ) -> *mut libc::c_char;
    //fn _gdbm_fatal(_: GDBM_FILE, _: *const libc::c_char);
    //fn _gdbm_mapped_lseek(_: GDBM_FILE, _: off_t, _: libc::c_int) -> off_t;

    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;

    fn msync(
        __addr: *mut libc::c_void,
        __len: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int; 
    //fn _gdbm_full_write(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
    //fn _gdbm_full_read(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;

}

pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const cache_failure: C2RustUnnamed = 2;
pub const cache_found: C2RustUnnamed = 0;
pub const cache_new: C2RustUnnamed = 1;
//pub const GDBM_BACKUP_FAILED: C2RustUnnamed = 30;	
pub const GDBM_BAD_AVAIL: C2RustUnnamed = 34;
pub const GDBM_BAD_BUCKET: C2RustUnnamed = 32;
pub const GDBM_BAD_DIR_ENTRY: C2RustUnnamed = 36;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed = 22;
pub const GDBM_BAD_HASH_ENTRY: C2RustUnnamed = 41;
pub const GDBM_BAD_HASH_TABLE: C2RustUnnamed = 35;
pub const GDBM_BAD_HEADER: C2RustUnnamed = 33;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed = 7;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed = 23;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed = 2;
pub const GDBM_BUCKET_CACHE_CORRUPTED: C2RustUnnamed = 40;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed = 21;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed = 17;
pub const GDBM_CANT_BE_READER: C2RustUnnamed = 9;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed = 10;
pub const GDBM_DIR_OVERFLOW: C2RustUnnamed = 31;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed = 8;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed = 27;
pub const GDBM_ERR_REALPATH: C2RustUnnamed = 43;
pub const GDBM_ERR_SNAPSHOT_CLONE: C2RustUnnamed = 42;
pub const GDBM_ERR_USAGE: C2RustUnnamed = 44;
pub const GDBM_FILE_CLOSE_ERROR: C2RustUnnamed = 37;
pub const GDBM_FILE_EOF: C2RustUnnamed = 25;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed = 3;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed = 5;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed = 24;
pub const GDBM_FILE_SYNC_ERROR: C2RustUnnamed = 38;
pub const GDBM_FILE_TRUNCATE_ERROR: C2RustUnnamed = 39;
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed = 4;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed = 18;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed = 15;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed = 18;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed = 1;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed = 29;
pub const GDBM_NO_DBNAME: C2RustUnnamed = 26;
pub const GDBM_NO_ERROR: C2RustUnnamed = 0;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed = 19;
pub const GDBM_OPT_BADVAL: C2RustUnnamed = 20;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed = 20;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed = 11;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed = 13;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed = 12;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed = 16;
pub const GDBM_SNAPSHOT_BAD: gdbm_latest_snapshot_status = 1;
pub const GDBM_SNAPSHOT_ERR: gdbm_latest_snapshot_status = 2;
pub const GDBM_SNAPSHOT_OK: gdbm_latest_snapshot_status = 0;
pub const GDBM_SNAPSHOT_SAME: gdbm_latest_snapshot_status = 3;
pub const GDBM_SNAPSHOT_SUSPICIOUS: gdbm_latest_snapshot_status = 4;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed = 14;
pub const LOCKING_FCNTL: C2RustUnnamed = 3;
pub const LOCKING_FLOCK: C2RustUnnamed = 1;
pub const LOCKING_LOCKF: C2RustUnnamed = 2;
pub const LOCKING_NONE: C2RustUnnamed = 0;
pub const GDBM_BACKUP_FAILED: C2RustUnnamed = 30;
pub type C2RustUnnamed = libc::c_uint;
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __uint32_t = libc::c_uint;
pub type gdbm_count_t = libc::c_ulonglong;
pub type gdbm_error = libc::c_int;
pub type GDBM_FILE = *mut gdbm_file_info;
pub type gdbm_latest_snapshot_status = libc::c_uint;
pub type gdbm_recovery = gdbm_recovery_s;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type uid_t = __uid_t;
pub type uint32_t = __uint32_t;
pub type blksize_t = __blksize_t;
pub type FILE = _IO_FILE;

pub type setopt_handler = Option::<
    unsafe extern "C" fn(GDBM_FILE, *mut libc::c_void, libc::c_int) -> libc::c_int,
>;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;


#[macro_use]
extern crate c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datum {
    pub dptr: *mut libc::c_char,
    pub dsize: libc::c_int,
}
#[repr(C, align(1))]
#[derive(BitfieldStruct)]
pub struct gdbm_file_info {
    pub name: *mut libc::c_char,
    #[bitfield(name = "read_write", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "fast_write", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "central_free", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "coalesce_blocks", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "file_locking", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "memory_mapping", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "cloexec", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "need_recovery", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "cache_auto", ty = "libc::c_uint", bits = "9..=9")]
    pub read_write_fast_write_central_free_coalesce_blocks_file_locking_memory_mapping_cloexec_need_recovery_cache_auto: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub last_error: gdbm_error,
    pub last_syserror: libc::c_int,
    pub last_errstr: *mut libc::c_char,
    pub lock_type: C2RustUnnamed,
    pub fatal_err: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub desc: libc::c_int,
    pub header: *mut gdbm_file_header,
    pub avail: *mut avail_block,
    pub avail_size: size_t,
    pub xheader: *mut gdbm_ext_header,
    pub dir: *mut off_t,
    pub cache_bits: libc::c_int,
    pub cache_size: size_t,
    pub cache_num: size_t,
    pub cache: *mut *mut cache_elem,
    pub cache_mru: *mut cache_elem,
    pub cache_lru: *mut cache_elem,
    pub cache_avail: *mut cache_elem,
    pub bucket: *mut hash_bucket,
    pub bucket_dir: libc::c_int,
    pub cache_access_count: size_t,
    pub cache_hits: size_t,
    #[bitfield(name = "header_changed", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "directory_changed", ty = "libc::c_uint", bits = "1..=1")]
    pub header_changed_directory_changed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub file_size: off_t,
    pub mapped_size_max: size_t,
    pub mapped_region: *mut libc::c_void,
    pub mapped_size: size_t,
    pub mapped_pos: off_t,
    pub mapped_off: off_t,
    #[bitfield(name = "mmap_preread", ty = "libc::c_int", bits = "0..=0")]
    pub mmap_preread: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub eo: libc::c_int,
    pub snapfd: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_bucket {
    pub av_count: libc::c_int,
    pub bucket_avail: [avail_elem; 6],
    pub bucket_bits: libc::c_int,
    pub count: libc::c_int,
    pub h_table: [bucket_element; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_element {
    pub hash_value: libc::c_int,
    pub key_start: [libc::c_char; 4],
    pub data_pointer: off_t,
    pub key_size: libc::c_int,
    pub data_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avail_elem {
    pub av_size: libc::c_int,
    pub av_adr: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_elem {
    pub ca_adr: off_t,
    pub ca_changed: libc::c_char,
    pub ca_data: data_cache_elem,
    pub ca_prev: *mut cache_elem,
    pub ca_next: *mut cache_elem,
    pub ca_coll: *mut cache_elem,
    pub ca_hits: size_t,
    pub ca_bucket: [hash_bucket; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_cache_elem {
    pub hash_val: libc::c_int,
    pub data_size: libc::c_int,
    pub key_size: libc::c_int,
    pub dptr: *mut libc::c_char,
    pub dsize: size_t,
    pub elem_loc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdbm_ext_header {
    pub version: libc::c_int,
    pub numsync: libc::c_uint,
    pub pad: [libc::c_int; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avail_block {
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub next_block: off_t,
    pub av_table: [avail_elem; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdbm_file_header {
    pub header_magic: libc::c_int,
    pub block_size: libc::c_int,
    pub dir: off_t,
    pub dir_size: libc::c_int,
    pub dir_bits: libc::c_int,
    pub bucket_size: libc::c_int,
    pub bucket_elems: libc::c_int,
    pub next_block: off_t,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct dump_file {
    pub fp: *mut FILE,
    pub line: size_t,
    pub linebuf: *mut libc::c_char,
    pub lbsize: size_t,
    pub lblevel: size_t,
    pub buffer: *mut libc::c_char,
    pub bufsize: size_t,
    pub buflevel: size_t,
    pub parmc: size_t,
    pub data: [datbuf; 2],
    pub header: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdbm_file_extended_header {
    pub hdr: gdbm_file_header,
    pub ext: gdbm_ext_header,
    pub avail: avail_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdbm_file_standard_header {
    pub hdr: gdbm_file_header,
    pub avail: avail_block,
}


#[repr(C)]
pub struct gdbm_recovery_s {
    pub errfun: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
    >,
    pub data: *mut libc::c_void,
    pub max_failed_keys: size_t,
    pub max_failed_buckets: size_t,
    pub max_failures: size_t,
    pub recovered_keys: size_t,
    pub recovered_buckets: size_t,
    pub failed_keys: size_t,
    pub failed_buckets: size_t,
    pub duplicate_keys: size_t,
    pub backup_name: *mut libc::c_char,
}




pub struct off_map {
    pub map_base: *mut off_t,
    pub map_size: size_t,
    pub map_max: size_t,
}



#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdbm_cache_stat {
    pub adr: off_t,
    pub hits: size_t,
}


static mut gdbm_errno_storage: gdbm_error = GDBM_NO_ERROR as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn gdbm_errno_location() -> *mut libc::c_int {
    return &mut gdbm_errno_storage;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_set_errno(
    mut dbf: GDBM_FILE,
    mut ec: gdbm_error,
    mut fatal: libc::c_int,
) {
    if !dbf.is_null() {
        free((*dbf).last_errstr as *mut libc::c_void);
        let ref mut fresh0 = (*dbf).last_errstr;
        *fresh0 = 0 as *mut libc::c_char;
        (*dbf).last_error = ec;
        if *gdbm_syserr.as_ptr().offset(ec as isize) != 0 {
            (*dbf).last_syserror = *__errno_location();
        } else {
            (*dbf).last_syserror = 0 as libc::c_int;
        }
        (*dbf).set_need_recovery(fatal as libc::c_uint);
    }
    *gdbm_errno_location() = ec;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_last_errno(mut dbf: GDBM_FILE) -> gdbm_error {
    if dbf.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return (*dbf).last_error;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_last_syserr(mut dbf: GDBM_FILE) -> libc::c_int {
    if dbf.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return (*dbf).last_syserror;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_needs_recovery(mut dbf: GDBM_FILE) -> libc::c_int {
    if dbf.is_null() {
        return 0 as libc::c_int;
    }
    return (*dbf).need_recovery() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_clear_error(mut dbf: GDBM_FILE) {
    if !dbf.is_null() {
        (*dbf).last_error = GDBM_NO_ERROR as libc::c_int;
        (*dbf).last_syserror = 0 as libc::c_int;
        free((*dbf).last_errstr as *mut libc::c_void);
        let ref mut fresh1 = (*dbf).last_errstr;
        *fresh1 = 0 as *mut libc::c_char;
    }
}
#[no_mangle]
pub static mut gdbm_errlist: [*const libc::c_char; 45] = [
    b"No error\0" as *const u8 as *const libc::c_char,
    b"Malloc error\0" as *const u8 as *const libc::c_char,
    b"Block size error\0" as *const u8 as *const libc::c_char,
    b"File open error\0" as *const u8 as *const libc::c_char,
    b"File write error\0" as *const u8 as *const libc::c_char,
    b"File seek error\0" as *const u8 as *const libc::c_char,
    b"File read error\0" as *const u8 as *const libc::c_char,
    b"Bad magic number\0" as *const u8 as *const libc::c_char,
    b"Empty database\0" as *const u8 as *const libc::c_char,
    b"Can't be reader\0" as *const u8 as *const libc::c_char,
    b"Can't be writer\0" as *const u8 as *const libc::c_char,
    b"Reader can't delete\0" as *const u8 as *const libc::c_char,
    b"Reader can't store\0" as *const u8 as *const libc::c_char,
    b"Reader can't reorganize\0" as *const u8 as *const libc::c_char,
    b"Should not happen: unused error code\0" as *const u8 as *const libc::c_char,
    b"Item not found\0" as *const u8 as *const libc::c_char,
    b"Reorganize failed\0" as *const u8 as *const libc::c_char,
    b"Cannot replace\0" as *const u8 as *const libc::c_char,
    b"Malformed data\0" as *const u8 as *const libc::c_char,
    b"Option already set\0" as *const u8 as *const libc::c_char,
    b"Bad option value\0" as *const u8 as *const libc::c_char,
    b"Byte-swapped file\0" as *const u8 as *const libc::c_char,
    b"File header assumes wrong off_t size\0" as *const u8 as *const libc::c_char,
    b"Bad file flags\0" as *const u8 as *const libc::c_char,
    b"Cannot stat file\0" as *const u8 as *const libc::c_char,
    b"Unexpected end of file\0" as *const u8 as *const libc::c_char,
    b"Database name not given\0" as *const u8 as *const libc::c_char,
    b"Failed to restore file owner\0" as *const u8 as *const libc::c_char,
    b"Failed to restore file mode\0" as *const u8 as *const libc::c_char,
    b"Database needs recovery\0" as *const u8 as *const libc::c_char,
    b"Failed to create backup copy\0" as *const u8 as *const libc::c_char,
    b"Bucket directory overflow\0" as *const u8 as *const libc::c_char,
    b"Malformed bucket header\0" as *const u8 as *const libc::c_char,
    b"Malformed database file header\0" as *const u8 as *const libc::c_char,
    b"Malformed avail_block\0" as *const u8 as *const libc::c_char,
    b"Malformed hash table\0" as *const u8 as *const libc::c_char,
    b"Invalid directory entry\0" as *const u8 as *const libc::c_char,
    b"Error closing file\0" as *const u8 as *const libc::c_char,
    b"Error synchronizing file\0" as *const u8 as *const libc::c_char,
    b"Error truncating file\0" as *const u8 as *const libc::c_char,
    b"Bucket cache corrupted\0" as *const u8 as *const libc::c_char,
    b"Malformed bucket hash entry\0" as *const u8 as *const libc::c_char,
    b"Reflink failed\0" as *const u8 as *const libc::c_char,
    b"Failed to resolve real path name\0" as *const u8 as *const libc::c_char,
    b"Function usage error\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn gdbm_strerror(mut error: gdbm_error) -> *const libc::c_char {
    if error < 0 as libc::c_int || error > GDBM_ERR_USAGE as libc::c_int {
        error = GDBM_UNKNOWN_ERROR as libc::c_int;
    }
    return dcgettext(
        b"gdbm\0" as *const u8 as *const libc::c_char,
        gdbm_errlist[error as usize],
        5 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_db_strerror(mut dbf: GDBM_FILE) -> *const libc::c_char {
    if ((*dbf).last_errstr).is_null() {
        let mut errstr: *const libc::c_char = gdbm_strerror((*dbf).last_error);
        if (*dbf).last_syserror != 0 {
            let mut syserrstr: *const libc::c_char = strerror((*dbf).last_syserror);
            let mut len: size_t = (strlen(errstr))
                .wrapping_add(strlen(syserrstr))
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            let ref mut fresh2 = (*dbf).last_errstr;
            *fresh2 = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if ((*dbf).last_errstr).is_null() {
                return errstr;
            }
            strcpy((*dbf).last_errstr, errstr);
            strcat((*dbf).last_errstr, b": \0" as *const u8 as *const libc::c_char);
            strcat((*dbf).last_errstr, syserrstr);
        } else {
            return errstr
        }
    }
    return (*dbf).last_errstr;
}
#[no_mangle]
pub static mut gdbm_syserr: [libc::c_int; 45] = [
    0,
    0,
    0,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    0,
    0,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn gdbm_check_syserr(mut n: gdbm_error) -> libc::c_int {
    if n >= 0 as libc::c_int && n <= GDBM_ERR_USAGE as libc::c_int {
        return gdbm_syserr[n as usize];
    }
    return 0 as libc::c_int;
}


//gdbmfetch.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_fetch(mut dbf: GDBM_FILE, mut key: datum) -> datum {
    let mut return_val: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut elem_loc: libc::c_int = 0;
    let mut find_data: *mut libc::c_char = 0 as *mut libc::c_char;
    return_val.dptr = 0 as *mut libc::c_char;
    return_val.dsize = 0 as libc::c_int;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return return_val;
    }
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    elem_loc = _gdbm_findkey(dbf, key, &mut find_data, 0 as *mut libc::c_int);
    if elem_loc >= 0 as libc::c_int {
        return_val
            .dsize = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .data_size;
        if return_val.dsize == 0 as libc::c_int {
            return_val
                .dptr = malloc(1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        } else {
            return_val
                .dptr = malloc(return_val.dsize as libc::c_ulong) as *mut libc::c_char;
        }
        if (return_val.dptr).is_null() {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            return return_val;
        }
        memcpy(
            return_val.dptr as *mut libc::c_void,
            find_data as *const libc::c_void,
            return_val.dsize as libc::c_ulong,
        );
    }
    return return_val;
}

//gdbmdelete.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_delete(mut dbf: GDBM_FILE, mut key: datum) -> libc::c_int {
    let mut elem_loc: libc::c_int = 0;
    let mut last_loc: libc::c_int = 0;
    let mut home: libc::c_int = 0;
    let mut elem: bucket_element = bucket_element {
        hash_value: 0,
        key_start: [0; 4],
        data_pointer: 0,
        key_size: 0,
        data_size: 0,
    };
    let mut free_adr: off_t = 0;
    let mut free_size: libc::c_int = 0;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        gdbm_set_errno(dbf, GDBM_READER_CANT_DELETE as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    elem_loc = _gdbm_findkey(
        dbf,
        key,
        0 as *mut *mut libc::c_char,
        0 as *mut libc::c_int,
    );
    if elem_loc == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    elem = *((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize);
    (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .hash_value = -(1 as libc::c_int);
    let ref mut fresh0 = (*(*dbf).bucket).count;
    *fresh0 -= 1;
    last_loc = elem_loc;
    elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
    while elem_loc != last_loc
        && (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value != -(1 as libc::c_int)
    {
        home = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value % (*(*dbf).header).bucket_elems;
        if last_loc < elem_loc && (home <= last_loc || home > elem_loc)
            || last_loc > elem_loc && home <= last_loc && home > elem_loc
        {
            *((*(*dbf).bucket).h_table)
                .as_mut_ptr()
                .offset(
                    last_loc as isize,
                ) = *((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize);
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .hash_value = -(1 as libc::c_int);
            last_loc = elem_loc;
        }
        elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
    }
    free_adr = elem.data_pointer;
    free_size = elem.key_size + elem.data_size;
    if _gdbm_free(dbf, free_adr, free_size) != 0 {
        return -(1 as libc::c_int);
    }
    _gdbm_current_bucket_changed(dbf);
    (*(*dbf).cache_mru).ca_data.hash_val = -(1 as libc::c_int);
    (*(*dbf).cache_mru).ca_data.key_size = 0 as libc::c_int;
    (*(*dbf).cache_mru).ca_data.elem_loc = -(1 as libc::c_int);
    return _gdbm_end_update(dbf);
}

//base64.rs
static mut b64tab: [libc::c_char; 65] = unsafe {
    *::std::mem::transmute::<
        &[u8; 65],
        &mut [libc::c_char; 65],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
};
static mut b64val: [libc::c_int; 128] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    62 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    63 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    55 as libc::c_int,
    56 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    59 as libc::c_int,
    60 as libc::c_int,
    61 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    48 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    51 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
];
#[no_mangle]
pub unsafe extern "C" fn _gdbm_base64_encode(
    mut input: *const libc::c_uchar,
    mut input_len: size_t,
    mut output: *mut *mut libc::c_uchar,
    mut output_size: *mut size_t,
    mut nbytes: *mut size_t,
) -> libc::c_int {
    let mut olen: size_t = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(input_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
        .wrapping_div(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if olen > *output_size {
        out = realloc(*output as *mut libc::c_void, olen) as *mut libc::c_uchar;
        if out.is_null() {
            return GDBM_MALLOC_ERROR as libc::c_int;
        }
        *output = out;
        *output_size = olen;
    } else {
        out = *output;
    }
    while input_len >= 3 as libc::c_int as libc::c_ulong {
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = b64tab[(*input.offset(0 as libc::c_int as isize) as libc::c_int
            >> 2 as libc::c_int) as usize] as libc::c_uchar;
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = b64tab[((*input.offset(0 as libc::c_int as isize) as libc::c_int)
            << 4 as libc::c_int & 0x30 as libc::c_int
            | *input.offset(1 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int) as usize] as libc::c_uchar;
        let fresh2 = out;
        out = out.offset(1);
        *fresh2 = b64tab[((*input.offset(1 as libc::c_int as isize) as libc::c_int)
            << 2 as libc::c_int & 0x3c as libc::c_int
            | *input.offset(2 as libc::c_int as isize) as libc::c_int
                >> 6 as libc::c_int) as usize] as libc::c_uchar;
        let fresh3 = out;
        out = out.offset(1);
        *fresh3 = b64tab[(*input.offset(2 as libc::c_int as isize) as libc::c_int
            & 0x3f as libc::c_int) as usize] as libc::c_uchar;
        input_len = (input_len as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
        input = input.offset(3 as libc::c_int as isize);
    }
    if input_len > 0 as libc::c_int as libc::c_ulong {
        let mut c: libc::c_uchar = ((*input.offset(0 as libc::c_int as isize)
            as libc::c_int) << 4 as libc::c_int & 0x30 as libc::c_int) as libc::c_uchar;
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 = b64tab[(*input.offset(0 as libc::c_int as isize) as libc::c_int
            >> 2 as libc::c_int) as usize] as libc::c_uchar;
        if input_len > 1 as libc::c_int as libc::c_ulong {
            c = (c as libc::c_int
                | *input.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 4 as libc::c_int) as libc::c_uchar;
        }
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = b64tab[c as usize] as libc::c_uchar;
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 = (if input_len < 2 as libc::c_int as libc::c_ulong {
            '=' as i32
        } else {
            b64tab[((*input.offset(1 as libc::c_int as isize) as libc::c_int)
                << 2 as libc::c_int & 0x3c as libc::c_int) as usize] as libc::c_int
        }) as libc::c_uchar;
        let fresh7 = out;
        out = out.offset(1);
        *fresh7 = '=' as i32 as libc::c_uchar;
    }
    *out = 0 as libc::c_int as libc::c_uchar;
    *nbytes = out.offset_from(*output) as libc::c_long as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_base64_decode(
    mut input: *const libc::c_uchar,
    mut input_len: size_t,
    mut output: *mut *mut libc::c_uchar,
    mut output_size: *mut size_t,
    mut inbytes: *mut size_t,
    mut outbytes: *mut size_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut olen: libc::c_int = input_len as libc::c_int;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ins: size_t = 0 as libc::c_int as size_t;
    if olen as libc::c_ulong > *output_size {
        out = realloc(*output as *mut libc::c_void, olen as libc::c_ulong)
            as *mut libc::c_uchar;
        if out.is_null() {
            return GDBM_MALLOC_ERROR as libc::c_int;
        }
        *output = out;
        *output_size = olen as size_t;
    } else {
        out = *output;
    }
    while !(input_len < 4 as libc::c_int as libc::c_ulong) {
        if *input.offset(0 as libc::c_int as isize) as libc::c_int > 127 as libc::c_int
            || b64val[*input.offset(0 as libc::c_int as isize) as usize]
                == -(1 as libc::c_int)
            || *input.offset(1 as libc::c_int as isize) as libc::c_int
                > 127 as libc::c_int
            || b64val[*input.offset(1 as libc::c_int as isize) as usize]
                == -(1 as libc::c_int)
            || *input.offset(2 as libc::c_int as isize) as libc::c_int
                > 127 as libc::c_int
            || *input.offset(2 as libc::c_int as isize) as libc::c_int != '=' as i32
                && b64val[*input.offset(2 as libc::c_int as isize) as usize]
                    == -(1 as libc::c_int)
            || *input.offset(3 as libc::c_int as isize) as libc::c_int
                > 127 as libc::c_int
            || *input.offset(3 as libc::c_int as isize) as libc::c_int != '=' as i32
                && b64val[*input.offset(3 as libc::c_int as isize) as usize]
                    == -(1 as libc::c_int)
        {
            rc = GDBM_MALFORMED_DATA as libc::c_int;
            break;
        } else {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = (b64val[*input.offset(0 as libc::c_int as isize) as usize]
                << 2 as libc::c_int
                | b64val[*input.offset(1 as libc::c_int as isize) as usize]
                    >> 4 as libc::c_int) as libc::c_uchar;
            if *input.offset(2 as libc::c_int as isize) as libc::c_int != '=' as i32 {
                let fresh9 = out;
                out = out.offset(1);
                *fresh9 = (b64val[*input.offset(1 as libc::c_int as isize) as usize]
                    << 4 as libc::c_int & 0xf0 as libc::c_int
                    | b64val[*input.offset(2 as libc::c_int as isize) as usize]
                        >> 2 as libc::c_int) as libc::c_uchar;
                if *input.offset(3 as libc::c_int as isize) as libc::c_int != '=' as i32
                {
                    let fresh10 = out;
                    out = out.offset(1);
                    *fresh10 = (b64val[*input.offset(2 as libc::c_int as isize) as usize]
                        << 6 as libc::c_int & 0xc0 as libc::c_int
                        | b64val[*input.offset(3 as libc::c_int as isize) as usize])
                        as libc::c_uchar;
                }
            }
            input = input.offset(4 as libc::c_int as isize);
            input_len = (input_len as libc::c_ulong)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
            ins = (ins as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            if !(input_len > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    *inbytes = ins;
    *outbytes = out.offset_from(*output) as libc::c_long as size_t;
    return rc;
}

//gdbmseq.rs
#[inline]
unsafe extern "C" fn gdbm_valid_key_p(
    mut dbf: GDBM_FILE,
    mut key_ptr: *mut libc::c_char,
    mut key_size: libc::c_int,
    mut elem_loc: libc::c_int,
) -> libc::c_int {
    let mut key: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut hash: libc::c_int = 0;
    let mut bucket: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    key.dptr = key_ptr;
    key.dsize = key_size;
    _gdbm_hash_key(dbf, key, &mut hash, &mut bucket, &mut offset);
    if gdbm_dir_entry_valid_p(dbf, bucket) != 0
        && *((*dbf).dir).offset(bucket as isize)
            == *((*dbf).dir).offset((*dbf).bucket_dir as isize)
        && hash
            == (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .hash_value
    {
        return 1 as libc::c_int;
    }
    gdbm_set_errno(dbf, GDBM_BAD_HASH_ENTRY as libc::c_int, 1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_next_key(
    mut dbf: GDBM_FILE,
    mut elem_loc: libc::c_int,
    mut return_val: *mut datum,
) {
    let mut found: libc::c_int = 0;
    let mut find_data: *mut libc::c_char = 0 as *mut libc::c_char;
    found = 0 as libc::c_int;
    while found == 0 {
        elem_loc += 1;
        if elem_loc == (*(*dbf).header).bucket_elems {
            elem_loc = 0 as libc::c_int;
            while ((*dbf).bucket_dir as libc::c_ulong)
                < ((*(*dbf).header).dir_size as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
                && (*(*dbf).cache_mru).ca_adr
                    == *((*dbf).dir).offset((*dbf).bucket_dir as isize)
            {
                let ref mut fresh0 = (*dbf).bucket_dir;
                *fresh0 += 1;
            }
            if ((*dbf).bucket_dir as libc::c_ulong)
                < ((*(*dbf).header).dir_size as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
            {
                if _gdbm_get_bucket(dbf, (*dbf).bucket_dir) != 0 {
                    return;
                }
            } else {
                gdbm_set_errno(
                    dbf,
                    GDBM_ITEM_NOT_FOUND as libc::c_int,
                    0 as libc::c_int,
                );
                return;
            }
        }
        found = ((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value != -(1 as libc::c_int)) as libc::c_int;
    }
    find_data = _gdbm_read_entry(dbf, elem_loc);
    if find_data.is_null() {
        return;
    }
    if gdbm_valid_key_p(
        dbf,
        find_data,
        (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize)).key_size,
        elem_loc,
    ) == 0
    {
        return;
    }
    (*return_val)
        .dsize = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .key_size;
    if (*return_val).dsize == 0 as libc::c_int {
        let ref mut fresh1 = (*return_val).dptr;
        *fresh1 = malloc(1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    } else {
        let ref mut fresh2 = (*return_val).dptr;
        *fresh2 = malloc((*return_val).dsize as libc::c_ulong) as *mut libc::c_char;
    }
    if ((*return_val).dptr).is_null() {
        (*return_val).dsize = 0 as libc::c_int;
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
    } else {
        memcpy(
            (*return_val).dptr as *mut libc::c_void,
            find_data as *const libc::c_void,
            (*return_val).dsize as libc::c_ulong,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_firstkey(mut dbf: GDBM_FILE) -> datum {
    let mut return_val: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    return_val.dptr = 0 as *mut libc::c_char;
    return_val.dsize = 0 as libc::c_int;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return return_val;
    }
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    if _gdbm_get_bucket(dbf, 0 as libc::c_int) == 0 as libc::c_int {
        get_next_key(dbf, -(1 as libc::c_int), &mut return_val);
        !(return_val.dptr).is_null();
    }
    return return_val;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_nextkey(mut dbf: GDBM_FILE, mut key: datum) -> datum {
    let mut return_val: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut elem_loc: libc::c_int = 0;
    return_val.dptr = 0 as *mut libc::c_char;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return return_val;
    }
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    if (key.dptr).is_null() {
        gdbm_set_errno(dbf, GDBM_ITEM_NOT_FOUND as libc::c_int, 0 as libc::c_int);
        return return_val;
    }
    elem_loc = _gdbm_findkey(
        dbf,
        key,
        0 as *mut *mut libc::c_char,
        0 as *mut libc::c_int,
    );
    if elem_loc == -(1 as libc::c_int) {
        return return_val;
    }
    get_next_key(dbf, elem_loc, &mut return_val);
    !(return_val.dptr).is_null();
    return return_val;
}


//gdbmstore.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_store(
    mut dbf: GDBM_FILE,
    mut key: datum,
    mut content: datum,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut new_hash_val: libc::c_int = 0;
    let mut elem_loc: libc::c_int = 0;
    let mut file_adr: off_t = 0;
    let mut file_pos: off_t = 0;
    let mut free_adr: off_t = 0;
    let mut free_size: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        gdbm_set_errno(dbf, GDBM_READER_CANT_STORE as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if (key.dptr).is_null() || (content.dptr).is_null() {
        gdbm_set_errno(dbf, GDBM_MALFORMED_DATA as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    elem_loc = _gdbm_findkey(dbf, key, 0 as *mut *mut libc::c_char, &mut new_hash_val);
    file_adr = 0 as libc::c_int as off_t;
    new_size = key.dsize + content.dsize;
    if elem_loc != -(1 as libc::c_int) {
        if flags == 1 as libc::c_int {
            free_adr = (*((*(*dbf).bucket).h_table)
                .as_mut_ptr()
                .offset(elem_loc as isize))
                .data_pointer;
            free_size = (*((*(*dbf).bucket).h_table)
                .as_mut_ptr()
                .offset(elem_loc as isize))
                .key_size
                + (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                    .data_size;
            if free_size != new_size {
                if _gdbm_free(dbf, free_adr, free_size) != 0 {
                    return -(1 as libc::c_int);
                }
            } else {
                file_adr = free_adr;
            }
        } else {
            gdbm_set_errno(dbf, GDBM_CANNOT_REPLACE as libc::c_int, 0 as libc::c_int);
            return 1 as libc::c_int;
        }
    } else if *gdbm_errno_location() == GDBM_ITEM_NOT_FOUND as libc::c_int {
        gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    } else {
        return -(1 as libc::c_int)
    }
    if file_adr == 0 as libc::c_int as libc::c_long {
        file_adr = _gdbm_alloc(dbf, new_size);
        if file_adr == 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
    }
    if elem_loc == -(1 as libc::c_int) {
        let mut start_loc: libc::c_int = 0;
        if (*(*dbf).bucket).count == (*(*dbf).header).bucket_elems {
            if _gdbm_split_bucket(dbf, new_hash_val) != 0 {
                return -(1 as libc::c_int);
            }
        }
        start_loc = new_hash_val % (*(*dbf).header).bucket_elems;
        elem_loc = start_loc;
        while (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value != -(1 as libc::c_int)
        {
            elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
            if elem_loc == start_loc {
                gdbm_set_errno(
                    dbf,
                    GDBM_BAD_HASH_TABLE as libc::c_int,
                    1 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        let ref mut fresh0 = (*(*dbf).bucket).count;
        *fresh0 += 1;
        (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value = new_hash_val;
        memcpy(
            ((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .key_start)
                .as_mut_ptr() as *mut libc::c_void,
            key.dptr as *const libc::c_void,
            (if (4 as libc::c_int) < key.dsize { 4 as libc::c_int } else { key.dsize })
                as libc::c_ulong,
        );
    }
    (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .data_pointer = file_adr;
    (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .key_size = key.dsize;
    (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .data_size = content.dsize;
    file_pos = gdbm_file_seek(dbf, file_adr, 0 as libc::c_int);
    if file_pos != file_adr {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"lseek error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    rc = _gdbm_full_write(dbf, key.dptr as *mut libc::c_void, key.dsize as size_t);
    if rc != 0 {
        _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
        return -(1 as libc::c_int);
    }
    rc = _gdbm_full_write(
        dbf,
        content.dptr as *mut libc::c_void,
        content.dsize as size_t,
    );
    if rc != 0 {
        _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
        return -(1 as libc::c_int);
    }
    _gdbm_current_bucket_changed(dbf);
    return _gdbm_end_update(dbf);
}

//gdbmcount.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_count(
    mut dbf: GDBM_FILE,
    mut pcount: *mut gdbm_count_t,
) -> libc::c_int {
    let mut nbuckets: libc::c_int = ((*(*dbf).header).dir_size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong) as libc::c_int;
    let mut count: gdbm_count_t = 0 as libc::c_int as gdbm_count_t;
    let mut i: libc::c_int = 0;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < nbuckets {
        if _gdbm_get_bucket(dbf, i) != 0 {
            return -(1 as libc::c_int);
        }
        count = (count as libc::c_ulonglong)
            .wrapping_add((*(*dbf).bucket).count as libc::c_ulonglong) as gdbm_count_t
            as gdbm_count_t;
        i = _gdbm_next_bucket_dir(dbf, i);
    }
    *pcount = count;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_bucket_count(
    mut dbf: GDBM_FILE,
    mut pcount: *mut size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < ((*(*dbf).header).dir_size as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
    {
        count = count.wrapping_add(1);
        i = _gdbm_next_bucket_dir(dbf, i);
    }
    *pcount = count;
    return 0 as libc::c_int;
}

//gdbmreorg.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_reorganize(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut rcvr: gdbm_recovery = gdbm_recovery {
        errfun: None,
        data: 0 as *mut libc::c_void,
        max_failed_keys: 0,
        max_failed_buckets: 0,
        max_failures: 0,
        recovered_keys: 0,
        recovered_buckets: 0,
        failed_keys: 0,
        failed_buckets: 0,
        duplicate_keys: 0,
        backup_name: 0 as *mut libc::c_char,
    };
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    rcvr.max_failures = 0 as libc::c_int as size_t;
    return gdbm_recover(dbf, &mut rcvr, 0x8 as libc::c_int | 0x20 as libc::c_int);
}

//gdbmexists.rs
pub unsafe extern "C" fn gdbm_exists(mut dbf: GDBM_FILE, mut key: datum) -> libc::c_int {
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return 0 as libc::c_int;
    }
    if _gdbm_findkey(dbf, key, 0 as *mut *mut libc::c_char, 0 as *mut libc::c_int)
        < 0 as libc::c_int
    {
        if *gdbm_errno_location() == GDBM_ITEM_NOT_FOUND as libc::c_int {
            gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

//gdbmimp.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_import_from_file(
    mut dbf: GDBM_FILE,
    mut fp: *mut FILE,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut seenbang: libc::c_int = 0;
    let mut seennewline: libc::c_int = 0;
    let mut rret: libc::c_int = 0;
    let mut rsize: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    let mut ec: libc::c_int = 0;
    let mut kbuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dbuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kbufsize: size_t = 0;
    let mut dbufsize: size_t = 0;
    let mut key: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut data: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut count: libc::c_int = 0 as libc::c_int;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    seenbang = 0 as libc::c_int;
    seennewline = 0 as libc::c_int;
    kbuffer = 0 as *mut libc::c_char;
    dbuffer = 0 as *mut libc::c_char;
    loop {
        rret = fgetc(fp);
        if rret == -(1 as libc::c_int) {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_FILE_READ_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        if rret == '!' as i32 {
            seenbang += 1;
        }
        if !(rret == '\n' as i32) {
            continue;
        }
        if seenbang > 3 as libc::c_int && seennewline > 2 as libc::c_int {
            break;
        }
        seennewline += 1;
    }
    kbufsize = 512 as libc::c_int as size_t;
    kbuffer = malloc(kbufsize) as *mut libc::c_char;
    if kbuffer.is_null() {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_MALLOC_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    dbufsize = 512 as libc::c_int as size_t;
    dbuffer = malloc(dbufsize) as *mut libc::c_char;
    if dbuffer.is_null() {
        free(kbuffer as *mut libc::c_void);
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_MALLOC_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    ec = GDBM_NO_ERROR as libc::c_int;
    loop {
        rret = fread(
            &mut rsize as *mut libc::c_ulong as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
            1 as libc::c_int as size_t,
            fp,
        ) as libc::c_int;
        if !(rret == 1 as libc::c_int) {
            break;
        }
        size = ({
            let mut __v: libc::c_uint = 0;
            let mut __x: libc::c_uint = rsize as libc::c_uint;
            if 0 != 0 {
                __v = (__x & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
                    | (__x & 0xff0000 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
                    | (__x & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
                    | (__x & 0xff as libc::c_int as libc::c_uint) << 24 as libc::c_int;
            } else {
                let fresh0 = &mut __v;
                let fresh1;
                let fresh2 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    options(preserves_flags, pure, readonly)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
        }) as libc::c_ulong;
        if size > 2147483647 as libc::c_int as libc::c_ulong {
            ec = GDBM_MALFORMED_DATA as libc::c_int;
            break;
        } else {
            if size > kbufsize {
                kbufsize = size.wrapping_add(512 as libc::c_int as libc::c_ulong);
                kbuffer = realloc(kbuffer as *mut libc::c_void, kbufsize)
                    as *mut libc::c_char;
                if kbuffer.is_null() {
                    ec = GDBM_MALLOC_ERROR as libc::c_int;
                    break;
                }
            }
            if fread(kbuffer as *mut libc::c_void, size, 1 as libc::c_int as size_t, fp)
                != 1 as libc::c_int as libc::c_ulong
            {
                ec = GDBM_FILE_READ_ERROR as libc::c_int;
                break;
            } else {
                key.dptr = kbuffer;
                key.dsize = size as libc::c_int;
                if fread(
                    &mut rsize as *mut libc::c_ulong as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    fp,
                ) != 1 as libc::c_int as libc::c_ulong
                {
                    ec = GDBM_FILE_READ_ERROR as libc::c_int;
                    break;
                } else {
                    size = ({
                        let mut __v: libc::c_uint = 0;
                        let mut __x: libc::c_uint = rsize as libc::c_uint;
                        if 0 != 0 {
                            __v = (__x & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
                                | (__x & 0xff0000 as libc::c_int as libc::c_uint)
                                    >> 8 as libc::c_int
                                | (__x & 0xff00 as libc::c_int as libc::c_uint)
                                    << 8 as libc::c_int
                                | (__x & 0xff as libc::c_int as libc::c_uint)
                                    << 24 as libc::c_int;
                        } else {
                            let fresh3 = &mut __v;
                            let fresh4;
                            let fresh5 = __x;
                            asm!(
                                "bswap {0}", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                                fresh4, options(preserves_flags, pure, readonly)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                        }
                        __v
                    }) as libc::c_ulong;
                    if size > 2147483647 as libc::c_int as libc::c_ulong {
                        ec = GDBM_MALFORMED_DATA as libc::c_int;
                        break;
                    } else {
                        if size > dbufsize {
                            dbufsize = size
                                .wrapping_add(512 as libc::c_int as libc::c_ulong);
                            dbuffer = realloc(dbuffer as *mut libc::c_void, dbufsize)
                                as *mut libc::c_char;
                            if dbuffer.is_null() {
                                ec = GDBM_MALLOC_ERROR as libc::c_int;
                                break;
                            }
                        }
                        if fread(
                            dbuffer as *mut libc::c_void,
                            size,
                            1 as libc::c_int as size_t,
                            fp,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            ec = GDBM_FILE_READ_ERROR as libc::c_int;
                            break;
                        } else {
                            data.dptr = dbuffer;
                            data.dsize = size as libc::c_int;
                            if gdbm_store(dbf, key, data, flag) != 0 as libc::c_int {
                                ec = *gdbm_errno_location();
                                break;
                            } else {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    if rret < 0 as libc::c_int {
        ec = GDBM_FILE_READ_ERROR as libc::c_int;
    }
    free(kbuffer as *mut libc::c_void);
    free(dbuffer as *mut libc::c_void);
    if ec == GDBM_NO_ERROR as libc::c_int {
        return count;
    }
    gdbm_set_errno(0 as GDBM_FILE, ec, 0 as libc::c_int);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_import(
    mut dbf: GDBM_FILE,
    mut importfile: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut rc: libc::c_int = 0;
    fp = fopen(importfile, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_OPEN_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    rc = gdbm_import_from_file(dbf, fp, flag);
    fclose(fp);
    return rc;
}

//gdbmfdesc.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_fdesc(mut dbf: GDBM_FILE) -> libc::c_int {
    return (*dbf).desc;
}

//gdbmexp.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_export_to_file(
    mut dbf: GDBM_FILE,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: libc::c_ulong = 0;
    let mut key: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut nextkey: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut data: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut header1: *const libc::c_char = b"!\r\n! GDBM FLAT FILE DUMP -- THIS IS NOT A TEXT FILE\r\n! \0"
        as *const u8 as *const libc::c_char;
    let mut header2: *const libc::c_char = b"\r\n!\r\n\0" as *const u8
        as *const libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if !(fwrite(
        header1 as *const libc::c_void,
        strlen(header1),
        1 as libc::c_int as size_t,
        fp,
    ) != 1 as libc::c_int as libc::c_ulong)
    {
        if !(fwrite(
            gdbm_version as *const libc::c_void,
            strlen(gdbm_version),
            1 as libc::c_int as size_t,
            fp,
        ) != 1 as libc::c_int as libc::c_ulong)
        {
            if !(fwrite(
                header2 as *const libc::c_void,
                strlen(header2),
                1 as libc::c_int as size_t,
                fp,
            ) != 1 as libc::c_int as libc::c_ulong)
            {
                key = gdbm_firstkey(dbf);
                loop {
                    if (key.dptr).is_null() {
                        current_block = 5689316957504528238;
                        break;
                    }
                    data = gdbm_fetch(dbf, key);
                    if (data.dptr).is_null() {
                        if *gdbm_errno_location() != GDBM_NO_ERROR as libc::c_int {
                            return -(1 as libc::c_int);
                        }
                    } else {
                        size = ({
                            let mut __v: libc::c_uint = 0;
                            let mut __x: libc::c_uint = key.dsize as libc::c_uint;
                            if 0 != 0 {
                                __v = (__x & 0xff000000 as libc::c_uint)
                                    >> 24 as libc::c_int
                                    | (__x & 0xff0000 as libc::c_int as libc::c_uint)
                                        >> 8 as libc::c_int
                                    | (__x & 0xff00 as libc::c_int as libc::c_uint)
                                        << 8 as libc::c_int
                                    | (__x & 0xff as libc::c_int as libc::c_uint)
                                        << 24 as libc::c_int;
                            } else {
                                let fresh0 = &mut __v;
                                let fresh1;
                                let fresh2 = __x;
                                asm!(
                                    "bswap {0}", inlateout(reg)
                                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) =>
                                    fresh1, options(preserves_flags, pure, readonly)
                                );
                                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                            }
                            __v
                        }) as libc::c_ulong;
                        if fwrite(
                            &mut size as *mut libc::c_ulong as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                            1 as libc::c_int as size_t,
                            fp,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 3698098564683483022;
                            break;
                        }
                        if fwrite(
                            key.dptr as *const libc::c_void,
                            key.dsize as size_t,
                            1 as libc::c_int as size_t,
                            fp,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 3698098564683483022;
                            break;
                        }
                        size = ({
                            let mut __v: libc::c_uint = 0;
                            let mut __x: libc::c_uint = data.dsize as libc::c_uint;
                            if 0 != 0 {
                                __v = (__x & 0xff000000 as libc::c_uint)
                                    >> 24 as libc::c_int
                                    | (__x & 0xff0000 as libc::c_int as libc::c_uint)
                                        >> 8 as libc::c_int
                                    | (__x & 0xff00 as libc::c_int as libc::c_uint)
                                        << 8 as libc::c_int
                                    | (__x & 0xff as libc::c_int as libc::c_uint)
                                        << 24 as libc::c_int;
                            } else {
                                let fresh3 = &mut __v;
                                let fresh4;
                                let fresh5 = __x;
                                asm!(
                                    "bswap {0}", inlateout(reg)
                                    c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                                    fresh4, options(preserves_flags, pure, readonly)
                                );
                                use c2rust_asm_casts::AsmCastTrait;
                                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                            }
                            __v
                        }) as libc::c_ulong;
                        if fwrite(
                            &mut size as *mut libc::c_ulong as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                            1 as libc::c_int as size_t,
                            fp,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 3698098564683483022;
                            break;
                        }
                        if fwrite(
                            data.dptr as *const libc::c_void,
                            data.dsize as size_t,
                            1 as libc::c_int as size_t,
                            fp,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 3698098564683483022;
                            break;
                        }
                    }
                    nextkey = gdbm_nextkey(dbf, key);
                    free(key.dptr as *mut libc::c_void);
                    free(data.dptr as *mut libc::c_void);
                    key = nextkey;
                    count += 1;
                }
                match current_block {
                    3698098564683483022 => {}
                    _ => {
                        if gdbm_last_errno(dbf) == GDBM_ITEM_NOT_FOUND as libc::c_int {
                            gdbm_clear_error(dbf);
                            *gdbm_errno_location() = GDBM_NO_ERROR as libc::c_int;
                        } else {
                            return -(1 as libc::c_int)
                        }
                        return count;
                    }
                }
            }
        }
    }
    gdbm_set_errno(
        0 as GDBM_FILE,
        GDBM_FILE_WRITE_ERROR as libc::c_int,
        0 as libc::c_int,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_export(
    mut dbf: GDBM_FILE,
    mut exportfile: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut nfd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    match flags {
        2 => {
            nfd = open(
                exportfile,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
                mode,
            );
            if nfd == -(1 as libc::c_int) {
                gdbm_set_errno(
                    0 as GDBM_FILE,
                    GDBM_FILE_OPEN_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        3 => {
            nfd = open(
                exportfile,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                mode,
            );
            if nfd == -(1 as libc::c_int) {
                gdbm_set_errno(
                    0 as GDBM_FILE,
                    GDBM_FILE_OPEN_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        _ => {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_FILE_OPEN_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
    }
    fp = fdopen(nfd, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        close(nfd);
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_OPEN_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    rc = gdbm_export_to_file(dbf, fp);
    fclose(fp);
    return rc;
}


//gdbmdump.rs
unsafe extern "C" fn print_datum(
    mut dat: *const datum,
    mut bufptr: *mut *mut libc::c_uchar,
    mut bufsize: *mut size_t,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    fprintf(
        fp,
        b"#:len=%lu\n\0" as *const u8 as *const libc::c_char,
        (*dat).dsize as libc::c_ulong,
    );
    rc = _gdbm_base64_encode(
        (*dat).dptr as *mut libc::c_uchar,
        (*dat).dsize as size_t,
        bufptr,
        bufsize,
        &mut len,
    );
    if rc != 0 {
        return rc;
    }
    p = *bufptr;
    while len != 0 {
        let mut n: size_t = len;
        if n > 76 as libc::c_int as libc::c_ulong {
            n = 76 as libc::c_int as size_t;
        }
        if fwrite(p as *const libc::c_void, n, 1 as libc::c_int as size_t, fp)
            != 1 as libc::c_int as libc::c_ulong
        {
            return GDBM_FILE_WRITE_ERROR as libc::c_int;
        }
        fputc('\n' as i32, fp);
        len = (len as libc::c_ulong).wrapping_sub(n) as size_t as size_t;
        p = p.offset(n as isize);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_dump_ascii(
    mut dbf: GDBM_FILE,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut t: time_t = 0;
    let mut fd: libc::c_int = 0;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut gr: *mut group = 0 as *mut group;
    let mut key: datum = datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    };
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0 as libc::c_int;
    fd = gdbm_fdesc(dbf);
    if fstat(fd, &mut st) != 0 {
        return GDBM_FILE_STAT_ERROR as libc::c_int;
    }
    time(&mut t);
    fprintf(
        fp,
        b"# GDBM dump file created by %s on %s\0" as *const u8 as *const libc::c_char,
        gdbm_version,
        ctime(&mut t),
    );
    fprintf(fp, b"#:version=1.1\n\0" as *const u8 as *const libc::c_char);
    fprintf(fp, b"#:file=%s\n\0" as *const u8 as *const libc::c_char, (*dbf).name);
    fprintf(
        fp,
        b"#:uid=%lu,\0" as *const u8 as *const libc::c_char,
        st.st_uid as libc::c_ulong,
    );
    pw = getpwuid(st.st_uid);
    if !pw.is_null() {
        fprintf(fp, b"user=%s,\0" as *const u8 as *const libc::c_char, (*pw).pw_name);
    }
    fprintf(
        fp,
        b"gid=%lu,\0" as *const u8 as *const libc::c_char,
        st.st_gid as libc::c_ulong,
    );
    gr = getgrgid(st.st_gid);
    if !gr.is_null() {
        fprintf(fp, b"group=%s,\0" as *const u8 as *const libc::c_char, (*gr).gr_name);
    }
    fprintf(
        fp,
        b"mode=%03o\n\0" as *const u8 as *const libc::c_char,
        st.st_mode & 0o777 as libc::c_int as libc::c_uint,
    );
    fprintf(
        fp,
        b"#:format=%s\n\0" as *const u8 as *const libc::c_char,
        if !((*dbf).xheader).is_null() {
            b"numsync\0" as *const u8 as *const libc::c_char
        } else {
            b"standard\0" as *const u8 as *const libc::c_char
        },
    );
    fprintf(fp, b"# End of header\n\0" as *const u8 as *const libc::c_char);
    key = gdbm_firstkey(dbf);
    while !(key.dptr).is_null() {
        let mut nextkey: datum = datum {
            dptr: 0 as *mut libc::c_char,
            dsize: 0,
        };
        let mut data: datum = gdbm_fetch(dbf, key);
        if (data.dptr).is_null() {
            break;
        }
        rc = print_datum(&mut key, &mut buffer, &mut bufsize, fp);
        if rc != 0
            || {
                rc = print_datum(&mut data, &mut buffer, &mut bufsize, fp);
                rc != 0
            }
        {
            free(key.dptr as *mut libc::c_void);
            free(data.dptr as *mut libc::c_void);
            gdbm_set_errno(dbf, rc, 0 as libc::c_int);
            break;
        } else {
            nextkey = gdbm_nextkey(dbf, key);
            free(key.dptr as *mut libc::c_void);
            free(data.dptr as *mut libc::c_void);
            key = nextkey;
            count = count.wrapping_add(1);
        }
    }
    fprintf(fp, b"#:count=%lu\n\0" as *const u8 as *const libc::c_char, count);
    fprintf(fp, b"# End of data\n\0" as *const u8 as *const libc::c_char);
    if rc == 0 as libc::c_int {
        rc = gdbm_last_errno(dbf);
        if rc == GDBM_ITEM_NOT_FOUND as libc::c_int {
            gdbm_clear_error(dbf);
            *gdbm_errno_location() = GDBM_NO_ERROR as libc::c_int;
            rc = 0 as libc::c_int;
        }
    }
    free(buffer as *mut libc::c_void);
    return if rc != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_dump_to_file(
    mut dbf: GDBM_FILE,
    mut fp: *mut FILE,
    mut format: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    match format {
        0 => {
            rc = (gdbm_export_to_file(dbf, fp) == -(1 as libc::c_int)) as libc::c_int;
        }
        1 => {
            rc = _gdbm_dump_ascii(dbf, fp);
        }
        _ => {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_BAD_OPEN_FLAGS as libc::c_int,
                0 as libc::c_int,
            );
            return 22 as libc::c_int;
        }
    }
    if rc == 0 as libc::c_int && ferror(fp) != 0 {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_WRITE_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        rc = -(1 as libc::c_int);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_dump(
    mut dbf: GDBM_FILE,
    mut filename: *const libc::c_char,
    mut fmt: libc::c_int,
    mut open_flags: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut nfd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    match open_flags {
        2 => {
            nfd = open(
                filename,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
                mode,
            );
            if nfd == -(1 as libc::c_int) {
                gdbm_set_errno(
                    0 as GDBM_FILE,
                    GDBM_FILE_OPEN_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        3 => {
            nfd = open(
                filename,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                mode,
            );
            if nfd == -(1 as libc::c_int) {
                gdbm_set_errno(
                    0 as GDBM_FILE,
                    GDBM_FILE_OPEN_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        _ => {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_BAD_OPEN_FLAGS as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
    }
    fp = fdopen(nfd, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        close(nfd);
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_OPEN_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    rc = gdbm_dump_to_file(dbf, fp, fmt);
    fclose(fp);
    return rc;
}

//fullio.rs
#[inline]
unsafe extern "C" fn gdbm_file_write(
    mut dbf: GDBM_FILE,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    return _gdbm_mapped_write(dbf, buf, size);
}
#[inline]
unsafe extern "C" fn gdbm_file_read(
    mut dbf: GDBM_FILE,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    return _gdbm_mapped_read(dbf, buf, size);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_full_read(
    mut dbf: GDBM_FILE,
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = buffer as *mut libc::c_char;
    while size != 0 {
        let mut rdbytes: ssize_t = gdbm_file_read(dbf, ptr as *mut libc::c_void, size);
        if rdbytes == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if gdbm_last_errno(dbf) == GDBM_NO_ERROR as libc::c_int {
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_READ_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        } else {
            if rdbytes == 0 as libc::c_int as libc::c_long {
                gdbm_set_errno(dbf, GDBM_FILE_EOF as libc::c_int, 0 as libc::c_int);
                return -(1 as libc::c_int);
            }
            ptr = ptr.offset(rdbytes as isize);
            size = (size as libc::c_ulong).wrapping_sub(rdbytes as libc::c_ulong)
                as size_t as size_t;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_full_write(
    mut dbf: GDBM_FILE,
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = buffer as *mut libc::c_char;
    (*dbf).file_size = -(1 as libc::c_int) as off_t;
    while size != 0 {
        let mut wrbytes: ssize_t = gdbm_file_write(dbf, ptr as *mut libc::c_void, size);
        if wrbytes == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if gdbm_last_errno(dbf) == GDBM_NO_ERROR as libc::c_int {
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_WRITE_ERROR as libc::c_int,
                    1 as libc::c_int,
                );
            }
            return -(1 as libc::c_int);
        } else {
            if wrbytes == 0 as libc::c_int as libc::c_long {
                *__errno_location() = 28 as libc::c_int;
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_WRITE_ERROR as libc::c_int,
                    1 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            ptr = ptr.offset(wrbytes as isize);
            size = (size as libc::c_ulong).wrapping_sub(wrbytes as libc::c_ulong)
                as size_t as size_t;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_file_extend(
    mut dbf: GDBM_FILE,
    mut size: off_t,
) -> libc::c_int {
    let mut page_size: size_t = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_end: off_t = 0;
    file_end = lseek((*dbf).desc, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    if file_end == 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    size -= file_end;
    if size > 0 as libc::c_int as libc::c_long {
        if (size as libc::c_ulong) < page_size {
            page_size = size as size_t;
        }
        buf = calloc(1 as libc::c_int as libc::c_ulong, page_size) as *mut libc::c_char;
        if buf.is_null() {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        (*dbf).file_size = -(1 as libc::c_int) as off_t;
        while size != 0 {
            let mut n: ssize_t = write(
                (*dbf).desc,
                buf as *const libc::c_void,
                if (size as libc::c_ulong) < page_size {
                    size as libc::c_ulong
                } else {
                    page_size
                },
            );
            if n <= 0 as libc::c_int as libc::c_long {
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_WRITE_ERROR as libc::c_int,
                    1 as libc::c_int,
                );
                break;
            } else {
                size -= n;
            }
        }
        free(buf as *mut libc::c_void);
        if size != 0 {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}


//falloc.rs
#[inline]
unsafe extern "C" fn _gdbm_current_bucket_changed(mut dbf: GDBM_FILE) {
    (*(*dbf).cache_mru).ca_changed = 1 as libc::c_int as libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn _gdbm_avail_block_read(
    mut dbf: GDBM_FILE,
    mut avblk: *mut avail_block,
    mut size: size_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = _gdbm_full_read(dbf, avblk as *mut libc::c_void, size);
    if !(rc != 0) {
        rc = gdbm_avail_block_validate(dbf, avblk, size);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_alloc(
    mut dbf: GDBM_FILE,
    mut num_bytes: libc::c_int,
) -> off_t {
    let mut file_adr: off_t = 0;
    let mut av_el: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    av_el = get_elem(
        num_bytes,
        ((*(*dbf).bucket).bucket_avail).as_mut_ptr(),
        &mut (*(*dbf).bucket).av_count,
    );
    if av_el.av_size == 0 as libc::c_int {
        if (*(*dbf).avail).count <= (*(*dbf).avail).size >> 1 as libc::c_int
            && (*(*dbf).avail).next_block != 0 as libc::c_int as libc::c_long
        {
            if pop_avail_block(dbf) != 0 {
                return 0 as libc::c_int as off_t;
            }
        }
        av_el = get_elem(
            num_bytes,
            ((*(*dbf).avail).av_table).as_mut_ptr(),
            &mut (*(*dbf).avail).count,
        );
        if av_el.av_size == 0 as libc::c_int {
            av_el = get_block(num_bytes, dbf);
        }
        (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    }
    file_adr = av_el.av_adr;
    av_el.av_adr += num_bytes as libc::c_long;
    av_el.av_size -= num_bytes;
    if _gdbm_free(dbf, av_el.av_adr, av_el.av_size) != 0 {
        return 0 as libc::c_int as off_t;
    }
    return file_adr;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_free(
    mut dbf: GDBM_FILE,
    mut file_adr: off_t,
    mut num_bytes: libc::c_int,
) -> libc::c_int {
    let mut temp: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    if num_bytes <= 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    temp.av_size = num_bytes;
    temp.av_adr = file_adr;
    if num_bytes >= (*(*dbf).header).block_size
        || (*dbf).central_free() as libc::c_int != 0
    {
        if (*(*dbf).avail).count == (*(*dbf).avail).size {
            if push_avail_block(dbf) != 0 {
                return -(1 as libc::c_int);
            }
        }
        _gdbm_put_av_elem(
            temp,
            ((*(*dbf).avail).av_table).as_mut_ptr(),
            &mut (*(*dbf).avail).count,
            (*dbf).coalesce_blocks() as libc::c_int,
        );
        (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    } else if (*(*dbf).bucket).av_count < 6 as libc::c_int {
        _gdbm_put_av_elem(
            temp,
            ((*(*dbf).bucket).bucket_avail).as_mut_ptr(),
            &mut (*(*dbf).bucket).av_count,
            (*dbf).coalesce_blocks() as libc::c_int,
        );
    } else {
        if (*(*dbf).avail).count == (*(*dbf).avail).size {
            if push_avail_block(dbf) != 0 {
                return -(1 as libc::c_int);
            }
        }
        _gdbm_put_av_elem(
            temp,
            ((*(*dbf).avail).av_table).as_mut_ptr(),
            &mut (*(*dbf).avail).count,
            (*dbf).coalesce_blocks() as libc::c_int,
        );
        (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    }
    if (*dbf).header_changed() as libc::c_int != 0 && adjust_bucket_avail(dbf) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pop_avail_block(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut file_pos: off_t = 0;
    let mut new_el: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    let mut new_blk: *mut avail_block = 0 as *mut avail_block;
    let mut index: libc::c_int = 0;
    if (*(*dbf).avail).count == (*(*dbf).avail).size {
        if push_avail_block(dbf) != 0 {
            return -(1 as libc::c_int);
        }
    }
    new_el.av_adr = (*(*dbf).avail).next_block;
    new_el
        .av_size = (((*(*dbf).avail).size as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
        >> 1 as libc::c_int)
        .wrapping_add(::std::mem::size_of::<avail_block>() as libc::c_ulong)
        as libc::c_int;
    new_blk = malloc(new_el.av_size as libc::c_ulong) as *mut avail_block;
    if new_blk.is_null() {
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"malloc failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    file_pos = gdbm_file_seek(dbf, new_el.av_adr, 0 as libc::c_int);
    if file_pos != new_el.av_adr {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        free(new_blk as *mut libc::c_void);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"lseek error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    if _gdbm_avail_block_read(dbf, new_blk, new_el.av_size as size_t) != 0 {
        free(new_blk as *mut libc::c_void);
        _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
        return -(1 as libc::c_int);
    }
    index = 0 as libc::c_int;
    while index < (*new_blk).count {
        while index < (*new_blk).count && (*(*dbf).avail).count < (*(*dbf).avail).size {
            _gdbm_put_av_elem(
                *((*new_blk).av_table).as_mut_ptr().offset(index as isize),
                ((*(*dbf).avail).av_table).as_mut_ptr(),
                &mut (*(*dbf).avail).count,
                1 as libc::c_int,
            );
            index += 1;
        }
        if (*(*dbf).avail).count == (*(*dbf).avail).size {
            if push_avail_block(dbf) != 0 {
                free(new_blk as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        }
    }
    (*(*dbf).avail).next_block = (*new_blk).next_block;
    (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    if (*(*dbf).avail).count == (*(*dbf).avail).size {
        if push_avail_block(dbf) != 0 {
            free(new_blk as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    }
    _gdbm_put_av_elem(
        new_el,
        ((*(*dbf).avail).av_table).as_mut_ptr(),
        &mut (*(*dbf).avail).count,
        1 as libc::c_int,
    );
    free(new_blk as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn push_avail_block(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut av_size: libc::c_int = 0;
    let mut av_adr: off_t = 0;
    let mut index: libc::c_int = 0;
    let mut file_pos: off_t = 0;
    let mut temp: *mut avail_block = 0 as *mut avail_block;
    let mut new_loc: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    let mut rc: libc::c_int = 0;
    av_size = (((*(*dbf).avail).size as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
        >> 1 as libc::c_int)
        .wrapping_add(::std::mem::size_of::<avail_block>() as libc::c_ulong)
        as libc::c_int;
    new_loc = get_elem(
        av_size,
        ((*(*dbf).avail).av_table).as_mut_ptr(),
        &mut (*(*dbf).avail).count,
    );
    if new_loc.av_size == 0 as libc::c_int {
        new_loc = get_block(av_size, dbf);
    }
    av_adr = new_loc.av_adr;
    temp = calloc(1 as libc::c_int as libc::c_ulong, av_size as libc::c_ulong)
        as *mut avail_block;
    if temp.is_null() {
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"malloc error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    (*temp).size = (*(*dbf).avail).size;
    (*temp).count = 0 as libc::c_int;
    (*temp).next_block = (*(*dbf).avail).next_block;
    (*(*dbf).avail).next_block = av_adr;
    index = 1 as libc::c_int;
    while index < (*(*dbf).avail).count {
        if index & 0x1 as libc::c_int == 1 as libc::c_int {
            let ref mut fresh0 = (*temp).count;
            let fresh1 = *fresh0;
            *fresh0 = *fresh0 + 1;
            *((*temp).av_table)
                .as_mut_ptr()
                .offset(
                    fresh1 as isize,
                ) = *((*(*dbf).avail).av_table).as_mut_ptr().offset(index as isize);
        } else {
            *((*(*dbf).avail).av_table)
                .as_mut_ptr()
                .offset(
                    (index >> 1 as libc::c_int) as isize,
                ) = *((*(*dbf).avail).av_table).as_mut_ptr().offset(index as isize);
        }
        index += 1;
    }
    (*(*dbf).avail).count -= (*temp).count;
    rc = 0 as libc::c_int;
    new_loc.av_adr += av_size as libc::c_long;
    new_loc.av_size -= av_size;
    if _gdbm_free(dbf, new_loc.av_adr, new_loc.av_size) != 0 {
        rc = -(1 as libc::c_int);
    } else {
        file_pos = gdbm_file_seek(dbf, av_adr, 0 as libc::c_int);
        if file_pos != av_adr {
            gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
            _gdbm_fatal(
                dbf,
                dcgettext(
                    b"gdbm\0" as *const u8 as *const libc::c_char,
                    b"lseek error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            rc = -(1 as libc::c_int);
        } else {
            rc = _gdbm_full_write(dbf, temp as *mut libc::c_void, av_size as size_t);
            if rc != 0 {
                _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
                rc = -(1 as libc::c_int);
            }
        }
    }
    free(temp as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn avail_lookup(
    mut size: libc::c_int,
    mut av_table: *mut avail_elem,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut start: libc::c_int = 0 as libc::c_int;
    while count > 0 as libc::c_int {
        let mut pivot: libc::c_int = start + (count >> 1 as libc::c_int);
        if size == (*av_table.offset(pivot as isize)).av_size {
            return pivot;
        }
        if size > (*av_table.offset(pivot as isize)).av_size {
            start = pivot + 1 as libc::c_int;
            count -= 1;
        }
        count >>= 1 as libc::c_int;
    }
    return start;
}
#[inline]
unsafe extern "C" fn avail_move(
    mut av_table: *mut avail_elem,
    mut av_count: *mut libc::c_int,
    mut src: libc::c_int,
    mut dst: libc::c_int,
) {
    memmove(
        av_table.offset(dst as isize) as *mut libc::c_void,
        av_table.offset(src as isize) as *const libc::c_void,
        ((*av_count - src) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<avail_elem>() as libc::c_ulong),
    );
    *av_count += dst - src;
}
unsafe extern "C" fn get_elem(
    mut size: libc::c_int,
    mut av_table: *mut avail_elem,
    mut av_count: *mut libc::c_int,
) -> avail_elem {
    let mut index: libc::c_int = 0;
    let mut val: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    val.av_adr = 0 as libc::c_int as off_t;
    val.av_size = 0 as libc::c_int;
    index = avail_lookup(size, av_table, *av_count);
    if index >= *av_count {
        return val;
    }
    val = *av_table.offset(index as isize);
    avail_move(av_table, av_count, index + 1 as libc::c_int, index);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_put_av_elem(
    mut new_el: avail_elem,
    mut av_table: *mut avail_elem,
    mut av_count: *mut libc::c_int,
    mut can_merge: libc::c_int,
) {
    let mut index: libc::c_int = 0;
    if new_el.av_size <= 4 as libc::c_int {
        return;
    }
    if can_merge == 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < *av_count {
            if (*av_table.offset(i as isize)).av_adr
                + (*av_table.offset(i as isize)).av_size as libc::c_long == new_el.av_adr
            {
                new_el.av_size += (*av_table.offset(i as isize)).av_size;
                new_el.av_adr = (*av_table.offset(i as isize)).av_adr;
                avail_move(av_table, av_count, i + 1 as libc::c_int, i);
                i -= 1;
            }
            if new_el.av_adr + new_el.av_size as libc::c_long
                == (*av_table.offset(i as isize)).av_adr
            {
                new_el.av_size += (*av_table.offset(i as isize)).av_size;
                avail_move(av_table, av_count, i + 1 as libc::c_int, i);
                i -= 1;
            }
            i += 1;
        }
    }
    index = avail_lookup(new_el.av_size, av_table, *av_count);
    avail_move(av_table, av_count, index, index + 1 as libc::c_int);
    *av_table.offset(index as isize) = new_el;
}
unsafe extern "C" fn get_block(mut size: libc::c_int, mut dbf: GDBM_FILE) -> avail_elem {
    let mut val: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    val.av_adr = (*(*dbf).header).next_block;
    val.av_size = (*(*dbf).header).block_size;
    while val.av_size < size {
        val.av_size += (*(*dbf).header).block_size;
    }
    let ref mut fresh2 = (*(*dbf).header).next_block;
    *fresh2 += val.av_size as libc::c_long;
    (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    return val;
}
unsafe extern "C" fn adjust_bucket_avail(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut third: libc::c_int = 6 as libc::c_int / 3 as libc::c_int;
    let mut av_el: avail_elem = avail_elem {
        av_size: 0,
        av_adr: 0,
    };
    if (*(*dbf).bucket).av_count < third {
        if (*(*dbf).avail).count > 0 as libc::c_int {
            (*(*dbf).avail).count -= 1 as libc::c_int;
            av_el = *((*(*dbf).avail).av_table)
                .as_mut_ptr()
                .offset((*(*dbf).avail).count as isize);
            _gdbm_put_av_elem(
                av_el,
                ((*(*dbf).bucket).bucket_avail).as_mut_ptr(),
                &mut (*(*dbf).bucket).av_count,
                (*dbf).coalesce_blocks() as libc::c_int,
            );
            _gdbm_current_bucket_changed(dbf);
        }
        return 0 as libc::c_int;
    }
    while (*(*dbf).bucket).av_count > 6 as libc::c_int - third
        && (*(*dbf).avail).count < (*(*dbf).avail).size
    {
        av_el = get_elem(
            0 as libc::c_int,
            ((*(*dbf).bucket).bucket_avail).as_mut_ptr(),
            &mut (*(*dbf).bucket).av_count,
        );
        if av_el.av_size == 0 as libc::c_int {
            gdbm_set_errno(dbf, GDBM_BAD_AVAIL as libc::c_int, 1 as libc::c_int);
            return -(1 as libc::c_int);
        }
        _gdbm_put_av_elem(
            av_el,
            ((*(*dbf).avail).av_table).as_mut_ptr(),
            &mut (*(*dbf).avail).count,
            (*dbf).coalesce_blocks() as libc::c_int,
        );
        _gdbm_current_bucket_changed(dbf);
    }
    return 0 as libc::c_int;
}

//mmap.rs
// #[inline]
// unsafe extern "C" fn off_t_sum_ok(mut a: off_t, mut b: off_t) -> libc::c_int {
//     return (a >= 0 as libc::c_int as libc::c_long
//         && b >= 0 as libc::c_int as libc::c_long
//         && (((1 as libc::c_int as off_t)
//             << (::std::mem::size_of::<off_t>() as libc::c_ulong)
//                 .wrapping_mul(8 as libc::c_int as libc::c_ulong)
//                 .wrapping_sub(2 as libc::c_int as libc::c_ulong))
//             - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
//             + 1 as libc::c_int as libc::c_long - a >= b) as libc::c_int;
// }
#[inline]
unsafe extern "C" fn SUM_FILE_SIZE(mut dbf: GDBM_FILE, mut delta: off_t) -> off_t {
    if delta >= 0 as libc::c_int as libc::c_long
        && off_t_sum_ok((*dbf).mapped_off, (*dbf).mapped_size as off_t) != 0
        && off_t_sum_ok(
            ((*dbf).mapped_off as libc::c_ulong).wrapping_add((*dbf).mapped_size)
                as off_t,
            delta,
        ) != 0
    {
        return ((*dbf).mapped_off as libc::c_ulong)
            .wrapping_add((*dbf).mapped_size)
            .wrapping_add(delta as libc::c_ulong) as off_t;
    }
    return -(1 as libc::c_int) as off_t;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_unmap(mut dbf: GDBM_FILE) {
    if !((*dbf).mapped_region).is_null() {
        munmap((*dbf).mapped_region, (*dbf).mapped_size);
        let ref mut fresh0 = (*dbf).mapped_region;
        *fresh0 = 0 as *mut libc::c_void;
        (*dbf).mapped_size = 0 as libc::c_int as size_t;
        (*dbf).mapped_pos = 0 as libc::c_int as off_t;
        (*dbf).mapped_off = 0 as libc::c_int as off_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_internal_remap(
    mut dbf: GDBM_FILE,
    mut size: size_t,
) -> libc::c_int {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut flags: libc::c_int = 0x1 as libc::c_int;
    let mut prot: libc::c_int = 0x1 as libc::c_int;
    let mut page_size: size_t = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    if !((*dbf).mapped_region).is_null() {
        munmap((*dbf).mapped_region, (*dbf).mapped_size);
        let ref mut fresh1 = (*dbf).mapped_region;
        *fresh1 = 0 as *mut libc::c_void;
    }
    (*dbf).mapped_size = size;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let ref mut fresh2 = (*dbf).mapped_pos;
    *fresh2 = (*fresh2 as libc::c_ulong)
        .wrapping_add(((*dbf).mapped_off as libc::c_ulong).wrapping_rem(page_size))
        as off_t as off_t;
    (*dbf)
        .mapped_off = ((*dbf).mapped_off as libc::c_ulong)
        .wrapping_div(page_size)
        .wrapping_mul(page_size) as off_t;
    if (*dbf).read_write() != 0 {
        prot |= 0x2 as libc::c_int;
    }
    if (*dbf).mmap_preread() != 0 {
        flags |= 0x8000 as libc::c_int;
    }
    p = mmap(
        0 as *mut libc::c_void,
        (*dbf).mapped_size,
        prot,
        flags,
        (*dbf).desc,
        (*dbf).mapped_off,
    );
    if p == -(1 as libc::c_int) as *mut libc::c_void {
        let ref mut fresh3 = (*dbf).mapped_region;
        *fresh3 = 0 as *mut libc::c_void;
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    let ref mut fresh4 = (*dbf).mapped_region;
    *fresh4 = p;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_remap(
    mut dbf: GDBM_FILE,
    mut size: off_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut file_size: off_t = 0;
    let mut pos: off_t = 0;
    if size < 0 as libc::c_int as libc::c_long {
        *__errno_location() = 22 as libc::c_int;
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if (size as libc::c_ulong) < (*dbf).mapped_size {
        return 0 as libc::c_int;
    }
    if _gdbm_file_size(dbf, &mut file_size) != 0 {
        let mut __gc: libc::c_int = *gdbm_errno_location();
        let mut __ec: libc::c_int = *__errno_location();
        _gdbm_mapped_unmap(dbf);
        *__errno_location() = __ec;
        *gdbm_errno_location() = __gc;
        return -(1 as libc::c_int);
    }
    if flag == 2 as libc::c_int && size < file_size {
        size = file_size;
    }
    if (*dbf).read_write() != 0 {
        if size > file_size {
            if flag != 0 as libc::c_int {
                if size < (*(*dbf).header).next_block {
                    size = (*(*dbf).header).next_block;
                }
                if _gdbm_file_extend(dbf, size) != 0 {
                    return -(1 as libc::c_int);
                }
                file_size = size;
            } else {
                return 0 as libc::c_int
            }
        }
    } else {
        if size > file_size {
            size = file_size;
        }
        if size == SUM_FILE_SIZE(dbf, 0 as libc::c_int as off_t) {
            return 0 as libc::c_int;
        }
    }
    pos = (*dbf).mapped_off + (*dbf).mapped_pos;
    if size as libc::c_ulong > (*dbf).mapped_size_max {
        (*dbf).mapped_off = pos;
        (*dbf).mapped_pos = 0 as libc::c_int as off_t;
        size = (*dbf).mapped_size_max as off_t;
        if (*dbf).mapped_off + size > file_size {
            size = file_size - (*dbf).mapped_off;
        }
    } else {
        let ref mut fresh5 = (*dbf).mapped_pos;
        *fresh5 += (*dbf).mapped_off;
        (*dbf).mapped_off = 0 as libc::c_int as off_t;
    }
    if pos > file_size {
        *__errno_location() = 22 as libc::c_int;
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    return _gdbm_internal_remap(dbf, size as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_init(mut dbf: GDBM_FILE) -> libc::c_int {
    if (*dbf).mapped_size_max == 0 as libc::c_int as libc::c_ulong {
        (*dbf).mapped_size_max = -(1 as libc::c_int) as size_t;
    }
    return _gdbm_mapped_remap(dbf, 0 as libc::c_int as off_t, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_read(
    mut dbf: GDBM_FILE,
    mut buffer: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    if (*dbf).memory_mapping() != 0 {
        let mut total: ssize_t = 0 as libc::c_int as ssize_t;
        let mut cbuf: *mut libc::c_char = buffer as *mut libc::c_char;
        while len != 0 {
            let mut nbytes: size_t = 0;
            if ((*dbf).mapped_region).is_null()
                || (*dbf).mapped_pos as libc::c_ulong == (*dbf).mapped_size
            {
                let mut pos: off_t = (*dbf).mapped_off + (*dbf).mapped_pos;
                if _gdbm_mapped_remap(
                    dbf,
                    SUM_FILE_SIZE(dbf, len as off_t),
                    0 as libc::c_int,
                ) != 0
                {
                    let mut rc: libc::c_int = 0;
                    if (*dbf).need_recovery() != 0 {
                        return -(1 as libc::c_int) as ssize_t;
                    }
                    (*dbf).set_memory_mapping(0 as libc::c_int as libc::c_uint);
                    if lseek((*dbf).desc, pos, 0 as libc::c_int) != pos {
                        return if total > 0 as libc::c_int as libc::c_long {
                            total
                        } else {
                            -(1 as libc::c_int) as libc::c_long
                        };
                    }
                    rc = read((*dbf).desc, cbuf as *mut libc::c_void, len)
                        as libc::c_int;
                    if rc == -(1 as libc::c_int) {
                        return if total > 0 as libc::c_int as libc::c_long {
                            total
                        } else {
                            -(1 as libc::c_int) as libc::c_long
                        };
                    }
                    return total + rc as libc::c_long;
                }
            }
            nbytes = ((*dbf).mapped_size)
                .wrapping_sub((*dbf).mapped_pos as libc::c_ulong);
            if nbytes == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if nbytes > len {
                nbytes = len;
            }
            memcpy(
                cbuf as *mut libc::c_void,
                ((*dbf).mapped_region as *mut libc::c_char)
                    .offset((*dbf).mapped_pos as isize) as *const libc::c_void,
                nbytes,
            );
            cbuf = cbuf.offset(nbytes as isize);
            let ref mut fresh6 = (*dbf).mapped_pos;
            *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(nbytes) as off_t as off_t;
            total = (total as libc::c_ulong).wrapping_add(nbytes) as ssize_t as ssize_t;
            len = (len as libc::c_ulong).wrapping_sub(nbytes) as size_t as size_t;
        }
        return total;
    }
    return read((*dbf).desc, buffer, len);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_write(
    mut dbf: GDBM_FILE,
    mut buffer: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    if (*dbf).memory_mapping() != 0 {
        let mut total: ssize_t = 0 as libc::c_int as ssize_t;
        let mut cbuf: *mut libc::c_char = buffer as *mut libc::c_char;
        while len != 0 {
            let mut nbytes: size_t = 0;
            if ((*dbf).mapped_region).is_null()
                || (*dbf).mapped_pos as libc::c_ulong == (*dbf).mapped_size
            {
                let mut pos: off_t = (*dbf).mapped_off + (*dbf).mapped_pos;
                if _gdbm_mapped_remap(
                    dbf,
                    SUM_FILE_SIZE(dbf, len as off_t),
                    1 as libc::c_int,
                ) != 0
                {
                    let mut rc: libc::c_int = 0;
                    if (*dbf).need_recovery() != 0 {
                        return -(1 as libc::c_int) as ssize_t;
                    }
                    (*dbf).set_memory_mapping(0 as libc::c_int as libc::c_uint);
                    if lseek((*dbf).desc, pos, 0 as libc::c_int) != pos {
                        return if total > 0 as libc::c_int as libc::c_long {
                            total
                        } else {
                            -(1 as libc::c_int) as libc::c_long
                        };
                    }
                    rc = write((*dbf).desc, cbuf as *const libc::c_void, len)
                        as libc::c_int;
                    if rc == -(1 as libc::c_int) {
                        return if total > 0 as libc::c_int as libc::c_long {
                            total
                        } else {
                            -(1 as libc::c_int) as libc::c_long
                        };
                    }
                    return total + rc as libc::c_long;
                }
            }
            nbytes = ((*dbf).mapped_size)
                .wrapping_sub((*dbf).mapped_pos as libc::c_ulong);
            if nbytes == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if nbytes > len {
                nbytes = len;
            }
            memcpy(
                ((*dbf).mapped_region as *mut libc::c_char)
                    .offset((*dbf).mapped_pos as isize) as *mut libc::c_void,
                cbuf as *const libc::c_void,
                nbytes,
            );
            cbuf = cbuf.offset(nbytes as isize);
            let ref mut fresh7 = (*dbf).mapped_pos;
            *fresh7 = (*fresh7 as libc::c_ulong).wrapping_add(nbytes) as off_t as off_t;
            total = (total as libc::c_ulong).wrapping_add(nbytes) as ssize_t as ssize_t;
            len = (len as libc::c_ulong).wrapping_sub(nbytes) as size_t as size_t;
        }
        return total;
    }
    return write((*dbf).desc, buffer, len);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_lseek(
    mut dbf: GDBM_FILE,
    mut offset: off_t,
    mut whence: libc::c_int,
) -> off_t {
    if (*dbf).memory_mapping() != 0 {
        let mut needle: off_t = 0;
        match whence {
            0 => {
                needle = offset;
            }
            1 => {
                needle = offset + ((*dbf).mapped_off + (*dbf).mapped_pos);
            }
            2 => {
                let mut file_size: off_t = 0;
                if _gdbm_file_size(dbf, &mut file_size) != 0 {
                    return -(1 as libc::c_int) as off_t;
                }
                needle = file_size - offset;
            }
            _ => {
                *__errno_location() = 22 as libc::c_int;
                return -(1 as libc::c_int) as off_t;
            }
        }
        if needle < 0 as libc::c_int as libc::c_long {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int) as off_t;
        }
        if !(needle >= (*dbf).mapped_off
            && ((needle - (*dbf).mapped_off) as libc::c_ulong) < (*dbf).mapped_size)
        {
            _gdbm_mapped_unmap(dbf);
            (*dbf).mapped_off = needle;
            (*dbf).mapped_pos = 0 as libc::c_int as off_t;
        } else {
            (*dbf).mapped_pos = needle - (*dbf).mapped_off;
        }
        return needle;
    }
    return lseek((*dbf).desc, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_mapped_sync(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if !((*dbf).mapped_region).is_null() {
        rc = msync(
            (*dbf).mapped_region,
            (*dbf).mapped_size,
            4 as libc::c_int | 2 as libc::c_int,
        );
    } else {
        rc = fsync((*dbf).desc);
    }
    if rc != 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SYNC_ERROR as libc::c_int, 1 as libc::c_int);
    }
    return rc;
}
//findkey.rs
// #[inline]
// unsafe extern "C" fn gdbm_file_seek(
//     mut dbf: GDBM_FILE,
//     mut off: off_t,
//     mut whence: libc::c_int,
// ) -> off_t {
//     return _gdbm_mapped_lseek(dbf, off, whence);
// }
#[inline]
unsafe extern "C" fn gdbm_offset_ok(mut dbf: GDBM_FILE, mut off: off_t) -> libc::c_int {
    let mut filesize: off_t = 0;
    if _gdbm_file_size(dbf, &mut filesize) != 0 {
        return 0 as libc::c_int;
    }
    return (off <= filesize) as libc::c_int;
}
#[inline]
unsafe extern "C" fn gdbm_bucket_element_valid_p(
    mut dbf: GDBM_FILE,
    mut elem_loc: libc::c_int,
) -> libc::c_int {
    return (elem_loc < (*(*dbf).header).bucket_elems
        && (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .hash_value != -(1 as libc::c_int)
        && (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize)).key_size
            >= 0 as libc::c_int
        && off_t_sum_ok(
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .data_pointer,
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize)).key_size
                as off_t,
        ) != 0
        && (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize)).data_size
            >= 0 as libc::c_int
        && off_t_sum_ok(
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .data_pointer
                + (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                    .key_size as libc::c_long,
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .data_size as off_t,
        ) != 0
        && gdbm_offset_ok(
            dbf,
            (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .data_pointer
                + (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                    .key_size as libc::c_long
                + (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                    .data_size as libc::c_long,
        ) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_read_entry(
    mut dbf: GDBM_FILE,
    mut elem_loc: libc::c_int,
) -> *mut libc::c_char {
    let mut rc: libc::c_int = 0;
    let mut file_pos: off_t = 0;
    let mut key_size: libc::c_int = 0;
    let mut data_size: libc::c_int = 0;
    let mut dsize: size_t = 0;
    let mut data_ca: *mut data_cache_elem = 0 as *mut data_cache_elem;
    if (*(*dbf).cache_mru).ca_data.elem_loc == elem_loc {
        return (*(*dbf).cache_mru).ca_data.dptr;
    }
    if gdbm_bucket_element_valid_p(dbf, elem_loc) == 0 {
        gdbm_set_errno(dbf, GDBM_BAD_HASH_TABLE as libc::c_int, 1 as libc::c_int);
        return 0 as *mut libc::c_char;
    }
    key_size = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .key_size;
    data_size = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .data_size;
    dsize = (key_size + data_size) as size_t;
    data_ca = &mut (*(*dbf).cache_mru).ca_data;
    if dsize <= (*data_ca).dsize {
        if (*data_ca).dsize == 0 as libc::c_int as libc::c_ulong {
            let ref mut fresh0 = (*data_ca).dptr;
            *fresh0 = malloc(1 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
            if !((*data_ca).dptr).is_null() {
                (*data_ca).dsize = 1 as libc::c_int as size_t;
            } else {
                gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
                _gdbm_fatal(
                    dbf,
                    dcgettext(
                        b"gdbm\0" as *const u8 as *const libc::c_char,
                        b"malloc error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as *mut libc::c_char;
            }
        }
    } else {
        let mut p: *mut libc::c_char = realloc(
            (*data_ca).dptr as *mut libc::c_void,
            dsize,
        ) as *mut libc::c_char;
        if !p.is_null() {
            let ref mut fresh1 = (*data_ca).dptr;
            *fresh1 = p;
            (*data_ca).dsize = dsize;
        } else {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            _gdbm_fatal(
                dbf,
                dcgettext(
                    b"gdbm\0" as *const u8 as *const libc::c_char,
                    b"malloc error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_char;
        }
    }
    file_pos = gdbm_file_seek(
        dbf,
        (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .data_pointer,
        0 as libc::c_int,
    );
    if file_pos
        != (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .data_pointer
    {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"lseek error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_char;
    }
    rc = _gdbm_full_read(
        dbf,
        (*data_ca).dptr as *mut libc::c_void,
        (key_size + data_size) as size_t,
    );
    if rc != 0 {
        (*dbf).set_need_recovery(1 as libc::c_int as libc::c_uint);
        _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
        return 0 as *mut libc::c_char;
    }
    (*data_ca).key_size = key_size;
    (*data_ca).data_size = data_size;
    (*data_ca).elem_loc = elem_loc;
    (*data_ca)
        .hash_val = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
        .hash_value;
    return (*data_ca).dptr;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_findkey(
    mut dbf: GDBM_FILE,
    mut key: datum,
    mut ret_dptr: *mut *mut libc::c_char,
    mut ret_hash_val: *mut libc::c_int,
) -> libc::c_int {
    let mut bucket_hash_val: libc::c_int = 0;
    let mut new_hash_val: libc::c_int = 0;
    let mut file_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bucket_dir: libc::c_int = 0;
    let mut elem_loc: libc::c_int = 0;
    let mut home_loc: libc::c_int = 0;
    let mut key_size: libc::c_int = 0;
    _gdbm_hash_key(dbf, key, &mut new_hash_val, &mut bucket_dir, &mut elem_loc);
    if !ret_hash_val.is_null() {
        *ret_hash_val = new_hash_val;
    }
    if _gdbm_get_bucket(dbf, bucket_dir) != 0 {
        return -(1 as libc::c_int);
    }
    if (*(*dbf).cache_mru).ca_data.elem_loc != -(1 as libc::c_int)
        && new_hash_val == (*(*dbf).cache_mru).ca_data.hash_val
        && (*(*dbf).cache_mru).ca_data.key_size == key.dsize
        && !((*(*dbf).cache_mru).ca_data.dptr).is_null()
        && memcmp(
            (*(*dbf).cache_mru).ca_data.dptr as *const libc::c_void,
            key.dptr as *const libc::c_void,
            key.dsize as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        if !ret_dptr.is_null() {
            *ret_dptr = ((*(*dbf).cache_mru).ca_data.dptr).offset(key.dsize as isize);
        }
        return (*(*dbf).cache_mru).ca_data.elem_loc;
    }
    home_loc = elem_loc;
    bucket_hash_val = (*((*(*dbf).bucket).h_table)
        .as_mut_ptr()
        .offset(elem_loc as isize))
        .hash_value;
    while bucket_hash_val != -(1 as libc::c_int) {
        key_size = (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
            .key_size;
        if bucket_hash_val != new_hash_val || key_size != key.dsize
            || memcmp(
                ((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                    .key_start)
                    .as_mut_ptr() as *const libc::c_void,
                key.dptr as *const libc::c_void,
                (if (4 as libc::c_int) < key_size { 4 as libc::c_int } else { key_size })
                    as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
            if elem_loc == home_loc {
                break;
            }
            bucket_hash_val = (*((*(*dbf).bucket).h_table)
                .as_mut_ptr()
                .offset(elem_loc as isize))
                .hash_value;
        } else {
            file_key = _gdbm_read_entry(dbf, elem_loc);
            if file_key.is_null() {
                return -(1 as libc::c_int);
            }
            if memcmp(
                file_key as *const libc::c_void,
                key.dptr as *const libc::c_void,
                key_size as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if !ret_dptr.is_null() {
                    *ret_dptr = file_key.offset(key.dsize as isize);
                }
                return elem_loc;
            } else {
                elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
                if elem_loc == home_loc {
                    break;
                }
                bucket_hash_val = (*((*(*dbf).bucket).h_table)
                    .as_mut_ptr()
                    .offset(elem_loc as isize))
                    .hash_value;
            }
        }
    }
    gdbm_set_errno(dbf, GDBM_ITEM_NOT_FOUND as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}

//lock.rs
#[no_mangle]
pub unsafe extern "C" fn _gdbm_unlock_file(mut dbf: GDBM_FILE) {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    match (*dbf).lock_type as libc::c_uint {
        1 => {
            flock((*dbf).desc, 8 as libc::c_int);
        }
        2 => {
            lockf((*dbf).desc, 0 as libc::c_int, 0 as libc::c_long);
        }
        3 => {
            fl.l_type = 2 as libc::c_int as libc::c_short;
            fl.l_whence = 0 as libc::c_int as libc::c_short;
            fl.l_len = 0 as libc::c_long;
            fl.l_start = fl.l_len;
            fcntl((*dbf).desc, 6 as libc::c_int, &mut fl as *mut flock);
        }
        0 | _ => {}
    }
    (*dbf).lock_type = LOCKING_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_lock_file(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut fl: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    let mut lock_val: libc::c_int = -(1 as libc::c_int);
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        lock_val = flock((*dbf).desc, 1 as libc::c_int + 4 as libc::c_int);
    } else {
        lock_val = flock((*dbf).desc, 2 as libc::c_int + 4 as libc::c_int);
    }
    if lock_val == -(1 as libc::c_int) && *__errno_location() == 11 as libc::c_int {
        (*dbf).lock_type = LOCKING_NONE;
        return lock_val;
    } else {
        if lock_val != -(1 as libc::c_int) {
            (*dbf).lock_type = LOCKING_FLOCK;
            return lock_val;
        }
    }
    lock_val = lockf((*dbf).desc, 1 as libc::c_int, 0 as libc::c_long);
    if lock_val == -(1 as libc::c_int) && *__errno_location() == 35 as libc::c_int {
        (*dbf).lock_type = LOCKING_NONE;
        return lock_val;
    } else {
        if lock_val != -(1 as libc::c_int) {
            (*dbf).lock_type = LOCKING_LOCKF;
            return lock_val;
        }
    }
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        fl.l_type = 0 as libc::c_int as libc::c_short;
    } else {
        fl.l_type = 1 as libc::c_int as libc::c_short;
    }
    fl.l_whence = 0 as libc::c_int as libc::c_short;
    fl.l_len = 0 as libc::c_long;
    fl.l_start = fl.l_len;
    lock_val = fcntl((*dbf).desc, 6 as libc::c_int, &mut fl as *mut flock);
    if lock_val != -(1 as libc::c_int) {
        (*dbf).lock_type = LOCKING_FCNTL;
    }
    if lock_val == -(1 as libc::c_int) {
        (*dbf).lock_type = LOCKING_NONE;
    }
    return lock_val;
}

//hash.rs
#[no_mangle]
pub unsafe extern "C" fn _gdbm_hash(mut key: datum) -> libc::c_int {
    let mut value: libc::c_uint = 0;
    let mut index: libc::c_int = 0;
    value = (0x238f13af as libc::c_uint).wrapping_mul(key.dsize as libc::c_uint);
    index = 0 as libc::c_int;
    while index < key.dsize {
        value = value
            .wrapping_add(
                (*(key.dptr).offset(index as isize) as libc::c_uint)
                    << (index as libc::c_uint)
                        .wrapping_mul(5 as libc::c_int as libc::c_uint)
                        .wrapping_rem(24 as libc::c_int as libc::c_uint),
            ) & 0x7fffffff as libc::c_int as libc::c_uint;
        index += 1;
    }
    value = (1103515243 as libc::c_uint)
        .wrapping_mul(value)
        .wrapping_add(12345 as libc::c_int as libc::c_uint)
        & 0x7fffffff as libc::c_int as libc::c_uint;
    return value as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_bucket_dir(
    mut dbf: GDBM_FILE,
    mut hash: libc::c_int,
) -> libc::c_int {
    return hash >> 31 as libc::c_int - (*(*dbf).header).dir_bits;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_hash_key(
    mut dbf: GDBM_FILE,
    mut key: datum,
    mut hash: *mut libc::c_int,
    mut bucket: *mut libc::c_int,
    mut offset: *mut libc::c_int,
) {
    let mut hashval: libc::c_int = _gdbm_hash(key);
    *hash = hashval;
    *bucket = _gdbm_bucket_dir(dbf, hashval);
    *offset = hashval % (*(*dbf).header).bucket_elems;
}

//gdbmload.rs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct datbuf {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
}


#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}

#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}




#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn dump_file_free(mut file: *mut dump_file) {
    free((*file).linebuf as *mut libc::c_void);
    free((*file).buffer as *mut libc::c_void);
    free((*file).data[0 as libc::c_int as usize].buffer as *mut libc::c_void);
    free((*file).data[1 as libc::c_int as usize].buffer as *mut libc::c_void);
    free((*file).header as *mut libc::c_void);
}
unsafe extern "C" fn getparm(
    mut buf: *const libc::c_char,
    mut parm: *const libc::c_char,
) -> *const libc::c_char {
    if buf.is_null() {
        return 0 as *const libc::c_char;
    }
    while *buf != 0 {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        p = parm;
        while *p as libc::c_int == *buf as libc::c_int {
            p = p.offset(1);
            buf = buf.offset(1);
        }
        if *p as libc::c_int == 0 as libc::c_int && *buf as libc::c_int == '=' as i32 {
            return buf.offset(1 as libc::c_int as isize);
        }
        buf = buf
            .offset(
                (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn get_dump_line(mut file: *mut dump_file) -> size_t {
    let mut buf: [libc::c_char; 80] = [0; 80];
    if (*file).lblevel == 0 as libc::c_int as libc::c_ulong {
        while !(fgets(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
            (*file).fp,
        ))
            .is_null()
        {
            let mut n: size_t = strlen(buf.as_mut_ptr());
            if buf[n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                as libc::c_int == '\n' as i32
            {
                let ref mut fresh0 = (*file).line;
                *fresh0 = (*fresh0).wrapping_add(1);
                n = n.wrapping_sub(1);
            }
            if n
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add((*file).lblevel) > (*file).lbsize
            {
                let mut s: size_t = ((*file).lblevel)
                    .wrapping_add(n)
                    .wrapping_add(76 as libc::c_int as libc::c_ulong)
                    .wrapping_div(76 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(76 as libc::c_int as libc::c_ulong);
                let mut newp: *mut libc::c_char = realloc(
                    (*file).linebuf as *mut libc::c_void,
                    s,
                ) as *mut libc::c_char;
                if newp.is_null() {
                    return GDBM_MALLOC_ERROR as libc::c_int as size_t;
                }
                let ref mut fresh1 = (*file).linebuf;
                *fresh1 = newp;
                (*file).lbsize = s;
            }
            memcpy(
                ((*file).linebuf).offset((*file).lblevel as isize) as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                n,
            );
            let ref mut fresh2 = (*file).lblevel;
            *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if !(buf[n as usize] != 0) {
                continue;
            }
            *((*file).linebuf)
                .offset((*file).lblevel as isize) = 0 as libc::c_int as libc::c_char;
            break;
        }
    }
    return (*file).lblevel;
}
unsafe extern "C" fn get_data(mut file: *mut dump_file) -> libc::c_int {
    let mut n: size_t = 0;
    (*file).buflevel = 0 as libc::c_int as size_t;
    (*file).parmc = 0 as libc::c_int as size_t;
    loop {
        n = get_dump_line(file);
        if !(n != 0) {
            break;
        }
        if *((*file).linebuf).offset(0 as libc::c_int as isize) as libc::c_int
            == '#' as i32
        {
            return 0 as libc::c_int;
        }
        if n.wrapping_add((*file).buflevel) > (*file).bufsize {
            let mut s: size_t = ((*file).buflevel)
                .wrapping_add(n)
                .wrapping_add(76 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(76 as libc::c_int as libc::c_ulong)
                .wrapping_mul(76 as libc::c_int as libc::c_ulong);
            let mut newp: *mut libc::c_char = realloc(
                (*file).buffer as *mut libc::c_void,
                s,
            ) as *mut libc::c_char;
            if newp.is_null() {
                return GDBM_MALLOC_ERROR as libc::c_int;
            }
            let ref mut fresh3 = (*file).buffer;
            *fresh3 = newp;
            (*file).bufsize = s;
        }
        memcpy(
            ((*file).buffer).offset((*file).buflevel as isize) as *mut libc::c_void,
            (*file).linebuf as *const libc::c_void,
            n,
        );
        let ref mut fresh4 = (*file).buflevel;
        *fresh4 = (*fresh4 as libc::c_ulong).wrapping_add(n) as size_t as size_t;
        (*file).lblevel = 0 as libc::c_int as size_t;
    }
    return if ferror((*file).fp) != 0 {
        GDBM_FILE_READ_ERROR as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn get_parms(mut file: *mut dump_file) -> libc::c_int {
    let mut n: size_t = 0;
    (*file).buflevel = 0 as libc::c_int as size_t;
    (*file).parmc = 0 as libc::c_int as size_t;
    loop {
        n = get_dump_line(file);
        if !(n != 0) {
            break;
        }
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = (*file).linebuf;
        if *p as libc::c_int != '#' as i32 {
            return 0 as libc::c_int;
        }
        if n == 0 as libc::c_int as libc::c_ulong
            || {
                p = p.offset(1);
                *p as libc::c_int != ':' as i32
            }
        {
            (*file).lblevel = 0 as libc::c_int as size_t;
        } else {
            n = n.wrapping_sub(1);
            if n == 0 as libc::c_int as libc::c_ulong {
                (*file).lblevel = 0 as libc::c_int as size_t;
            } else {
                if n
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add((*file).buflevel) > (*file).bufsize
                {
                    let mut s: size_t = ((*file).buflevel)
                        .wrapping_add(n)
                        .wrapping_add(76 as libc::c_int as libc::c_ulong)
                        .wrapping_div(76 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(76 as libc::c_int as libc::c_ulong);
                    let mut newp: *mut libc::c_char = realloc(
                        (*file).buffer as *mut libc::c_void,
                        s,
                    ) as *mut libc::c_char;
                    if newp.is_null() {
                        return GDBM_MALLOC_ERROR as libc::c_int;
                    }
                    let ref mut fresh5 = (*file).buffer;
                    *fresh5 = newp;
                    (*file).bufsize = s;
                }
                while *p != 0 {
                    p = p.offset(1);
                    while *p as libc::c_int == ' ' as i32
                        || *p as libc::c_int == '\t' as i32
                    {
                        p = p.offset(1);
                    }
                    if !(*p != 0) {
                        break;
                    }
                    while *p as libc::c_int != 0 && *p as libc::c_int != '=' as i32 {
                        let fresh6 = p;
                        p = p.offset(1);
                        let ref mut fresh7 = (*file).buflevel;
                        let fresh8 = *fresh7;
                        *fresh7 = (*fresh7).wrapping_add(1);
                        *((*file).buffer).offset(fresh8 as isize) = *fresh6;
                    }
                    if *p as libc::c_int == '=' as i32 {
                        let fresh9 = p;
                        p = p.offset(1);
                        let ref mut fresh10 = (*file).buflevel;
                        let fresh11 = *fresh10;
                        *fresh10 = (*fresh10).wrapping_add(1);
                        *((*file).buffer).offset(fresh11 as isize) = *fresh9;
                        if *p as libc::c_int == '"' as i32 {
                            p = p.offset(1);
                            while *p as libc::c_int != 0
                                && *p as libc::c_int != '"' as i32
                            {
                                let fresh12 = p;
                                p = p.offset(1);
                                let ref mut fresh13 = (*file).buflevel;
                                let fresh14 = *fresh13;
                                *fresh13 = (*fresh13).wrapping_add(1);
                                *((*file).buffer).offset(fresh14 as isize) = *fresh12;
                            }
                            if *p as libc::c_int == '"' as i32 {
                                p = p.offset(1);
                            }
                        } else {
                            while !(*p as libc::c_int == 0 as libc::c_int
                                || *p as libc::c_int == ',' as i32)
                            {
                                let fresh15 = p;
                                p = p.offset(1);
                                let ref mut fresh16 = (*file).buflevel;
                                let fresh17 = *fresh16;
                                *fresh16 = (*fresh16).wrapping_add(1);
                                *((*file).buffer).offset(fresh17 as isize) = *fresh15;
                            }
                        }
                        let ref mut fresh18 = (*file).parmc;
                        *fresh18 = (*fresh18).wrapping_add(1);
                        let ref mut fresh19 = (*file).buflevel;
                        let fresh20 = *fresh19;
                        *fresh19 = (*fresh19).wrapping_add(1);
                        *((*file).buffer)
                            .offset(fresh20 as isize) = 0 as libc::c_int as libc::c_char;
                    } else {
                        return GDBM_MALFORMED_DATA as libc::c_int
                    }
                }
                (*file).lblevel = 0 as libc::c_int as size_t;
            }
        }
    }
    if !((*file).buffer).is_null() {
        *((*file).buffer)
            .offset((*file).buflevel as isize) = 0 as libc::c_int as libc::c_char;
    }
    return if ferror((*file).fp) != 0 {
        GDBM_FILE_READ_ERROR as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn get_len(
    mut param: *const libc::c_char,
    mut plen: *mut size_t,
) -> libc::c_int {
    let mut n: libc::c_ulong = 0;
    let mut p: *const libc::c_char = getparm(
        param,
        b"len\0" as *const u8 as *const libc::c_char,
    );
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if p.is_null() {
        return GDBM_ITEM_NOT_FOUND as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    n = strtoul(p, &mut end, 10 as libc::c_int);
    if *end as libc::c_int == 0 as libc::c_int && *__errno_location() == 0 as libc::c_int
    {
        *plen = n;
        return 0 as libc::c_int;
    }
    return GDBM_MALFORMED_DATA as libc::c_int;
}
unsafe extern "C" fn read_record(
    mut file: *mut dump_file,
    mut param: *mut libc::c_char,
    mut n: libc::c_int,
    mut dat: *mut datum,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut consumed_size: size_t = 0;
    let mut decoded_size: size_t = 0;
    if param.is_null() {
        rc = get_parms(file);
        if rc != 0 {
            return rc;
        }
        if (*file).parmc == 0 as libc::c_int as libc::c_ulong {
            return GDBM_ITEM_NOT_FOUND as libc::c_int;
        }
        param = (*file).buffer;
    }
    rc = get_len(param, &mut len);
    if rc != 0 {
        return rc;
    }
    (*dat).dsize = len as libc::c_int;
    rc = get_data(file);
    if rc != 0 {
        return rc;
    }
    rc = _gdbm_base64_decode(
        (*file).buffer as *mut libc::c_uchar,
        (*file).buflevel,
        &mut (*((*file).data).as_mut_ptr().offset(n as isize)).buffer,
        &mut (*((*file).data).as_mut_ptr().offset(n as isize)).size,
        &mut consumed_size,
        &mut decoded_size,
    );
    if rc != 0 {
        return rc;
    }
    if consumed_size != (*file).buflevel || decoded_size != len {
        return GDBM_MALFORMED_DATA as libc::c_int;
    }
    let ref mut fresh21 = (*dat).dptr;
    *fresh21 = (*file).data[n as usize].buffer as *mut libc::c_void as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _set_gdbm_meta_info(
    mut dbf: GDBM_FILE,
    mut param: *mut libc::c_char,
    mut meta_mask: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_ulong = 0;
    let mut owner_uid: uid_t = 0;
    let mut owner_gid: uid_t = 0;
    let mut mode: mode_t = 0;
    let mut meta_flags: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if meta_mask & 0x2 as libc::c_int == 0 {
        p = getparm(param, b"user\0" as *const u8 as *const libc::c_char);
        if !p.is_null() {
            let mut pw: *mut passwd = getpwnam(p);
            if !pw.is_null() {
                owner_uid = (*pw).pw_uid;
                meta_flags |= 0x1 as libc::c_int;
            }
        }
        if meta_flags & 0x1 as libc::c_int == 0
            && {
                p = getparm(param, b"uid\0" as *const u8 as *const libc::c_char);
                !p.is_null()
            }
        {
            *__errno_location() = 0 as libc::c_int;
            n = strtoul(p, &mut end, 10 as libc::c_int);
            if *end as libc::c_int == 0 as libc::c_int
                && *__errno_location() == 0 as libc::c_int
            {
                owner_uid = n as uid_t;
                meta_flags |= 0x1 as libc::c_int;
            }
        }
        p = getparm(param, b"group\0" as *const u8 as *const libc::c_char);
        if !p.is_null() {
            let mut gr: *mut group = getgrnam(p);
            if !gr.is_null() {
                owner_gid = (*gr).gr_gid;
                meta_flags |= 0x2 as libc::c_int;
            }
        }
        if meta_flags & 0x2 as libc::c_int == 0
            && {
                p = getparm(param, b"gid\0" as *const u8 as *const libc::c_char);
                !p.is_null()
            }
        {
            *__errno_location() = 0 as libc::c_int;
            n = strtoul(p, &mut end, 10 as libc::c_int);
            if *end as libc::c_int == 0 as libc::c_int
                && *__errno_location() == 0 as libc::c_int
            {
                owner_gid = n as uid_t;
                meta_flags |= 0x2 as libc::c_int;
            }
        }
    }
    if meta_mask & 0x1 as libc::c_int == 0 {
        p = getparm(param, b"mode\0" as *const u8 as *const libc::c_char);
        if !p.is_null() {
            *__errno_location() = 0 as libc::c_int;
            n = strtoul(p, &mut end, 8 as libc::c_int);
            if *end as libc::c_int == 0 as libc::c_int
                && *__errno_location() == 0 as libc::c_int
            {
                mode = (n & 0o777 as libc::c_int as libc::c_ulong) as mode_t;
                meta_flags |= 0x4 as libc::c_int;
            }
        }
    }
    if meta_flags != 0 {
        let mut fd: libc::c_int = gdbm_fdesc(dbf);
        if getuid() == 0 as libc::c_int as libc::c_uint
            && meta_flags & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0
        {
            if meta_flags & (0x1 as libc::c_int | 0x2 as libc::c_int)
                != 0x1 as libc::c_int | 0x2 as libc::c_int
            {
                let mut st: stat = stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: 0,
                    st_blksize: 0,
                    st_blocks: 0,
                    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                    __glibc_reserved: [0; 3],
                };
                if fstat(fd, &mut st) != 0 {
                    gdbm_set_errno(
                        dbf,
                        GDBM_FILE_STAT_ERROR as libc::c_int,
                        0 as libc::c_int,
                    );
                    return 1 as libc::c_int;
                }
                if meta_flags & 0x1 as libc::c_int == 0 {
                    owner_uid = st.st_uid;
                }
                if meta_flags & 0x2 as libc::c_int == 0 {
                    owner_gid = st.st_gid;
                }
            }
            if fchown(fd, owner_uid, owner_gid) != 0 {
                gdbm_set_errno(
                    dbf,
                    GDBM_ERR_FILE_OWNER as libc::c_int,
                    0 as libc::c_int,
                );
                return 1 as libc::c_int;
            }
        }
        if meta_flags & 0x4 as libc::c_int != 0 && fchmod(fd, mode) != 0 {
            gdbm_set_errno(dbf, GDBM_ERR_FILE_OWNER as libc::c_int, 0 as libc::c_int);
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_str2fmt(mut str: *const libc::c_char) -> libc::c_int {
    if strcmp(str, b"numsync\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0x2000 as libc::c_int;
    }
    if strcmp(str, b"standard\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn _gdbm_load_file(
    mut file: *mut dump_file,
    mut dbf: GDBM_FILE,
    mut ofp: *mut GDBM_FILE,
    mut replace: libc::c_int,
    mut meta_mask: libc::c_int,
) -> libc::c_int {
    let mut param: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut tmp: GDBM_FILE = 0 as GDBM_FILE;
    let mut format: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    rc = get_parms(file);
    if rc != 0 {
        return rc;
    }
    if (*file).parmc != 0 {
        let ref mut fresh22 = (*file).header;
        *fresh22 = (*file).buffer;
        let ref mut fresh23 = (*file).buffer;
        *fresh23 = 0 as *mut libc::c_char;
        let ref mut fresh24 = (*file).buflevel;
        *fresh24 = 0 as libc::c_int as size_t;
        (*file).bufsize = *fresh24;
    } else {
        return GDBM_MALFORMED_DATA as libc::c_int
    }
    p = getparm((*file).header, b"format\0" as *const u8 as *const libc::c_char);
    if !p.is_null() {
        let mut n: libc::c_int = _gdbm_str2fmt(p);
        if n != -(1 as libc::c_int) {
            format = n;
        }
    }
    if dbf.is_null() {
        let mut flags: libc::c_int = if replace != 0 {
            2 as libc::c_int
        } else {
            3 as libc::c_int
        };
        let mut filename: *const libc::c_char = getparm(
            (*file).header,
            b"file\0" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            return GDBM_NO_DBNAME as libc::c_int;
        }
        tmp = gdbm_open(
            filename,
            0 as libc::c_int,
            flags | format,
            0o600 as libc::c_int,
            None,
        );
        if tmp.is_null() {
            return *gdbm_errno_location();
        }
        dbf = tmp;
    }
    if format != 0 {
        if gdbm_convert(dbf, format) != 0 {
            rc = *gdbm_errno_location();
            if !tmp.is_null() {
                gdbm_close(tmp);
            }
            return rc;
        }
    }
    param = (*file).header;
    loop {
        let mut key: datum = datum {
            dptr: 0 as *mut libc::c_char,
            dsize: 0,
        };
        let mut content: datum = datum {
            dptr: 0 as *mut libc::c_char,
            dsize: 0,
        };
        rc = read_record(file, param, 0 as libc::c_int, &mut key);
        if rc != 0 {
            if rc == GDBM_ITEM_NOT_FOUND as libc::c_int && feof((*file).fp) != 0 {
                rc = 0 as libc::c_int;
            }
            break;
        } else {
            param = 0 as *mut libc::c_char;
            rc = read_record(
                file,
                0 as *mut libc::c_char,
                1 as libc::c_int,
                &mut content,
            );
            if rc != 0 {
                break;
            }
            if !(gdbm_store(dbf, key, content, replace) != 0) {
                continue;
            }
            rc = *gdbm_errno_location();
            break;
        }
    }
    if rc == 0 as libc::c_int {
        rc = _set_gdbm_meta_info(dbf, (*file).header, meta_mask);
        *ofp = dbf;
    } else if !tmp.is_null() {
        gdbm_close(tmp);
    }
    return rc;
}
unsafe extern "C" fn read_bdb_header(mut file: *mut dump_file) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    (*file).line = 1 as libc::c_int as size_t;
    if (fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        (*file).fp,
    ))
        .is_null()
    {
        return -(1 as libc::c_int);
    }
    if strcmp(buf.as_mut_ptr(), b"VERSION=3\n\0" as *const u8 as *const libc::c_char)
        != 0
    {
        return -(1 as libc::c_int);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        (*file).fp,
    ))
        .is_null()
    {
        let ref mut fresh25 = (*file).line;
        *fresh25 = (*fresh25).wrapping_add(1);
        if strcmp(
            buf.as_mut_ptr(),
            b"HEADER=END\n\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}

unsafe extern "C" fn c2x(mut c: libc::c_int) -> libc::c_int {
    static mut xdig: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"0123456789abcdef\0")
    };
    let mut p: *mut libc::c_char = strchr(xdig.as_mut_ptr(), c);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    return p.offset_from(xdig.as_mut_ptr()) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn xdatum_read(
    mut fp: *mut FILE,
    mut d: *mut datum,
    mut pdmax: *mut size_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut dmax: size_t = *pdmax;
    (*d).dsize = 0 as libc::c_int;
    loop {
        c = fgetc(fp);
        if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
            break;
        }
        let mut t: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        t = c2x(c);
        if t == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        t <<= 4 as libc::c_int;
        c = fgetc(fp);
        if c == -(1 as libc::c_int) {
            break;
        }
        n = c2x(c);
        if n == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        t += n;
        if (*d).dsize as libc::c_ulong == dmax {
            let mut np: *mut libc::c_char = realloc(
                (*d).dptr as *mut libc::c_void,
                dmax.wrapping_add(128 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if np.is_null() {
                return GDBM_MALLOC_ERROR as libc::c_int;
            }
            let ref mut fresh26 = (*d).dptr;
            *fresh26 = np;
            dmax = (dmax as libc::c_ulong)
                .wrapping_add(128 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        let ref mut fresh27 = (*d).dsize;
        let fresh28 = *fresh27;
        *fresh27 = *fresh27 + 1;
        *((*d).dptr).offset(fresh28 as isize) = t as libc::c_char;
    }
    *pdmax = dmax;
    if c == '\n' as i32 {
        return 0 as libc::c_int;
    }
    return c;
}
unsafe extern "C" fn gdbm_load_bdb_dump(
    mut file: *mut dump_file,
    mut dbf: GDBM_FILE,
    mut replace: libc::c_int,
) -> libc::c_int {
    let mut xd: [datum; 2] = [datum {
        dptr: 0 as *mut libc::c_char,
        dsize: 0,
    }; 2];
    let mut xs: [size_t; 2] = [0; 2];
    let mut rc: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if read_bdb_header(file) != 0 {
        return -(1 as libc::c_int);
    }
    memset(
        &mut xd as *mut [datum; 2] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[datum; 2]>() as libc::c_ulong,
    );
    xs[1 as libc::c_int as usize] = 0 as libc::c_int as size_t;
    xs[0 as libc::c_int as usize] = xs[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    rc = 0 as libc::c_int;
    loop {
        c = fgetc((*file).fp);
        if !(c == ' ' as i32) {
            break;
        }
        rc = xdatum_read(
            (*file).fp,
            &mut *xd.as_mut_ptr().offset(i as isize),
            &mut *xs.as_mut_ptr().offset(i as isize),
        );
        if rc != 0 {
            break;
        }
        let ref mut fresh29 = (*file).line;
        *fresh29 = (*fresh29).wrapping_add(1);
        if i == 1 as libc::c_int {
            if gdbm_store(
                dbf,
                xd[0 as libc::c_int as usize],
                xd[1 as libc::c_int as usize],
                replace,
            ) != 0
            {
                return *gdbm_errno_location();
            }
        }
        i = (i == 0) as libc::c_int;
    }
    free(xd[0 as libc::c_int as usize].dptr as *mut libc::c_void);
    free(xd[1 as libc::c_int as usize].dptr as *mut libc::c_void);
    if rc == 0 as libc::c_int && i != 0 {
        rc = -(1 as libc::c_int);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_load_from_file(
    mut pdbf: *mut GDBM_FILE,
    mut fp: *mut FILE,
    mut replace: libc::c_int,
    mut meta_mask: libc::c_int,
    mut line: *mut libc::c_ulong,
) -> libc::c_int {
    let mut df: dump_file = dump_file {
        fp: 0 as *mut FILE,
        line: 0,
        linebuf: 0 as *mut libc::c_char,
        lbsize: 0,
        lblevel: 0,
        buffer: 0 as *mut libc::c_char,
        bufsize: 0,
        buflevel: 0,
        parmc: 0,
        data: [datbuf {
            buffer: 0 as *mut libc::c_uchar,
            size: 0,
        }; 2],
        header: 0 as *mut libc::c_char,
    };
    let mut rc: libc::c_int = 0;
    if pdbf.is_null() || fp.is_null() {
        return 22 as libc::c_int;
    }
    rc = fgetc(fp);
    ungetc(rc, fp);
    if rc == '!' as i32 {
        if !line.is_null() {
            *line = 0 as libc::c_int as libc::c_ulong;
        }
        if (*pdbf).is_null() {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_NO_DBNAME as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        if gdbm_import_from_file(*pdbf, fp, replace) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    memset(
        &mut df as *mut dump_file as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dump_file>() as libc::c_ulong,
    );
    df.fp = fp;
    if rc == 'V' as i32 {
        if (*pdbf).is_null() {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_NO_DBNAME as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        rc = gdbm_load_bdb_dump(&mut df, *pdbf, replace);
    } else {
        rc = _gdbm_load_file(&mut df, *pdbf, pdbf, replace, meta_mask);
    }
    dump_file_free(&mut df);
    if rc != 0 {
        if !line.is_null() {
            *line = df.line;
        }
        gdbm_set_errno(0 as GDBM_FILE, rc, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_load(
    mut pdbf: *mut GDBM_FILE,
    mut filename: *const libc::c_char,
    mut replace: libc::c_int,
    mut meta_mask: libc::c_int,
    mut line: *mut libc::c_ulong,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut rc: libc::c_int = 0;
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_OPEN_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    rc = gdbm_load_from_file(pdbf, fp, replace, meta_mask, line);
    fclose(fp);
    return rc;
}

//recover.rs
#[no_mangle]
pub unsafe extern "C" fn gdbm_copy_meta(
    mut dst: GDBM_FILE,
    mut src: GDBM_FILE,
) -> libc::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fstat((*src).desc, &mut st) != 0 {
        gdbm_set_errno(
            src,
            GDBM_FILE_STAT_ERROR as libc::c_int,
            (*src).need_recovery() as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if fchown((*dst).desc, st.st_uid, st.st_gid) != 0 {
        gdbm_set_errno(
            dst,
            GDBM_ERR_FILE_OWNER as libc::c_int,
            (*dst).need_recovery() as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if fchmod((*dst).desc, st.st_mode & 0o777 as libc::c_int as libc::c_uint) != 0 {
        gdbm_set_errno(
            dst,
            GDBM_ERR_FILE_MODE as libc::c_int,
            (*dst).need_recovery() as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn backup_name(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut suf_pos: size_t = 0;
    let mut suf_len: size_t = 0;
    len = (strlen(name))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong);
    buf = malloc(len) as *mut libc::c_char;
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(buf, name);
    suf_pos = (strlen(buf)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    suf_len = 1 as libc::c_int as size_t;
    strcat(buf, b".~1~\0" as *const u8 as *const libc::c_char);
    while access(buf, 0 as libc::c_int) == 0 as libc::c_int {
        let mut i: size_t = suf_len;
        while *buf
            .offset(
                suf_pos.wrapping_add(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int == '9' as i32
        {
            *buf
                .offset(
                    suf_pos
                        .wrapping_add(i)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '0' as i32 as libc::c_char;
            i = i.wrapping_sub(1);
            if i == 0 as libc::c_int as libc::c_ulong {
                len = len.wrapping_add(1);
                let mut p: *mut libc::c_char = realloc(buf as *mut libc::c_void, len)
                    as *mut libc::c_char;
                if p.is_null() {
                    let mut __gc: libc::c_int = *gdbm_errno_location();
                    let mut __ec: libc::c_int = *__errno_location();
                    free(buf as *mut libc::c_void);
                    *__errno_location() = __ec;
                    *gdbm_errno_location() = __gc;
                    return 0 as *mut libc::c_char;
                }
                memmove(
                    p.offset(suf_pos as isize).offset(1 as libc::c_int as isize)
                        as *mut libc::c_void,
                    p.offset(suf_pos as isize) as *const libc::c_void,
                    suf_len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                );
                buf = p;
                suf_len = suf_len.wrapping_add(1);
                i = i.wrapping_add(1);
            }
        }
        let ref mut fresh0 = *buf
            .offset(
                suf_pos.wrapping_add(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            );
        *fresh0 += 1;
    }
    return buf;
}
unsafe extern "C" fn _gdbm_finish_transfer(
    mut dbf: GDBM_FILE,
    mut new_dbf: GDBM_FILE,
    mut rcvr: *mut gdbm_recovery,
    mut flags: libc::c_int,
) -> libc::c_int {
    if _gdbm_end_update(new_dbf) != 0 {
        gdbm_close(new_dbf);
        return -(1 as libc::c_int);
    }
    gdbm_sync(new_dbf);
    if gdbm_copy_meta(new_dbf, dbf) != 0 {
        gdbm_close(new_dbf);
        return -(1 as libc::c_int);
    }
    _gdbm_mapped_unmap(dbf);
    if flags & 0x10 as libc::c_int != 0 {
        let mut bkname: *mut libc::c_char = backup_name((*dbf).name);
        if bkname.is_null() {
            let mut __gc: libc::c_int = *gdbm_errno_location();
            let mut __ec: libc::c_int = *__errno_location();
            gdbm_close(new_dbf);
            *__errno_location() = __ec;
            *gdbm_errno_location() = __gc;
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_BACKUP_FAILED as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        if rename((*dbf).name, bkname) != 0 as libc::c_int {
            let mut __gc_0: libc::c_int = *gdbm_errno_location();
            let mut __ec_0: libc::c_int = *__errno_location();
            gdbm_close(new_dbf);
            free(bkname as *mut libc::c_void);
            *__errno_location() = __ec_0;
            *gdbm_errno_location() = __gc_0;
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_BACKUP_FAILED as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        let ref mut fresh1 = (*rcvr).backup_name;
        *fresh1 = bkname;
    }
    if (*dbf).cache_auto() == 0 {
        _gdbm_cache_init(new_dbf, (*dbf).cache_size);
    }
    if rename((*new_dbf).name, (*dbf).name) != 0 as libc::c_int {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_REORGANIZE_FAILED as libc::c_int,
            0 as libc::c_int,
        );
        gdbm_close(new_dbf);
        return -(1 as libc::c_int);
    }
    if (*dbf).file_locking() != 0 {
        _gdbm_unlock_file(dbf);
    }
    close((*dbf).desc);
    free((*dbf).header as *mut libc::c_void);
    free((*dbf).dir as *mut libc::c_void);
    _gdbm_cache_flush(dbf);
    _gdbm_cache_free(dbf);
    (*dbf).lock_type = (*new_dbf).lock_type;
    (*dbf).desc = (*new_dbf).desc;
    let ref mut fresh2 = (*dbf).header;
    *fresh2 = (*new_dbf).header;
    let ref mut fresh3 = (*dbf).dir;
    *fresh3 = (*new_dbf).dir;
    let ref mut fresh4 = (*dbf).bucket;
    *fresh4 = (*new_dbf).bucket;
    (*dbf).bucket_dir = (*new_dbf).bucket_dir;
    let ref mut fresh5 = (*dbf).avail;
    *fresh5 = (*new_dbf).avail;
    (*dbf).avail_size = (*new_dbf).avail_size;
    let ref mut fresh6 = (*dbf).xheader;
    *fresh6 = (*new_dbf).xheader;
    (*dbf).cache_bits = (*new_dbf).cache_bits;
    (*dbf).cache_size = (*new_dbf).cache_size;
    (*dbf).cache_num = (*new_dbf).cache_num;
    let ref mut fresh7 = (*dbf).cache;
    *fresh7 = (*new_dbf).cache;
    let ref mut fresh8 = (*dbf).cache_mru;
    *fresh8 = (*new_dbf).cache_mru;
    let ref mut fresh9 = (*dbf).cache_lru;
    *fresh9 = (*new_dbf).cache_lru;
    let ref mut fresh10 = (*dbf).cache_avail;
    *fresh10 = (*new_dbf).cache_avail;
    (*dbf).set_header_changed((*new_dbf).header_changed());
    (*dbf).set_directory_changed((*new_dbf).directory_changed());
    (*dbf).file_size = -(1 as libc::c_int) as off_t;
    (*dbf).mapped_size_max = (*new_dbf).mapped_size_max;
    let ref mut fresh11 = (*dbf).mapped_region;
    *fresh11 = (*new_dbf).mapped_region;
    (*dbf).mapped_size = (*new_dbf).mapped_size;
    (*dbf).mapped_pos = (*new_dbf).mapped_pos;
    (*dbf).mapped_off = (*new_dbf).mapped_off;
    (*dbf).set_mmap_preread((*new_dbf).mmap_preread());
    free((*new_dbf).name as *mut libc::c_void);
    free(new_dbf as *mut libc::c_void);
    gdbm_file_sync(dbf);
    return _gdbm_get_bucket(dbf, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_next_bucket_dir(
    mut dbf: GDBM_FILE,
    mut bucket_dir: libc::c_int,
) -> libc::c_int {
    let mut dir_count: libc::c_int = ((*(*dbf).header).dir_size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong) as libc::c_int;
    if bucket_dir < 0 as libc::c_int || bucket_dir >= dir_count {
        bucket_dir = dir_count;
    } else {
        let mut cur: off_t = *((*dbf).dir).offset(bucket_dir as isize);
        loop {
            bucket_dir += 1;
            if !(bucket_dir < dir_count
                && cur == *((*dbf).dir).offset(bucket_dir as isize))
            {
                break;
            }
        }
    }
    return bucket_dir;
}
unsafe extern "C" fn check_db(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut bucket_dir: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nbuckets: libc::c_int = ((*(*dbf).header).dir_size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong) as libc::c_int;
    if _gdbm_validate_header(dbf) != 0 {
        return 1 as libc::c_int;
    }
    bucket_dir = 0 as libc::c_int;
    while bucket_dir < nbuckets {
        if _gdbm_get_bucket(dbf, bucket_dir) != 0 {
            return 1 as libc::c_int
        } else {
            if (*(*dbf).bucket).count < 0 as libc::c_int
                || (*(*dbf).bucket).count > (*(*dbf).header).bucket_elems
            {
                return 1 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < (*(*dbf).header).bucket_elems {
                let mut dptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut key: datum = datum {
                    dptr: 0 as *mut libc::c_char,
                    dsize: 0,
                };
                let mut hashval: libc::c_int = 0;
                let mut bucket: libc::c_int = 0;
                let mut off: libc::c_int = 0;
                if !((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(i as isize))
                    .hash_value == -(1 as libc::c_int))
                {
                    dptr = _gdbm_read_entry(dbf, i);
                    if dptr.is_null() {
                        return 1 as libc::c_int;
                    }
                    key.dptr = dptr;
                    key
                        .dsize = (*((*(*dbf).bucket).h_table)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .key_size;
                    if memcmp(
                        ((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(i as isize))
                            .key_start)
                            .as_mut_ptr() as *const libc::c_void,
                        key.dptr as *const libc::c_void,
                        (if (4 as libc::c_int) < key.dsize {
                            4 as libc::c_int
                        } else {
                            key.dsize
                        }) as libc::c_ulong,
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    _gdbm_hash_key(dbf, key, &mut hashval, &mut bucket, &mut off);
                    if bucket >= nbuckets {
                        return 1 as libc::c_int;
                    }
                    if hashval
                        != (*((*(*dbf).bucket).h_table).as_mut_ptr().offset(i as isize))
                            .hash_value
                    {
                        return 1 as libc::c_int;
                    }
                    if *((*dbf).dir).offset(bucket as isize)
                        != *((*dbf).dir).offset(bucket_dir as isize)
                    {
                        return 1 as libc::c_int;
                    }
                }
                i += 1;
            }
        }
        bucket_dir = _gdbm_next_bucket_dir(dbf, bucket_dir);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_recovery(
    mut dbf: GDBM_FILE,
    mut new_dbf: GDBM_FILE,
    mut rcvr: *mut gdbm_recovery,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut bucket_dir: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nbuckets: libc::c_int = ((*(*dbf).header).dir_size as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong) as libc::c_int;
    bucket_dir = 0 as libc::c_int;
    while bucket_dir < nbuckets {
        if _gdbm_get_bucket(dbf, bucket_dir) != 0 {
            if flags & 0x1 as libc::c_int != 0 {
                ((*rcvr).errfun)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*rcvr).data,
                    dcgettext(
                        b"gdbm\0" as *const u8 as *const libc::c_char,
                        b"can't read bucket #%d: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    bucket_dir,
                    gdbm_db_strerror(dbf),
                );
            }
            let ref mut fresh12 = (*rcvr).failed_buckets;
            *fresh12 = (*fresh12).wrapping_add(1);
            if flags & 0x4 as libc::c_int != 0
                && (*rcvr).failed_buckets == (*rcvr).max_failed_buckets
            {
                return -(1 as libc::c_int);
            }
            if flags & 0x8 as libc::c_int != 0
                && ((*rcvr).failed_buckets).wrapping_add((*rcvr).failed_keys)
                    == (*rcvr).max_failures
            {
                return -(1 as libc::c_int);
            }
        } else {
            let ref mut fresh13 = (*rcvr).recovered_buckets;
            *fresh13 = (*fresh13).wrapping_add(1);
            i = 0 as libc::c_int;
            while i < (*(*dbf).header).bucket_elems {
                let mut dptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut key: datum = datum {
                    dptr: 0 as *mut libc::c_char,
                    dsize: 0,
                };
                let mut data: datum = datum {
                    dptr: 0 as *mut libc::c_char,
                    dsize: 0,
                };
                if !((*((*(*dbf).bucket).h_table).as_mut_ptr().offset(i as isize))
                    .hash_value == -(1 as libc::c_int))
                {
                    dptr = _gdbm_read_entry(dbf, i);
                    if !dptr.is_null() {
                        let ref mut fresh14 = (*rcvr).recovered_keys;
                        *fresh14 = (*fresh14).wrapping_add(1);
                        key.dptr = dptr;
                        key
                            .dsize = (*((*(*dbf).bucket).h_table)
                            .as_mut_ptr()
                            .offset(i as isize))
                            .key_size;
                        data.dptr = dptr.offset(key.dsize as isize);
                        data
                            .dsize = (*((*(*dbf).bucket).h_table)
                            .as_mut_ptr()
                            .offset(i as isize))
                            .data_size;
                        if gdbm_store(new_dbf, key, data, 0 as libc::c_int)
                            != 0 as libc::c_int
                        {
                            match gdbm_last_errno(new_dbf) {
                                17 => {
                                    let ref mut fresh16 = (*rcvr).duplicate_keys;
                                    *fresh16 = (*fresh16).wrapping_add(1);
                                    if flags & 0x1 as libc::c_int != 0 {
                                        ((*rcvr).errfun)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*rcvr).data,
                                            dcgettext(
                                                b"gdbm\0" as *const u8 as *const libc::c_char,
                                                b"ignoring duplicate key %d:%d (%lu:%d)\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            bucket_dir,
                                            i,
                                            (*((*(*dbf).bucket).h_table)
                                                .as_mut_ptr()
                                                .offset(i as isize))
                                                .data_pointer as libc::c_ulong,
                                            (*((*(*dbf).bucket).h_table)
                                                .as_mut_ptr()
                                                .offset(i as isize))
                                                .key_size
                                                + (*((*(*dbf).bucket).h_table)
                                                    .as_mut_ptr()
                                                    .offset(i as isize))
                                                    .data_size,
                                        );
                                    }
                                }
                                _ => {
                                    if flags & 0x1 as libc::c_int != 0 {
                                        ((*rcvr).errfun)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*rcvr).data,
                                            dcgettext(
                                                b"gdbm\0" as *const u8 as *const libc::c_char,
                                                b"fatal: can't store element %d:%d (%lu:%d): %s\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            bucket_dir,
                                            i,
                                            (*((*(*dbf).bucket).h_table)
                                                .as_mut_ptr()
                                                .offset(i as isize))
                                                .data_pointer as libc::c_ulong,
                                            (*((*(*dbf).bucket).h_table)
                                                .as_mut_ptr()
                                                .offset(i as isize))
                                                .key_size
                                                + (*((*(*dbf).bucket).h_table)
                                                    .as_mut_ptr()
                                                    .offset(i as isize))
                                                    .data_size,
                                            gdbm_db_strerror(new_dbf),
                                        );
                                    }
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    } else {
                        if flags & 0x1 as libc::c_int != 0 {
                            ((*rcvr).errfun)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*rcvr).data,
                                dcgettext(
                                    b"gdbm\0" as *const u8 as *const libc::c_char,
                                    b"can't read key pair %d:%d (%lu:%d): %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                bucket_dir,
                                i,
                                (*((*(*dbf).bucket).h_table)
                                    .as_mut_ptr()
                                    .offset(i as isize))
                                    .data_pointer as libc::c_ulong,
                                (*((*(*dbf).bucket).h_table)
                                    .as_mut_ptr()
                                    .offset(i as isize))
                                    .key_size
                                    + (*((*(*dbf).bucket).h_table)
                                        .as_mut_ptr()
                                        .offset(i as isize))
                                        .data_size,
                                gdbm_db_strerror(dbf),
                            );
                        }
                        let ref mut fresh15 = (*rcvr).failed_keys;
                        *fresh15 = (*fresh15).wrapping_add(1);
                        if flags & 0x2 as libc::c_int != 0
                            && (*rcvr).failed_keys == (*rcvr).max_failed_keys
                        {
                            return -(1 as libc::c_int);
                        }
                        if flags & 0x8 as libc::c_int != 0
                            && ((*rcvr).failed_buckets).wrapping_add((*rcvr).failed_keys)
                                == (*rcvr).max_failures
                        {
                            return -(1 as libc::c_int);
                        }
                    }
                }
                i += 1;
            }
        }
        bucket_dir = _gdbm_next_bucket_dir(dbf, bucket_dir);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_recover(
    mut dbf: GDBM_FILE,
    mut rcvr: *mut gdbm_recovery,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut new_dbf: GDBM_FILE = 0 as *mut gdbm_file_info;
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut rs: gdbm_recovery = gdbm_recovery {
        errfun: None,
        data: 0 as *mut libc::c_void,
        max_failed_keys: 0,
        max_failed_buckets: 0,
        max_failures: 0,
        recovered_keys: 0,
        recovered_buckets: 0,
        failed_keys: 0,
        failed_buckets: 0,
        duplicate_keys: 0,
        backup_name: 0 as *mut libc::c_char,
    };
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        gdbm_set_errno(
            dbf,
            GDBM_READER_CANT_REORGANIZE as libc::c_int,
            (*dbf).need_recovery() as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if rcvr.is_null() {
        rcvr = &mut rs;
        flags = 0 as libc::c_int;
    }
    (*rcvr).recovered_keys = 0 as libc::c_int as size_t;
    (*rcvr).recovered_buckets = 0 as libc::c_int as size_t;
    (*rcvr).failed_keys = 0 as libc::c_int as size_t;
    (*rcvr).failed_buckets = 0 as libc::c_int as size_t;
    (*rcvr).duplicate_keys = 0 as libc::c_int as size_t;
    let ref mut fresh17 = (*rcvr).backup_name;
    *fresh17 = 0 as *mut libc::c_char;
    rc = 0 as libc::c_int;
    if flags & 0x20 as libc::c_int != 0 || check_db(dbf) != 0 {
        gdbm_clear_error(dbf);
        len = strlen((*dbf).name);
        new_name = malloc(
            len.wrapping_add(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong),
        ) as *mut libc::c_char;
        if new_name.is_null() {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        strcat(
            strcpy(new_name, (*dbf).name),
            b".XXXXXX\0" as *const u8 as *const libc::c_char,
        );
        fd = mkstemp(new_name);
        if fd == -(1 as libc::c_int) {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_FILE_OPEN_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            free(new_name as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        new_dbf = gdbm_fd_open(
            fd,
            new_name,
            (*(*dbf).header).block_size,
            2 as libc::c_int
                | (if (*dbf).cloexec() as libc::c_int != 0 {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                })
                | (if !((*dbf).xheader).is_null() {
                    0x2000 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | 0x400 as libc::c_int,
            (*dbf).fatal_err,
        );
        let mut __gc: libc::c_int = *gdbm_errno_location();
        let mut __ec: libc::c_int = *__errno_location();
        free(new_name as *mut libc::c_void);
        *__errno_location() = __ec;
        *gdbm_errno_location() = __gc;
        if new_dbf.is_null() {
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_REORGANIZE_FAILED as libc::c_int,
                0 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        rc = run_recovery(dbf, new_dbf, rcvr, flags);
        if rc == 0 as libc::c_int {
            rc = _gdbm_finish_transfer(dbf, new_dbf, rcvr, flags);
        } else {
            gdbm_close(new_dbf);
        }
    }
    if rc == 0 as libc::c_int {
        gdbm_clear_error(dbf);
        (*dbf).set_need_recovery(0 as libc::c_int as libc::c_uint);
    }
    return rc;
}

//gdbmsetopt.rs
unsafe extern "C" fn getbool(
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        || {
            n = *(optval as *mut libc::c_int);
            n != 1 as libc::c_int && n != 0 as libc::c_int
        }
    {
        return -(1 as libc::c_int);
    }
    return n;
}
unsafe extern "C" fn get_size(
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
    mut ret: *mut size_t,
) -> libc::c_int {
    if optval.is_null() {
        return -(1 as libc::c_int);
    }
    if optlen as libc::c_ulong == ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
    {
        *ret = *(optval as *mut libc::c_uint) as size_t;
    } else if optlen as libc::c_ulong
            == ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong
        {
        *ret = *(optval as *mut libc::c_ulong);
    } else if optlen as libc::c_ulong == ::std::mem::size_of::<size_t>() as libc::c_ulong
        {
        *ret = *(optval as *mut size_t);
    } else {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setcachesize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut sz: size_t = 0;
    if get_size(optval, optlen, &mut sz) != 0 {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    return _gdbm_cache_init(dbf, sz);
}
unsafe extern "C" fn setopt_gdbm_getcachesize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong != ::std::mem::size_of::<size_t>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut size_t) = (*dbf).cache_size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setcacheauto(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).set_cache_auto(n as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getcacheauto(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut libc::c_int) = ((*dbf).cache_auto() != 0) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_fastmode(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).set_fast_write(n as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setsyncmode(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).set_fast_write((n == 0) as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getsyncmode(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut libc::c_int) = ((*dbf).fast_write() == 0) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setcentfree(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).set_central_free(n as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getcentfree(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut libc::c_int) = ((*dbf).central_free() == 0) as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setcoalesceblks(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).set_coalesce_blocks(n as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getcoalesceblks(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut libc::c_int) = (*dbf).coalesce_blocks() as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setmmap(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = getbool(optval, optlen);
    if n == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    gdbm_file_sync(dbf);
    if n == (*dbf).memory_mapping() as libc::c_int {
        return 0 as libc::c_int;
    }
    if n != 0 {
        if _gdbm_mapped_init(dbf) == 0 as libc::c_int {
            (*dbf).set_memory_mapping(1 as libc::c_int as libc::c_uint);
        } else {
            return -(1 as libc::c_int)
        }
    } else {
        _gdbm_mapped_unmap(dbf);
        (*dbf).set_memory_mapping(0 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getmmap(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut libc::c_int) = (*dbf).memory_mapping() as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_setmaxmapsize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut page_size: size_t = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    let mut sz: size_t = 0;
    if get_size(optval, optlen, &mut sz) != 0 {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf)
        .mapped_size_max = sz
        .wrapping_add(page_size)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(page_size)
        .wrapping_mul(page_size);
    _gdbm_mapped_init(dbf);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getmaxmapsize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong != ::std::mem::size_of::<size_t>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    *(optval as *mut size_t) = (*dbf).mapped_size_max;
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getflags(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    } else {
        let mut flags: libc::c_int = (*dbf).read_write() as libc::c_int;
        if (*dbf).fast_write() == 0 {
            flags |= 0x20 as libc::c_int;
        }
        if (*dbf).file_locking() == 0 {
            flags |= 0x40 as libc::c_int;
        }
        if (*dbf).memory_mapping() == 0 {
            flags |= 0x80 as libc::c_int;
        } else if (*dbf).mmap_preread() != 0 {
            flags |= 0x1000 as libc::c_int;
        }
        if (*dbf).cloexec() != 0 {
            flags |= 0x100 as libc::c_int;
        }
        if (*(*dbf).header).header_magic as libc::c_uint == 0x13579ad1 as libc::c_uint {
            flags |= 0x2000 as libc::c_int;
        }
        *(optval as *mut libc::c_int) = flags;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getdbname(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if optval.is_null()
        || optlen as libc::c_ulong
            != ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
    {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    } else {
        let mut p: *mut libc::c_char = strdup((*dbf).name);
        if p.is_null() {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        let ref mut fresh0 = *(optval as *mut *mut libc::c_char);
        *fresh0 = p;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn setopt_gdbm_getblocksize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if !optval.is_null()
        && optlen as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        *(optval as *mut libc::c_int) = (*(*dbf).header).block_size;
        return 0 as libc::c_int;
    }
    gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn setopt_gdbm_getdbformat(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if !optval.is_null()
        && optlen as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        match (*(*dbf).header).header_magic {
            324508366 | 324508367 => {
                *(optval as *mut libc::c_int) = 0 as libc::c_int;
            }
            324508369 => {
                *(optval as *mut libc::c_int) = 0x2000 as libc::c_int;
            }
            _ => {}
        }
        return 0 as libc::c_int;
    }
    gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn setopt_gdbm_getdirdepth(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if !optval.is_null()
        && optlen as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        *(optval as *mut libc::c_int) = (*(*dbf).header).dir_bits;
        return 0 as libc::c_int;
    }
    gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn setopt_gdbm_getbucketsize(
    mut dbf: GDBM_FILE,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if !optval.is_null()
        && optlen as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        *(optval as *mut libc::c_int) = (*(*dbf).header).bucket_elems;
        return 0 as libc::c_int;
    }
    gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}
static mut setopt_handler_tab: [setopt_handler; 22] = unsafe {
    [
        None,
        Some(
            setopt_gdbm_setcachesize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_fastmode
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setsyncmode
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setcentfree
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setcoalesceblks
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setmaxmapsize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setmmap
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getflags
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getmmap
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getcachesize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getsyncmode
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getcentfree
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getcoalesceblks
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getmaxmapsize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getdbname
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getblocksize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getdbformat
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getdirdepth
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getbucketsize
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_getcacheauto
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            setopt_gdbm_setcacheauto
                as unsafe extern "C" fn(
                    GDBM_FILE,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn gdbm_setopt(
    mut dbf: GDBM_FILE,
    mut optflag: libc::c_int,
    mut optval: *mut libc::c_void,
    mut optlen: libc::c_int,
) -> libc::c_int {
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if optflag >= 0 as libc::c_int
        && (optflag as libc::c_ulong)
            < (::std::mem::size_of::<[setopt_handler; 22]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<setopt_handler>() as libc::c_ulong)
        && (setopt_handler_tab[optflag as usize]).is_some()
    {
        return (setopt_handler_tab[optflag as usize])
            .expect("non-null function pointer")(dbf, optval, optlen);
    }
    gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
    return -(1 as libc::c_int);
}

//avail.rs
#[inline]
unsafe extern "C" fn off_t_sum_ok(mut a: off_t, mut b: off_t) -> libc::c_int {
    return (a >= 0 as libc::c_int as libc::c_long
        && b >= 0 as libc::c_int as libc::c_long
        && (((1 as libc::c_int as off_t)
            << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long - a >= b) as libc::c_int;
}
unsafe extern "C" fn avail_comp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ava: *const avail_elem = a as *const avail_elem;
    let mut avb: *const avail_elem = b as *const avail_elem;
    if (*ava).av_size < (*avb).av_size {
        return -(1 as libc::c_int)
    } else {
        if (*ava).av_size > (*avb).av_size {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gdbm_avail_table_valid_p(
    mut dbf: GDBM_FILE,
    mut av: *mut avail_elem,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut prev: off_t = 0 as libc::c_int as off_t;
    let mut i: libc::c_int = 0;
    let mut needs_sorting: libc::c_int = 0 as libc::c_int;
    let mut p: *mut avail_elem = av;
    prev = 0 as libc::c_int as off_t;
    i = 0 as libc::c_int;
    while i < count {
        if !((*p).av_adr >= (*(*dbf).header).bucket_size as libc::c_long
            && off_t_sum_ok((*p).av_adr, (*p).av_size as off_t) != 0
            && (*p).av_adr + (*p).av_size as libc::c_long <= (*(*dbf).header).next_block)
        {
            return 0 as libc::c_int;
        }
        if ((*p).av_size as libc::c_long) < prev {
            needs_sorting = 1 as libc::c_int;
        }
        prev = (*p).av_size as off_t;
        i += 1;
        p = p.offset(1);
    }
    if needs_sorting != 0 && (*dbf).read_write() as libc::c_int != 0 {
        qsort(
            av as *mut libc::c_void,
            count as size_t,
            ::std::mem::size_of::<avail_elem>() as libc::c_ulong,
            Some(
                avail_comp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_avail_block_validate(
    mut dbf: GDBM_FILE,
    mut avblk: *mut avail_block,
    mut size: size_t,
) -> libc::c_int {
    if !(size > ::std::mem::size_of::<avail_block>() as libc::c_ulong
        && ((*avblk).size > 1 as libc::c_int && (*avblk).count >= 0 as libc::c_int
            && (*avblk).count <= (*avblk).size)
        && size
            .wrapping_sub(::std::mem::size_of::<avail_block>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            >= (*avblk).count as libc::c_ulong
        && gdbm_avail_table_valid_p(
            dbf,
            ((*avblk).av_table).as_mut_ptr(),
            (*avblk).count,
        ) != 0)
    {
        gdbm_set_errno(dbf, GDBM_BAD_AVAIL as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_bucket_avail_table_validate(
    mut dbf: GDBM_FILE,
    mut bucket: *mut hash_bucket,
) -> libc::c_int {
    if !((*bucket).av_count >= 0 as libc::c_int && (*bucket).av_count <= 6 as libc::c_int
        && gdbm_avail_table_valid_p(
            dbf,
            ((*bucket).bucket_avail).as_mut_ptr(),
            (*bucket).av_count,
        ) != 0)
    {
        gdbm_set_errno(dbf, GDBM_BAD_AVAIL as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn off_map_free(mut map: *mut off_map) {
    free((*map).map_base as *mut libc::c_void);
}
unsafe extern "C" fn off_map_expand(mut map: *mut off_map) -> libc::c_int {
    let mut n: size_t = (*map).map_max;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*map).map_base).is_null() {
        if n == 0 {
            n = 64 as libc::c_int as size_t;
        }
    } else {
        if (-(1 as libc::c_int) as size_t)
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong) <= n
        {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    p = realloc(
        (*map).map_base as *mut libc::c_void,
        n.wrapping_mul(::std::mem::size_of::<off_t>() as libc::c_ulong),
    );
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    let ref mut fresh0 = (*map).map_base;
    *fresh0 = p as *mut off_t;
    (*map).map_max = n;
    return 0 as libc::c_int;
}
unsafe extern "C" fn off_map_lookup(mut map: *mut off_map, mut n: off_t) -> libc::c_int {
    let mut lo: ssize_t = 0;
    let mut hi: ssize_t = 0;
    let mut mid: ssize_t = 0;
    if (*map).map_size != 0 {
        lo = 0 as libc::c_int as ssize_t;
        hi = ((*map).map_size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as ssize_t;
        while lo <= hi {
            mid = (lo + hi) / 2 as libc::c_int as libc::c_long;
            if *((*map).map_base).offset(mid as isize) > n {
                hi = mid - 1 as libc::c_int as libc::c_long;
            } else if *((*map).map_base).offset(mid as isize) < n {
                lo = mid + 1 as libc::c_int as libc::c_long;
            } else {
                return GDBM_CANNOT_REPLACE as libc::c_int
            }
        }
    } else {
        hi = -(1 as libc::c_int) as ssize_t;
    }
    if (*map).map_size == (*map).map_max {
        if off_map_expand(map) != 0 {
            return GDBM_MALLOC_ERROR as libc::c_int;
        }
    }
    hi += 1;
    if (*map).map_size > hi as libc::c_ulong {
        memmove(
            ((*map).map_base).offset(hi as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            ((*map).map_base).offset(hi as isize) as *const libc::c_void,
            ((*map).map_size)
                .wrapping_sub(hi as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<off_t>() as libc::c_ulong),
        );
    }
    *((*map).map_base).offset(hi as isize) = n;
    let ref mut fresh1 = (*map).map_size;
    *fresh1 = (*fresh1).wrapping_add(1);
    return GDBM_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_avail_traverse(
    mut dbf: GDBM_FILE,
    mut cb: Option::<
        unsafe extern "C" fn(*mut avail_block, off_t, *mut libc::c_void) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut blk: *mut avail_block = 0 as *mut avail_block;
    let mut size: size_t = 0;
    let mut n: off_t = 0;
    let mut map: off_map = {
        let mut init = off_map {
            map_base: 0 as *mut off_t,
            map_size: 0 as libc::c_int as size_t,
            map_max: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if gdbm_avail_block_validate(dbf, (*dbf).avail, (*dbf).avail_size) != 0 {
        return -(1 as libc::c_int);
    }
    if off_map_lookup(
        &mut map,
        ((*dbf).avail as *mut libc::c_char)
            .offset_from((*dbf).header as *mut libc::c_char) as libc::c_long,
    ) != 0
    {
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    size = (((*(*dbf).avail).size as size_t)
        .wrapping_mul(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
        >> 1 as libc::c_int)
        .wrapping_add(::std::mem::size_of::<avail_block>() as libc::c_ulong);
    blk = malloc(size) as *mut avail_block;
    if blk.is_null() {
        gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
        off_map_free(&mut map);
        return -(1 as libc::c_int);
    }
    if !(cb.is_some()
        && cb
            .expect(
                "non-null function pointer",
            )((*dbf).avail, 0 as libc::c_int as off_t, data) != 0)
    {
        n = (*(*dbf).avail).next_block;
        while n != 0 {
            rc = off_map_lookup(&mut map, n);
            if rc != GDBM_NO_ERROR as libc::c_int {
                if rc == GDBM_CANNOT_REPLACE as libc::c_int {
                    gdbm_set_errno(dbf, GDBM_BAD_AVAIL as libc::c_int, 1 as libc::c_int);
                } else {
                    gdbm_set_errno(dbf, rc, 0 as libc::c_int);
                }
                rc = -(1 as libc::c_int);
                break;
            } else if gdbm_file_seek(dbf, n, 0 as libc::c_int) != n {
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_SEEK_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                rc = -(1 as libc::c_int);
                break;
            } else if _gdbm_avail_block_read(dbf, blk, size) != 0 {
                rc = -(1 as libc::c_int);
                break;
            } else {
                if cb.is_some()
                    && cb.expect("non-null function pointer")(blk, n, data) != 0
                {
                    break;
                }
                n = (*blk).next_block;
            }
        }
    }
    free(blk as *mut libc::c_void);
    off_map_free(&mut map);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_avail_verify(mut dbf: GDBM_FILE) -> libc::c_int {
    return gdbm_avail_traverse(dbf, None, 0 as *mut libc::c_void);
}

//version.rs
#[no_mangle]
pub static mut gdbm_version: *const libc::c_char = b"GDBM version 1.23. 04/02/2022 (built Dec 22 2022 11:02:40)\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut gdbm_version_number: [libc::c_int; 3] = [
    1 as libc::c_int,
    23 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn gdbm_version_cmp(
    mut a: *const libc::c_int,
    mut b: *const libc::c_int,
) -> libc::c_int {
    if *a.offset(0 as libc::c_int as isize) > *b.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int
    } else {
        if *a.offset(0 as libc::c_int as isize) < *b.offset(0 as libc::c_int as isize) {
            return -(1 as libc::c_int);
        }
    }
    if *a.offset(1 as libc::c_int as isize) > *b.offset(1 as libc::c_int as isize) {
        return 1 as libc::c_int
    } else {
        if *a.offset(1 as libc::c_int as isize) < *b.offset(1 as libc::c_int as isize) {
            return -(1 as libc::c_int);
        }
    }
    if *a.offset(2 as libc::c_int as isize) > *b.offset(2 as libc::c_int as isize) {
        return 1 as libc::c_int
    } else {
        if *a.offset(2 as libc::c_int as isize) < *b.offset(2 as libc::c_int as isize) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}

//gdbmclose.rs

#[no_mangle]
pub unsafe extern "C" fn gdbm_close(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut syserrno: libc::c_int = 0;
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    if (*dbf).desc != -(1 as libc::c_int) {
        if (*dbf).read_write() as libc::c_int != 0 as libc::c_int {
            gdbm_file_sync(dbf);
        }
        _gdbmsync_done(dbf);
        _gdbm_mapped_unmap(dbf);
        if (*dbf).file_locking() != 0 {
            _gdbm_unlock_file(dbf);
        }
        if close((*dbf).desc) != 0 {
            gdbm_set_errno(dbf, GDBM_FILE_CLOSE_ERROR as libc::c_int, 0 as libc::c_int);
        }
    }
    syserrno = gdbm_last_syserr(dbf);
    gdbm_clear_error(dbf);
    free((*dbf).name as *mut libc::c_void);
    free((*dbf).dir as *mut libc::c_void);
    _gdbm_cache_free(dbf);
    free((*dbf).header as *mut libc::c_void);
    free(dbf as *mut libc::c_void);
    if *gdbm_errno_location() != 0 {
        *__errno_location() = syserrno;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}

//gdbmopen.rs


#[inline]
unsafe extern "C" fn gdbm_file_seek(
    mut dbf: GDBM_FILE,
    mut off: off_t,
    mut whence: libc::c_int,
) -> off_t {
    return _gdbm_mapped_lseek(dbf, off, whence);
}







unsafe extern "C" fn compute_directory_size(
    mut block_size: blksize_t,
    mut ret_dir_size: *mut libc::c_int,
    mut ret_dir_bits: *mut libc::c_int,
) {
    let mut dir_size: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<off_t>() as libc::c_ulong) as libc::c_int;
    let mut dir_bits: libc::c_int = 3 as libc::c_int;
    if block_size > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_long {
        block_size = (2147483647 as libc::c_int / 2 as libc::c_int) as blksize_t;
    }
    while (dir_size as libc::c_long) < block_size
        && dir_bits < 31 as libc::c_int - 3 as libc::c_int
    {
        dir_size <<= 1 as libc::c_int;
        dir_bits += 1;
    }
    *ret_dir_size = dir_size;
    *ret_dir_bits = dir_bits;
}
#[inline]
unsafe extern "C" fn bucket_element_count(mut bucket_size: size_t) -> libc::c_int {
    return bucket_size
        .wrapping_sub(::std::mem::size_of::<hash_bucket>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<bucket_element>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn gdbm_header_avail(
    mut hdr: *mut gdbm_file_header,
    mut avail_ptr: *mut *mut avail_block,
    mut avail_size: *mut size_t,
    mut exhdr: *mut *mut gdbm_ext_header,
) {
    match (*hdr).header_magic {
        324508366 | 324508367 => {
            *exhdr = 0 as *mut gdbm_ext_header;
            *avail_ptr = &mut (*(hdr as *mut gdbm_file_standard_header)).avail;
            *avail_size = ((*hdr).block_size as libc::c_ulong)
                .wrapping_sub(40 as libc::c_ulong);
        }
        324508369 => {
            *exhdr = &mut (*(hdr as *mut gdbm_file_extended_header)).ext;
            *avail_ptr = &mut (*(hdr as *mut gdbm_file_extended_header)).avail;
            *avail_size = ((*hdr).block_size as libc::c_ulong)
                .wrapping_sub(72 as libc::c_ulong);
        }
        _ => {}
    };
}
unsafe extern "C" fn validate_header_std(
    mut hdr: *const gdbm_file_header,
    mut st: *const stat,
) -> libc::c_int {
    let mut result: libc::c_int = GDBM_NO_ERROR as libc::c_int;
    let mut dir_size: libc::c_int = 0;
    let mut dir_bits: libc::c_int = 0;
    if !((*hdr).block_size > 0 as libc::c_int
        && (*hdr).block_size as libc::c_ulong
            > ::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong
        && ((*hdr).block_size as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong)
            >= ::std::mem::size_of::<avail_block>() as libc::c_ulong)
    {
        return GDBM_BLOCK_SIZE_ERROR as libc::c_int;
    }
    if (*hdr).next_block < (*st).st_size {
        result = GDBM_NEED_RECOVERY as libc::c_int;
    }
    if !((*hdr).dir > 0 as libc::c_int as libc::c_long && (*hdr).dir < (*st).st_size
        && (*hdr).dir_size > 0 as libc::c_int
        && ((*hdr).dir + (*hdr).dir_size as libc::c_long) < (*st).st_size)
    {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    compute_directory_size((*hdr).block_size as blksize_t, &mut dir_size, &mut dir_bits);
    if !((*hdr).dir_size >= dir_size) {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    compute_directory_size((*hdr).dir_size as blksize_t, &mut dir_size, &mut dir_bits);
    if (*hdr).dir_bits != dir_bits {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    if !((*hdr).bucket_size > 0 as libc::c_int
        && (*hdr).bucket_size as libc::c_ulong
            > ::std::mem::size_of::<hash_bucket>() as libc::c_ulong)
    {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    if (*hdr).bucket_elems != bucket_element_count((*hdr).bucket_size as size_t) {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn validate_header_numsync(
    mut hdr: *const gdbm_file_header,
    mut st: *const stat,
) -> libc::c_int {
    let mut result: libc::c_int = GDBM_NO_ERROR as libc::c_int;
    let mut dir_size: libc::c_int = 0;
    let mut dir_bits: libc::c_int = 0;
    if !((*hdr).block_size > 0 as libc::c_int
        && (*hdr).block_size as libc::c_ulong
            > (::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<gdbm_ext_header>() as libc::c_ulong)
        && ((*hdr).block_size as libc::c_ulong)
            .wrapping_sub(
                (::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<gdbm_ext_header>() as libc::c_ulong,
                    ),
            ) >= ::std::mem::size_of::<avail_block>() as libc::c_ulong)
    {
        return GDBM_BLOCK_SIZE_ERROR as libc::c_int;
    }
    if (*hdr).next_block < (*st).st_size {
        result = GDBM_NEED_RECOVERY as libc::c_int;
    }
    if !((*hdr).dir > 0 as libc::c_int as libc::c_long && (*hdr).dir < (*st).st_size
        && (*hdr).dir_size > 0 as libc::c_int
        && ((*hdr).dir + (*hdr).dir_size as libc::c_long) < (*st).st_size)
    {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    compute_directory_size((*hdr).block_size as blksize_t, &mut dir_size, &mut dir_bits);
    if !((*hdr).dir_size >= dir_size) {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    compute_directory_size((*hdr).dir_size as blksize_t, &mut dir_size, &mut dir_bits);
    if (*hdr).dir_bits != dir_bits {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    if !((*hdr).bucket_size > 0 as libc::c_int
        && (*hdr).bucket_size as libc::c_ulong
            > ::std::mem::size_of::<hash_bucket>() as libc::c_ulong)
    {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    if (*hdr).bucket_elems != bucket_element_count((*hdr).bucket_size as size_t) {
        return GDBM_BAD_HEADER as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn validate_header(
    mut hdr: *const gdbm_file_header,
    mut st: *const stat,
) -> libc::c_int {
    match (*hdr).header_magic {
        324508366 | 324508367 => return validate_header_std(hdr, st),
        324508369 => return validate_header_numsync(hdr, st),
        _ => {
            match (*hdr).header_magic {
                -828745965 | -845523181 | -811968749 | -795191533 | -778414317 => {
                    return GDBM_BYTE_SWAPPED as libc::c_int;
                }
                324508365 | 324508367 | 324508368 | 324508369 => {
                    return GDBM_BAD_FILE_OFFSET as libc::c_int;
                }
                _ => return GDBM_BAD_MAGIC_NUMBER as libc::c_int,
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_validate_header(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rc: libc::c_int = 0;
    if fstat((*dbf).desc, &mut file_stat) != 0 {
        return GDBM_FILE_STAT_ERROR as libc::c_int;
    }
    rc = validate_header((*dbf).header, &mut file_stat);
    if rc == 0 as libc::c_int {
        if gdbm_avail_block_validate(dbf, (*dbf).avail, (*dbf).avail_size) != 0 {
            rc = GDBM_BAD_AVAIL as libc::c_int;
        }
    }
    return rc;
}
#[inline]
unsafe extern "C" fn _gdbm_ftruncate(mut dbf: GDBM_FILE) -> libc::c_int {
    return ftruncate((*dbf).desc, 0 as libc::c_int as __off_t);
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_fd_open(
    mut fd: libc::c_int,
    mut file_name: *const libc::c_char,
    mut block_size: libc::c_int,
    mut flags: libc::c_int,
    mut fatal_func: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
) -> GDBM_FILE {
    let mut dbf: GDBM_FILE = 0 as *mut gdbm_file_info;
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut file_pos: off_t = 0;
    let mut index: libc::c_int = 0;
    gdbm_set_errno(0 as GDBM_FILE, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    if fstat(fd, &mut file_stat) != 0 {
        if flags & 0x400 as libc::c_int != 0 {
            let mut __gc: libc::c_int = *gdbm_errno_location();
            let mut __ec: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = __ec;
            *gdbm_errno_location() = __gc;
        }
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_STAT_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return 0 as GDBM_FILE;
    }
    dbf = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gdbm_file_info>() as libc::c_ulong,
    ) as GDBM_FILE;
    if dbf.is_null() {
        if flags & 0x400 as libc::c_int != 0 {
            let mut __gc_0: libc::c_int = *gdbm_errno_location();
            let mut __ec_0: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = __ec_0;
            *gdbm_errno_location() = __gc_0;
        }
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_MALLOC_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return 0 as GDBM_FILE;
    }
    (*dbf).desc = fd;
    let ref mut fresh1 = (*dbf).dir;
    *fresh1 = 0 as *mut off_t;
    let ref mut fresh2 = (*dbf).bucket;
    *fresh2 = 0 as *mut hash_bucket;
    let ref mut fresh3 = (*dbf).header;
    *fresh3 = 0 as *mut gdbm_file_header;
    (*dbf).file_size = -(1 as libc::c_int) as off_t;
    (*dbf).set_memory_mapping(0 as libc::c_int as libc::c_uint);
    (*dbf).mapped_size_max = -(1 as libc::c_int) as size_t;
    let ref mut fresh4 = (*dbf).mapped_region;
    *fresh4 = 0 as *mut libc::c_void;
    (*dbf).mapped_size = 0 as libc::c_int as size_t;
    (*dbf).mapped_pos = 0 as libc::c_int as off_t;
    (*dbf).mapped_off = 0 as libc::c_int as off_t;
    let ref mut fresh5 = (*dbf).name;
    *fresh5 = strdup(file_name);
    if ((*dbf).name).is_null() {
        if flags & 0x400 as libc::c_int != 0 {
            close(fd);
        }
        _gdbm_cache_free(dbf);
        free(dbf as *mut libc::c_void);
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_MALLOC_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return 0 as GDBM_FILE;
    }
    let ref mut fresh6 = (*dbf).fatal_err;
    *fresh6 = fatal_func;
    (*dbf).set_fast_write(1 as libc::c_int as libc::c_uint);
    (*dbf).set_file_locking(1 as libc::c_int as libc::c_uint);
    (*dbf).set_central_free(0 as libc::c_int as libc::c_uint);
    (*dbf).set_coalesce_blocks(0 as libc::c_int as libc::c_uint);
    (*dbf).set_need_recovery(0 as libc::c_int as libc::c_uint);
    (*dbf).last_error = GDBM_NO_ERROR as libc::c_int;
    (*dbf).last_syserror = 0 as libc::c_int;
    let ref mut fresh7 = (*dbf).last_errstr;
    *fresh7 = 0 as *mut libc::c_char;
    _gdbmsync_init(dbf);
    if flags & 0x20 as libc::c_int != 0 {
        (*dbf).set_fast_write(0 as libc::c_int as libc::c_uint);
    }
    if flags & 0x40 as libc::c_int != 0 {
        (*dbf).set_file_locking(0 as libc::c_int as libc::c_uint);
    }
    (*dbf)
        .set_cloexec((flags & 0x100 as libc::c_int != 0) as libc::c_int as libc::c_uint);
    if flags & 7 as libc::c_int == 0 as libc::c_int
        && file_stat.st_size == 0 as libc::c_int as libc::c_long
    {
        if flags & 0x400 as libc::c_int == 0 {
            (*dbf).desc = -(1 as libc::c_int);
        }
        gdbm_close(dbf);
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_EMPTY_DATABASE as libc::c_int,
            0 as libc::c_int,
        );
        return 0 as GDBM_FILE;
    }
    (*dbf).set_read_write((flags & 7 as libc::c_int) as libc::c_uint);
    if (*dbf).file_locking() != 0 {
        if _gdbm_lock_file(dbf) == -(1 as libc::c_int) {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_1: libc::c_int = *gdbm_errno_location();
            let mut __ec_1: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_1;
            *gdbm_errno_location() = __gc_1;
            gdbm_set_errno(
                0 as GDBM_FILE,
                if flags & 7 as libc::c_int == 0 as libc::c_int {
                    GDBM_CANT_BE_READER as libc::c_int
                } else {
                    GDBM_CANT_BE_WRITER as libc::c_int
                },
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
    }
    if flags & 7 as libc::c_int == 3 as libc::c_int
        && file_stat.st_size != 0 as libc::c_int as libc::c_long
    {
        if _gdbm_ftruncate(dbf) != 0 {
            gdbm_set_errno(
                dbf,
                GDBM_FILE_TRUNCATE_ERROR as libc::c_int,
                0 as libc::c_int,
            );
        } else if fstat((*dbf).desc, &mut file_stat) != 0 {
            gdbm_set_errno(dbf, GDBM_FILE_STAT_ERROR as libc::c_int, 0 as libc::c_int);
        }
        if gdbm_last_errno(dbf) != 0 {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_2: libc::c_int = *gdbm_errno_location();
            let mut __ec_2: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_2;
            *gdbm_errno_location() = __gc_2;
            return 0 as GDBM_FILE;
        }
    }
    if file_stat.st_size == 0 as libc::c_int as libc::c_long {
        let mut dir_size: libc::c_int = 0;
        let mut dir_bits: libc::c_int = 0;
        if block_size < 512 as libc::c_int {
            block_size = file_stat.st_blksize as libc::c_int;
            flags &= !(0x200 as libc::c_int);
        }
        compute_directory_size(block_size as blksize_t, &mut dir_size, &mut dir_bits);
        if dir_size != block_size {
            if flags & 0x200 as libc::c_int != 0 {
                if flags & 0x400 as libc::c_int == 0 {
                    (*dbf).desc = -(1 as libc::c_int);
                }
                gdbm_close(dbf);
                gdbm_set_errno(
                    0 as GDBM_FILE,
                    GDBM_BLOCK_SIZE_ERROR as libc::c_int,
                    0 as libc::c_int,
                );
                return 0 as GDBM_FILE;
            } else {
                block_size = dir_size;
            }
        }
        let ref mut fresh8 = (*dbf).header;
        *fresh8 = calloc(1 as libc::c_int as libc::c_ulong, block_size as libc::c_ulong)
            as *mut gdbm_file_header;
        if ((*dbf).header).is_null() {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        if flags & 0x2000 as libc::c_int != 0 {
            (*(*dbf).header).header_magic = 0x13579ad1 as libc::c_uint as libc::c_int;
        } else {
            (*(*dbf).header).header_magic = 0x13579acf as libc::c_uint as libc::c_int;
        }
        (*(*dbf).header).block_size = block_size;
        gdbm_header_avail(
            (*dbf).header,
            &mut (*dbf).avail,
            &mut (*dbf).avail_size,
            &mut (*dbf).xheader,
        );
        (*(*dbf).header).dir_size = dir_size;
        (*(*dbf).header).dir_bits = dir_bits;
        let ref mut fresh9 = (*dbf).dir;
        *fresh9 = malloc((*(*dbf).header).dir_size as libc::c_ulong) as *mut off_t;
        if ((*dbf).dir).is_null() {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        (*(*dbf).header).dir = (*(*dbf).header).block_size as off_t;
        (*(*dbf).header)
            .bucket_elems = bucket_element_count((*(*dbf).header).block_size as size_t);
        (*(*dbf).header).bucket_size = (*(*dbf).header).block_size;
        let ref mut fresh10 = (*dbf).bucket;
        *fresh10 = calloc(
            1 as libc::c_int as libc::c_ulong,
            (*(*dbf).header).bucket_size as libc::c_ulong,
        ) as *mut hash_bucket;
        if ((*dbf).bucket).is_null() {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        _gdbm_new_bucket(dbf, (*dbf).bucket, 0 as libc::c_int);
        (*(*dbf).bucket).av_count = 1 as libc::c_int;
        (*(*dbf).bucket)
            .bucket_avail[0 as libc::c_int as usize]
            .av_adr = (3 as libc::c_int * (*(*dbf).header).block_size) as off_t;
        (*(*dbf).bucket)
            .bucket_avail[0 as libc::c_int as usize]
            .av_size = (*(*dbf).header).block_size;
        index = 0 as libc::c_int;
        while (index as libc::c_ulong)
            < ((*(*dbf).header).dir_size as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
        {
            *((*dbf).dir)
                .offset(
                    index as isize,
                ) = (2 as libc::c_int * (*(*dbf).header).block_size) as off_t;
            index += 1;
        }
        (*(*dbf).avail)
            .size = ((*dbf).avail_size)
            .wrapping_sub(16 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
            as libc::c_int;
        (*(*dbf).avail).count = 0 as libc::c_int;
        (*(*dbf).avail).next_block = 0 as libc::c_int as off_t;
        (*(*dbf).header)
            .next_block = (4 as libc::c_int * (*(*dbf).header).block_size) as off_t;
        if _gdbm_full_write(
            dbf,
            (*dbf).header as *mut libc::c_void,
            (*(*dbf).header).block_size as size_t,
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_3: libc::c_int = *gdbm_errno_location();
            let mut __ec_3: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_3;
            *gdbm_errno_location() = __gc_3;
            return 0 as GDBM_FILE;
        }
        if _gdbm_full_write(
            dbf,
            (*dbf).dir as *mut libc::c_void,
            (*(*dbf).header).dir_size as size_t,
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_4: libc::c_int = *gdbm_errno_location();
            let mut __ec_4: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_4;
            *gdbm_errno_location() = __gc_4;
            return 0 as GDBM_FILE;
        }
        if _gdbm_full_write(
            dbf,
            (*dbf).bucket as *mut libc::c_void,
            (*(*dbf).header).bucket_size as size_t,
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_5: libc::c_int = *gdbm_errno_location();
            let mut __ec_5: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_5;
            *gdbm_errno_location() = __gc_5;
            return 0 as GDBM_FILE;
        }
        if _gdbm_file_extend(dbf, (*(*dbf).header).next_block) != 0 {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_6: libc::c_int = *gdbm_errno_location();
            let mut __ec_6: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_6;
            *gdbm_errno_location() = __gc_6;
            return 0 as GDBM_FILE;
        }
        gdbm_file_sync(dbf);
        free((*dbf).bucket as *mut libc::c_void);
    } else {
        let mut partial_header: gdbm_file_header = gdbm_file_header {
            header_magic: 0,
            block_size: 0,
            dir: 0,
            dir_size: 0,
            dir_bits: 0,
            bucket_size: 0,
            bucket_elems: 0,
            next_block: 0,
        };
        let mut rc: libc::c_int = 0;
        if _gdbm_full_read(
            dbf,
            &mut partial_header as *mut gdbm_file_header as *mut libc::c_void,
            ::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong,
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_7: libc::c_int = *gdbm_errno_location();
            let mut __ec_7: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_7;
            *gdbm_errno_location() = __gc_7;
            return 0 as GDBM_FILE;
        }
        rc = validate_header(&mut partial_header, &mut file_stat);
        if rc == GDBM_NEED_RECOVERY as libc::c_int {
            (*dbf).set_need_recovery(1 as libc::c_int as libc::c_uint);
        } else if rc != GDBM_NO_ERROR as libc::c_int {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(0 as GDBM_FILE, rc, 0 as libc::c_int);
            return 0 as GDBM_FILE;
        }
        let ref mut fresh11 = (*dbf).header;
        *fresh11 = malloc(partial_header.block_size as libc::c_ulong)
            as *mut gdbm_file_header;
        if ((*dbf).header).is_null() {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_8: libc::c_int = *gdbm_errno_location();
            let mut __ec_8: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_8;
            *gdbm_errno_location() = __gc_8;
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        memcpy(
            (*dbf).header as *mut libc::c_void,
            &mut partial_header as *mut gdbm_file_header as *const libc::c_void,
            ::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong,
        );
        if _gdbm_full_read(
            dbf,
            ((*dbf).header).offset(1 as libc::c_int as isize) as *mut libc::c_void,
            ((*(*dbf).header).block_size as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<gdbm_file_header>() as libc::c_ulong),
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_9: libc::c_int = *gdbm_errno_location();
            let mut __ec_9: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_9;
            *gdbm_errno_location() = __gc_9;
            return 0 as GDBM_FILE;
        }
        gdbm_header_avail(
            (*dbf).header,
            &mut (*dbf).avail,
            &mut (*dbf).avail_size,
            &mut (*dbf).xheader,
        );
        if ((*(*dbf).header).block_size as libc::c_ulong)
            .wrapping_sub(
                (((*dbf).avail as *mut libc::c_char)
                    .offset_from((*dbf).header as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<avail_block>() as libc::c_ulong),
            )
            .wrapping_div(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            != (*(*dbf).avail).size as libc::c_ulong
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_BAD_HEADER as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        if gdbm_avail_block_validate(dbf, (*dbf).avail, (*dbf).avail_size) != 0 {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_10: libc::c_int = *gdbm_errno_location();
            let mut __ec_10: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_10;
            *gdbm_errno_location() = __gc_10;
            return 0 as GDBM_FILE;
        }
        let ref mut fresh12 = (*dbf).dir;
        *fresh12 = malloc((*(*dbf).header).dir_size as libc::c_ulong) as *mut off_t;
        if ((*dbf).dir).is_null() {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            gdbm_close(dbf);
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_MALLOC_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        file_pos = gdbm_file_seek(dbf, (*(*dbf).header).dir, 0 as libc::c_int);
        if file_pos != (*(*dbf).header).dir {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_11: libc::c_int = *gdbm_errno_location();
            let mut __ec_11: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_11;
            *gdbm_errno_location() = __gc_11;
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_FILE_SEEK_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
        if _gdbm_full_read(
            dbf,
            (*dbf).dir as *mut libc::c_void,
            (*(*dbf).header).dir_size as size_t,
        ) != 0
        {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_12: libc::c_int = *gdbm_errno_location();
            let mut __ec_12: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_12;
            *gdbm_errno_location() = __gc_12;
            return 0 as GDBM_FILE;
        }
    }
    if _gdbm_cache_init(dbf, 0 as libc::c_int as size_t) != 0 {
        if flags & 0x400 as libc::c_int == 0 {
            (*dbf).desc = -(1 as libc::c_int);
        }
        let mut __gc_13: libc::c_int = *gdbm_errno_location();
        let mut __ec_13: libc::c_int = *__errno_location();
        gdbm_close(dbf);
        *__errno_location() = __ec_13;
        *gdbm_errno_location() = __gc_13;
        return 0 as GDBM_FILE;
    }
    if flags & 0x80 as libc::c_int == 0 {
        (*dbf)
            .set_mmap_preread(
                (flags & 0x1000 as libc::c_int != 0 as libc::c_int) as libc::c_int,
            );
        if _gdbm_mapped_init(dbf) == 0 as libc::c_int {
            (*dbf).set_memory_mapping(1 as libc::c_int as libc::c_uint);
        } else {
            if flags & 0x400 as libc::c_int == 0 {
                (*dbf).desc = -(1 as libc::c_int);
            }
            let mut __gc_14: libc::c_int = *gdbm_errno_location();
            let mut __ec_14: libc::c_int = *__errno_location();
            gdbm_close(dbf);
            *__errno_location() = __ec_14;
            *gdbm_errno_location() = __gc_14;
            return 0 as GDBM_FILE;
        }
    }
    let ref mut fresh13 = (*dbf).bucket;
    *fresh13 = 0 as *mut hash_bucket;
    (*dbf).bucket_dir = 0 as libc::c_int;
    (*dbf).set_header_changed(0 as libc::c_int as libc::c_uint);
    (*dbf).set_directory_changed(0 as libc::c_int as libc::c_uint);
    if flags & 0x800 as libc::c_int != 0 {
        gdbm_avail_verify(dbf);
    }
    return dbf;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_open(
    mut file: *const libc::c_char,
    mut block_size: libc::c_int,
    mut flags: libc::c_int,
    mut mode: libc::c_int,
    mut fatal_func: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
) -> GDBM_FILE {
    let mut fd: libc::c_int = 0;
    let mut fbits: libc::c_int = 0 as libc::c_int;
    match flags & 7 as libc::c_int {
        0 => {
            fbits = 0 as libc::c_int;
        }
        1 => {
            fbits = 0o2 as libc::c_int;
        }
        2 | 3 => {
            fbits = 0o2 as libc::c_int | 0o100 as libc::c_int;
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            gdbm_set_errno(
                0 as GDBM_FILE,
                GDBM_FILE_OPEN_ERROR as libc::c_int,
                0 as libc::c_int,
            );
            return 0 as GDBM_FILE;
        }
    }
    if flags & 0x100 as libc::c_int != 0 {
        fbits |= 0o2000000 as libc::c_int;
    }
    fd = open(file, fbits, mode);
    if fd < 0 as libc::c_int {
        gdbm_set_errno(
            0 as GDBM_FILE,
            GDBM_FILE_OPEN_ERROR as libc::c_int,
            0 as libc::c_int,
        );
        return 0 as GDBM_FILE;
    }
    return gdbm_fd_open(fd, file, block_size, flags | 0x400 as libc::c_int, fatal_func);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_file_size(
    mut dbf: GDBM_FILE,
    mut psize: *mut off_t,
) -> libc::c_int {
    if (*dbf).file_size == -(1 as libc::c_int) as libc::c_long {
        let mut sb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if fstat((*dbf).desc, &mut sb) != 0 {
            gdbm_set_errno(dbf, GDBM_FILE_STAT_ERROR as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        (*dbf).file_size = sb.st_size;
    }
    *psize = (*dbf).file_size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _gdbm_convert_from_numsync(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut old_avail: *mut avail_block = (*dbf).avail;
    (*(*dbf).header).header_magic = 0x13579acf as libc::c_uint as libc::c_int;
    gdbm_header_avail(
        (*dbf).header,
        &mut (*dbf).avail,
        &mut (*dbf).avail_size,
        &mut (*dbf).xheader,
    );
    memmove(
        (*dbf).avail as *mut libc::c_void,
        old_avail as *const libc::c_void,
        ((*dbf).avail_size)
            .wrapping_sub(::std::mem::size_of::<gdbm_ext_header>() as libc::c_ulong),
    );
    (*(*dbf).avail)
        .size = ((*dbf).avail_size)
        .wrapping_sub(16 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
        as libc::c_int;
    (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _gdbm_convert_to_numsync(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut old_avail: *mut avail_block = (*dbf).avail;
    let mut old_avail_size: size_t = (*(*dbf).avail).size as size_t;
    let mut n: size_t = 0;
    let mut rc: libc::c_int = 0;
    let mut av: *mut avail_elem = 0 as *mut avail_elem;
    (*(*dbf).header).header_magic = 0x13579ad1 as libc::c_uint as libc::c_int;
    gdbm_header_avail(
        (*dbf).header,
        &mut (*dbf).avail,
        &mut (*dbf).avail_size,
        &mut (*dbf).xheader,
    );
    (*old_avail)
        .size = ((*dbf).avail_size)
        .wrapping_sub(16 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<avail_elem>() as libc::c_ulong)
        as libc::c_int;
    n = old_avail_size.wrapping_sub((*old_avail).size as libc::c_ulong);
    if n > 0 as libc::c_int as libc::c_ulong {
        av = calloc(n, ::std::mem::size_of::<avail_elem>() as libc::c_ulong)
            as *mut avail_elem;
        if av.is_null() {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        n = 0 as libc::c_int as size_t;
        while (*old_avail).count > (*old_avail).size {
            let ref mut fresh14 = (*old_avail).count;
            *fresh14 -= 1;
            let fresh15 = n;
            n = n.wrapping_add(1);
            *av
                .offset(
                    fresh15 as isize,
                ) = *((*old_avail).av_table)
                .as_mut_ptr()
                .offset((*old_avail).count as isize);
        }
    }
    memmove(
        (*dbf).avail as *mut libc::c_void,
        old_avail as *const libc::c_void,
        (*dbf).avail_size,
    );
    memset(
        (*dbf).xheader as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gdbm_ext_header>() as libc::c_ulong,
    );
    rc = 0 as libc::c_int;
    if !av.is_null() {
        if ((*dbf).bucket).is_null() {
            rc = _gdbm_get_bucket(dbf, 0 as libc::c_int);
        }
        if rc == 0 as libc::c_int {
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < n {
                rc = _gdbm_free(
                    dbf,
                    (*av.offset(i as isize)).av_adr,
                    (*av.offset(i as isize)).av_size,
                );
                if rc != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        free(av as *mut libc::c_void);
    }
    (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_convert(
    mut dbf: GDBM_FILE,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if (*dbf).read_write() as libc::c_int == 0 as libc::c_int {
        gdbm_set_errno(dbf, GDBM_READER_CANT_STORE as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    }
    match flag {
        0 | 8192 => {}
        _ => {
            gdbm_set_errno(dbf, GDBM_MALFORMED_DATA as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
    }
    rc = 0 as libc::c_int;
    match (*(*dbf).header).header_magic {
        324508366 | 324508367 => {
            if flag == 0x2000 as libc::c_int {
                rc = _gdbm_convert_to_numsync(dbf);
            }
        }
        324508369 => {
            if flag == 0 as libc::c_int {
                rc = _gdbm_convert_from_numsync(dbf);
            }
        }
        _ => {}
    }
    if rc == 0 as libc::c_int {
        rc = _gdbm_end_update(dbf);
    }
    return 0 as libc::c_int;
}

//update.rs
unsafe extern "C" fn write_header(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut file_pos: off_t = 0;
    let mut rc: libc::c_int = 0;
    file_pos = gdbm_file_seek(dbf, 0 as libc::c_long, 0 as libc::c_int);
    if file_pos != 0 as libc::c_int as libc::c_long {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"lseek error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    rc = _gdbm_full_write(
        dbf,
        (*dbf).header as *mut libc::c_void,
        (*(*dbf).header).block_size as size_t,
    );
    if rc != 0 {
        return -(1 as libc::c_int);
    }
    if (*dbf).fast_write() as libc::c_int == 0 as libc::c_int {
        gdbm_file_sync(dbf);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_end_update(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut file_pos: off_t = 0;
    let mut rc: libc::c_int = 0;
    _gdbm_cache_flush(dbf);
    if (*dbf).directory_changed() != 0 {
        file_pos = gdbm_file_seek(dbf, (*(*dbf).header).dir, 0 as libc::c_int);
        if file_pos != (*(*dbf).header).dir {
            gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
            _gdbm_fatal(
                dbf,
                dcgettext(
                    b"gdbm\0" as *const u8 as *const libc::c_char,
                    b"lseek error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        rc = _gdbm_full_write(
            dbf,
            (*dbf).dir as *mut libc::c_void,
            (*(*dbf).header).dir_size as size_t,
        );
        if rc != 0 {
            _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
            return -(1 as libc::c_int);
        }
        (*dbf).set_directory_changed(0 as libc::c_int as libc::c_uint);
        if (*dbf).header_changed() == 0
            && (*dbf).fast_write() as libc::c_int == 0 as libc::c_int
        {
            gdbm_file_sync(dbf);
        }
    }
    if (*dbf).header_changed() != 0 {
        if write_header(dbf) != 0 {
            return -(1 as libc::c_int);
        }
        if _gdbm_file_extend(dbf, (*(*dbf).header).next_block) != 0 {
            return -(1 as libc::c_int);
        }
        (*dbf).set_header_changed(0 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_fatal(mut dbf: GDBM_FILE, mut val: *const libc::c_char) {
    if !dbf.is_null() && ((*dbf).fatal_err).is_some() {
        (Some(((*dbf).fatal_err).expect("non-null function pointer")))
            .expect("non-null function pointer")(val);
        exit(1 as libc::c_int);
    }
}

//gdbmsync.rs
#[inline]
unsafe extern "C" fn _gdbmsync_init(mut dbf: GDBM_FILE) {
    (*dbf).snapfd[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*dbf).snapfd[0 as libc::c_int as usize] =
        (*dbf).snapfd[1 as libc::c_int as usize];
    (*dbf).eo = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _gdbmsync_done(mut dbf: GDBM_FILE) {
    if (*dbf).snapfd[0 as libc::c_int as usize] >= 0 as libc::c_int {
        close((*dbf).snapfd[0 as libc::c_int as usize]);
    }
    if (*dbf).snapfd[1 as libc::c_int as usize] >= 0 as libc::c_int {
        close((*dbf).snapfd[1 as libc::c_int as usize]);
    };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
/* gdbmsync.c - Sync the disk with the in memory state. */
/* This file is part of GDBM, the GNU data base manager.
   Copyright (C) 1990-2022 Free Software Foundation, Inc.

   GDBM is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GDBM is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GDBM. If not, see <http://www.gnu.org/licenses/>.   */
/* Include system configuration before all else. */
/* Sometimes, to ensure durability, a new file *and* all directories
   on its full path must be fsync()'d up to the root directory.  */
unsafe extern "C" fn fsync_to_root(mut f: *const libc::c_char)
 -> libc::c_int {
    let mut flags: libc::c_int = 0o1 as libc::c_int;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if realpath(f, path.as_mut_ptr()).is_null() {
        return GDBM_ERR_REALPATH as libc::c_int
    }
    end = path.as_mut_ptr().offset(strlen(path.as_mut_ptr()) as isize);
    while path.as_mut_ptr() < end {
        let mut fd: libc::c_int = 0;
        *end = 0 as libc::c_int as libc::c_char;
        fd = open(path.as_mut_ptr(), flags);
        flags = 0 as libc::c_int;
        if fd == -(1 as libc::c_int) {
            return GDBM_FILE_OPEN_ERROR as libc::c_int
        }
        if fsync(fd) != 0 {
            let mut ec: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = ec;
            return GDBM_FILE_SYNC_ERROR as libc::c_int
        }
        if close(fd) != 0 { return GDBM_FILE_CLOSE_ERROR as libc::c_int }
        loop  {
            end = end.offset(-1);
            if !(path.as_mut_ptr() < end &&
                     *end.offset(-(1 as libc::c_int) as isize) as libc::c_int
                         != '/' as i32) {
                break ;
            }
        }
    }
    return GDBM_NO_ERROR as libc::c_int;
}
/* Note:  Valgrind complains about ioctl() call below, but it appears
   that Valgrind is simply confused; it issues similar complaints
   about very simple and correct uses of ioctl(FICLONE). */
#[no_mangle]
pub unsafe extern "C" fn _gdbm_snapshot(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut s: libc::c_int = 0; /* snapshot file descriptor */
    let mut oldsnap: libc::c_int = 0; /* previous snapshot file descriptor */
    if (*dbf).snapfd[0 as libc::c_int as usize] < 0 as libc::c_int {
        /* crash consistency hasn't been requested on this database */
        return 0 as libc::c_int
    }
    if !((*dbf).eo == 0 as libc::c_int || (*dbf).eo == 1 as libc::c_int) {
        /* Shouldn't happen, but still... */
        _gdbmsync_done(dbf);
        _gdbmsync_init(dbf);
        gdbm_set_errno(dbf, GDBM_ERR_USAGE as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    s = (*dbf).snapfd[(*dbf).eo as usize];
    (*dbf).eo = ((*dbf).eo == 0) as libc::c_int;
    oldsnap = (*dbf).snapfd[(*dbf).eo as usize];
    /* says "DON'T recover from this snapshot, writing in progress " */
    if fchmod(s, 0o200 as libc::c_int as __mode_t) != 0 {
        gdbm_set_errno(dbf, GDBM_ERR_FILE_MODE as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* commit permission bits */
    if fsync(s) != 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SYNC_ERROR as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* make efficient reflink copy into snapshot file, overwrite previous
     contents */
    if ioctl(s,
             ((1 as libc::c_uint) <<
                  0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                      14 as libc::c_int |
                  ((0x94 as libc::c_int) <<
                       0 as libc::c_int + 8 as libc::c_int) as libc::c_uint |
                  ((9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as
                 libc::c_ulong |
                 (::std::mem::size_of::<libc::c_int>() as libc::c_ulong) <<
                     0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
             (*dbf).desc) == -(1 as libc::c_int) {
        if *__errno_location() == 22 as libc::c_int ||
               *__errno_location() == 38 as libc::c_int {
            _gdbmsync_done(dbf);
            _gdbmsync_init(dbf);
        }
        gdbm_set_errno(dbf, GDBM_ERR_SNAPSHOT_CLONE as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* commit snapshot data */
    if fsync(s) != 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SYNC_ERROR as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* says "DO recover from this snapshot, writing completed successfully" */
    if fchmod(s, 0o400 as libc::c_int as __mode_t) != 0 {
        gdbm_set_errno(dbf, GDBM_ERR_FILE_MODE as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* commit permission bits again */
    if fsync(s) != 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SYNC_ERROR as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /*
   * Mark the previous snapshot file write-only, indicating thereby
   * that it contains obsolete data.  The point of this additional
   * operation is to reduce the time window during which a crash would
   * leave two readable snapshot files.  
   */
    if fchmod(oldsnap, 0o200 as libc::c_int as __mode_t) != 0 {
        gdbm_set_errno(dbf, GDBM_ERR_FILE_MODE as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* commit permission bits */
    if fsync(oldsnap) != 0 {
        gdbm_set_errno(dbf, GDBM_FILE_SYNC_ERROR as libc::c_int,
                       0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Snapshot files even & odd must not exist already. */
#[no_mangle]
pub unsafe extern "C" fn gdbm_failure_atomic(mut dbf: GDBM_FILE,
                                             mut even: *const libc::c_char,
                                             mut odd: *const libc::c_char)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    /* Return immediately if the database needs recovery */
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int,
                       1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    if even.is_null() || odd.is_null() ||
           strcmp(even, odd) == 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        gdbm_set_errno(dbf, GDBM_ERR_USAGE as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int)
    }
    if (*dbf).snapfd[0 as libc::c_int as usize] != -(1 as libc::c_int) {
        /*
       * This function has been called before for this dbf: reinitialize
       * the snapshot system.
       */
        _gdbmsync_done(dbf);
        _gdbmsync_init(dbf);
    }
    (*dbf).snapfd[0 as libc::c_int as usize] =
        open(even,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
             0o200 as libc::c_int);
    if (*dbf).snapfd[0 as libc::c_int as usize] == -(1 as libc::c_int) {
        gdbm_set_errno(dbf, GDBM_FILE_OPEN_ERROR as libc::c_int,
                       0 as libc::c_int);
    } else {
        (*dbf).snapfd[1 as libc::c_int as usize] =
            open(odd,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o200 as libc::c_int, 0o200 as libc::c_int);
        if (*dbf).snapfd[1 as libc::c_int as usize] == -(1 as libc::c_int) {
            gdbm_set_errno(dbf, GDBM_FILE_OPEN_ERROR as libc::c_int,
                           0 as libc::c_int);
        } else {
            r = fsync_to_root(even);
            if r != 0 as libc::c_int ||
                   { r = fsync_to_root(odd); (r) != 0 as libc::c_int } {
                gdbm_set_errno(dbf, r, 0 as libc::c_int);
            } else {
                (*dbf).eo = 0 as libc::c_int;
                if _gdbm_snapshot(dbf) == 0 as libc::c_int {
                    return 0 as libc::c_int
                }
            }
        }
    }
    _gdbmsync_done(dbf);
    _gdbmsync_init(dbf);
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: *const stat, mut b: *const stat)
 -> libc::c_int {
    if (*a).st_mtim.tv_sec < (*b).st_mtim.tv_sec {
        return -(1 as libc::c_int)
    }
    if (*a).st_mtim.tv_sec > (*b).st_mtim.tv_sec { return 1 as libc::c_int }
    if (*a).st_mtim.tv_nsec < (*b).st_mtim.tv_nsec {
        return -(1 as libc::c_int)
    }
    if (*a).st_mtim.tv_nsec > (*b).st_mtim.tv_nsec { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_snapshot_mode(mut mode: libc::c_int)
 -> libc::c_int {
    if !(mode & 0o170000 as libc::c_int == 0o100000 as libc::c_int) {
        /* file is not a regular file */
        return -(1 as libc::c_int)
    }
    if 0o100 as libc::c_int & mode != 0 {
        /* file is executable */
        return -(1 as libc::c_int)
    } /* file is neither readable nor writable */
    if 0o400 as libc::c_int & mode != 0 {
        if 0o200 as libc::c_int & mode != 0 { return -(1 as libc::c_int) }
        /* file is both readable and writable */
    } else if 0o200 as libc::c_int & mode == 0 { return -(1 as libc::c_int) }
    /* All OK */
    return 0 as libc::c_int;
}
unsafe extern "C" fn stat_snapshot(mut f: *const libc::c_char,
                                   mut st: *mut stat) -> libc::c_int {
    if stat(f, st) != 0 { return -(1 as libc::c_int) }
    if check_snapshot_mode((*st).st_mode as libc::c_int) != 0 {
        *__errno_location() = 13 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gdbm_numsync(mut dbname: *const libc::c_char,
                                  mut numsync: *mut libc::c_uint)
 -> libc::c_int {
    let mut dbf: GDBM_FILE = 0 as *mut gdbm_file_info;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    dbf =
        gdbm_open(dbname, 0 as libc::c_int, 0 as libc::c_int,
                  0o400 as libc::c_int, None);
    if !dbf.is_null() {
        if !(*dbf).xheader.is_null() {
            *numsync = (*(*dbf).xheader).numsync;
            rc = 0 as libc::c_int
        }
        gdbm_close(dbf);
    }
    return rc;
}
/*
 * Return:
 *   0  both numsyncs equal or result undefined
 *  -1  a's numsync is one less than b's
 *  -2  a's numsync is less than b's
 *  +1  a's numsync is one greater than b's
 *  +2  a's numsync is greater than b's
 *
 * Takes into account integer overflow. 
 */
unsafe extern "C" fn gdbm_numsync_cmp(mut a: *const libc::c_char,
                                      mut b: *const libc::c_char)
 -> libc::c_int {
    let mut na: libc::c_uint = 0;
    let mut nb: libc::c_uint = 0;
    if gdbm_numsync(a, &mut na) == 0 as libc::c_int &&
           gdbm_numsync(b, &mut nb) == 0 as libc::c_int {
        if na ==
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
               && nb == 0 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        } else {
            if na == 0 as libc::c_int as libc::c_uint &&
                   nb ==
                       (2147483647 as libc::c_int as
                            libc::c_uint).wrapping_mul(2 as
                                                           libc::c_uint).wrapping_add(1
                                                                                          as
                                                                                          libc::c_uint)
               {
                return 1 as libc::c_int
            } else {
                if na < nb {
                    return if na.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) == nb {
                               -(1 as libc::c_int)
                           } else { -(2 as libc::c_int) }
                } else {
                    if na > nb {
                        return if na ==
                                      nb.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint) {
                                   1 as libc::c_int
                               } else { 2 as libc::c_int }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
/* Selected the right snapshot. */
/* Neither snapshot is readable. */
/* Error selecting snapshot. Inspect errno. */
/* Snapshot numsync and dates are the same. */
/* Selected snapshot is unreliable: numsyncs
				differ by more than 1. */
/*
 * Selects among the two given snapshot files the one to be used for
 * post-crash recovery.
 * Returns one of the GDBM_SNAPSHOT_* constants (see gdbm.h).
 * If GDBM_SNAPSHOT_OK is returned a pointer to the most recent snapshot
 * name is stored in *ret.  Otherwise, *ret is untouched.
 */
#[no_mangle]
pub unsafe extern "C" fn gdbm_latest_snapshot(mut even: *const libc::c_char,
                                              mut odd: *const libc::c_char,
                                              mut ret:
                                                  *mut *const libc::c_char)
 -> libc::c_int {
    let mut st_even: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut st_odd: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if ret.is_null() || even.is_null() || odd.is_null() ||
           strcmp(even, odd) == 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return GDBM_SNAPSHOT_ERR as libc::c_int
    }
    if stat_snapshot(even, &mut st_even) != 0 {
        return GDBM_SNAPSHOT_ERR as libc::c_int
    }
    if stat_snapshot(odd, &mut st_odd) != 0 {
        return GDBM_SNAPSHOT_ERR as libc::c_int
    }
    if st_even.st_mode & 0o400 as libc::c_int as libc::c_uint != 0 {
        let mut rc: libc::c_int = GDBM_SNAPSHOT_OK as libc::c_int;
        if st_odd.st_mode & 0o400 as libc::c_int as libc::c_uint == 0 {
            *ret = even;
            return GDBM_SNAPSHOT_OK as libc::c_int
        }
        /* Both readable: compare numsync value in the extended header.
       * Select the snapshot with greater numsync value.
       */
        match gdbm_numsync_cmp(even, odd) {
            -1 => { *ret = odd }
            -2 => { rc = GDBM_SNAPSHOT_SUSPICIOUS as libc::c_int }
            1 => { *ret = even }
            2 => { rc = GDBM_SNAPSHOT_SUSPICIOUS as libc::c_int }
            _ => {
                /*
	   * Both readable: check mtime.
	   * Select the newer snapshot, i.e. the one whose mtime
	   * is greater than the other's
	   */
                match timespec_cmp(&mut st_even, &mut st_odd) {
                    -1 => { *ret = odd }
                    1 => { *ret = even }
                    0 => {
                        /* Shouldn't happen */
                        rc = GDBM_SNAPSHOT_SAME as libc::c_int
                    }
                    _ => { }
                }
            }
        }
        return rc
    } else {
        if st_odd.st_mode & 0o400 as libc::c_int as libc::c_uint != 0 {
            *ret = odd;
            return GDBM_SNAPSHOT_OK as libc::c_int
        }
    }
    return GDBM_SNAPSHOT_BAD as libc::c_int;
}
/* proto.h - The prototypes for the dbm routines. */
/* This file is part of GDBM, the GNU data base manager.
   Copyright (C) 1990-2022 Free Software Foundation, Inc.

   GDBM is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GDBM is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GDBM. If not, see <http://www.gnu.org/licenses/>.   */
/* From bucket.c */
/* Mark current bucket as changed. */
/* Return true if the directory entry at DIR_INDEX can be considered
   valid. This means that DIR_INDEX is in the valid range for addressing
   the dir array, and the offset stored in dir[DIR_INDEX] points past
   first two blocks in file. This does not necessarily mean that there's
   a valid bucket or data block at that offset. All this implies is that
   it is safe to use the offset for look up in the bucket cache and to
   attempt to read a block at that offset. */
/* From falloc.c */
/* From findkey.c */
/* From hash.c */
/* From update.c */
/* From gdbmopen.c */
/* From gdbmload.c */
/* From mmap.c */
/* From lock.c */
/* From fullio.c */
/* From base64.c */
/* From recover.c */
/* avail.c */
/* I/O functions */
/* From gdbmsync.c */
/* GDBM_FAILURE_ATOMIC */
#[no_mangle]
pub unsafe extern "C" fn gdbm_file_sync(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int; /* return value */
    r = _gdbm_mapped_sync(dbf);
    /* If and only if the conventional fsync/msync/sync succeeds,
     attempt to clone the data file. */
    if r == 0 as libc::c_int { r = _gdbm_snapshot(dbf) }
    /* GDBM_FAILURE_ATOMIC */
    return r;
}

#[no_mangle]
pub unsafe extern "C" fn gdbm_sync(mut dbf: GDBM_FILE) -> libc::c_int {
    /* Return immediately if the database needs recovery */
    if (*dbf).need_recovery() != 0 {
        gdbm_set_errno(dbf, GDBM_NEED_RECOVERY as libc::c_int,
                       1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* Initialize the gdbm_errno variable. */
    gdbm_set_errno(dbf, GDBM_NO_ERROR as libc::c_int, 0 as libc::c_int);
    if !(*dbf).xheader.is_null() {
        (*(*dbf).xheader).numsync = (*(*dbf).xheader).numsync.wrapping_add(1);
        (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint)
    }
    _gdbm_end_update(dbf);
    /* Do the sync on the file. */
    return gdbm_file_sync(dbf);
}


//bucket.rs
#[inline]
unsafe extern "C" fn gdbm_dir_entry_valid_p(
    mut dbf: GDBM_FILE,
    mut dir_index: libc::c_int,
) -> libc::c_int {
    return (dir_index >= 0 as libc::c_int
        && (dir_index as libc::c_ulong)
            < ((*(*dbf).header).dir_size as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
        && *((*dbf).dir).offset(dir_index as isize)
            >= (*(*dbf).header).block_size as libc::c_long) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_new_bucket(
    mut dbf: GDBM_FILE,
    mut bucket: *mut hash_bucket,
    mut bits: libc::c_int,
) {
    let mut index: libc::c_int = 0;
    (*bucket).av_count = 0 as libc::c_int;
    (*bucket).bucket_bits = bits;
    (*bucket).count = 0 as libc::c_int;
    index = 0 as libc::c_int;
    while index < (*(*dbf).header).bucket_elems {
        (*((*bucket).h_table).as_mut_ptr().offset(index as isize))
            .hash_value = -(1 as libc::c_int);
        index += 1;
    }
}
unsafe extern "C" fn adrhash(mut adr: off_t, mut nbits: size_t) -> size_t {
    adr
        ^= adr
            >> ((31 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_sub(nbits);
    return ((265443576910 as libc::c_ulong).wrapping_mul(adr as libc::c_ulong)
        & 0xffffffff as libc::c_uint as libc::c_ulong)
        >> ((31 as libc::c_int + 1 as libc::c_int) as libc::c_ulong).wrapping_sub(nbits);
}
unsafe extern "C" fn cache_tab_lookup_slot(
    mut dbf: GDBM_FILE,
    mut adr: off_t,
) -> *mut *mut cache_elem {
    let mut cache: *mut *mut cache_elem = (*dbf).cache;
    let mut h: size_t = adrhash(adr, (*dbf).cache_bits as size_t);
    if !(*cache.offset(h as isize)).is_null() {
        if (**cache.offset(h as isize)).ca_adr != adr {
            let mut prev: *mut cache_elem = *cache.offset(h as isize);
            let mut p: *mut cache_elem = (*prev).ca_coll;
            while !p.is_null() {
                if (*p).ca_adr == adr {
                    break;
                }
                prev = p;
                p = (*prev).ca_coll;
            }
            return &mut (*prev).ca_coll;
        }
    }
    return &mut *cache.offset(h as isize) as *mut *mut cache_elem;
}
unsafe extern "C" fn lru_link_elem(
    mut dbf: GDBM_FILE,
    mut elem: *mut cache_elem,
    mut ref_0: *mut cache_elem,
) {
    if ref_0.is_null() {
        let ref mut fresh0 = (*elem).ca_prev;
        *fresh0 = 0 as *mut cache_elem;
        let ref mut fresh1 = (*elem).ca_next;
        *fresh1 = (*dbf).cache_mru;
        if !((*dbf).cache_mru).is_null() {
            let ref mut fresh2 = (*(*dbf).cache_mru).ca_prev;
            *fresh2 = elem;
        } else {
            let ref mut fresh3 = (*dbf).cache_lru;
            *fresh3 = elem;
        }
        let ref mut fresh4 = (*dbf).cache_mru;
        *fresh4 = elem;
        let ref mut fresh5 = (*dbf).bucket;
        *fresh5 = ((*(*dbf).cache_mru).ca_bucket).as_mut_ptr();
    } else {
        let mut x: *mut cache_elem = 0 as *mut cache_elem;
        let ref mut fresh6 = (*elem).ca_prev;
        *fresh6 = ref_0;
        let ref mut fresh7 = (*elem).ca_next;
        *fresh7 = (*ref_0).ca_next;
        x = (*ref_0).ca_next;
        if !x.is_null() {
            let ref mut fresh8 = (*x).ca_prev;
            *fresh8 = elem;
        } else {
            let ref mut fresh9 = (*dbf).cache_lru;
            *fresh9 = elem;
        }
        let ref mut fresh10 = (*ref_0).ca_next;
        *fresh10 = elem;
    };
}
unsafe extern "C" fn lru_unlink_elem(mut dbf: GDBM_FILE, mut elem: *mut cache_elem) {
    let mut x: *mut cache_elem = 0 as *mut cache_elem;
    x = (*elem).ca_prev;
    if !x.is_null() {
        let ref mut fresh11 = (*x).ca_next;
        *fresh11 = (*elem).ca_next;
    } else {
        let ref mut fresh12 = (*dbf).cache_mru;
        *fresh12 = (*elem).ca_next;
        let ref mut fresh13 = (*dbf).bucket;
        *fresh13 = if !((*dbf).cache_mru).is_null() {
            ((*(*dbf).cache_mru).ca_bucket).as_mut_ptr()
        } else {
            0 as *mut hash_bucket
        };
    }
    x = (*elem).ca_next;
    if !x.is_null() {
        let ref mut fresh14 = (*x).ca_prev;
        *fresh14 = (*elem).ca_prev;
    } else {
        let ref mut fresh15 = (*dbf).cache_lru;
        *fresh15 = (*elem).ca_prev;
    }
    let ref mut fresh16 = (*elem).ca_next;
    *fresh16 = 0 as *mut cache_elem;
    let ref mut fresh17 = (*elem).ca_prev;
    *fresh17 = *fresh16;
}
unsafe extern "C" fn cache_elem_new(
    mut dbf: GDBM_FILE,
    mut adr: off_t,
) -> *mut cache_elem {
    let mut elem: *mut cache_elem = 0 as *mut cache_elem;
    elem = (*dbf).cache_avail;
    if !elem.is_null() {
        let ref mut fresh18 = (*dbf).cache_avail;
        *fresh18 = (*elem).ca_next;
    } else {
        elem = calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<cache_elem>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<hash_bucket>() as libc::c_ulong)
                .wrapping_add((*(*dbf).header).bucket_size as libc::c_ulong),
        ) as *mut cache_elem;
        if elem.is_null() {
            return 0 as *mut cache_elem;
        }
    }
    (*elem).ca_adr = adr;
    (*elem).ca_changed = 0 as libc::c_int as libc::c_char;
    (*elem).ca_data.hash_val = -(1 as libc::c_int);
    (*elem).ca_data.elem_loc = -(1 as libc::c_int);
    let ref mut fresh19 = (*elem).ca_coll;
    *fresh19 = 0 as *mut cache_elem;
    let ref mut fresh20 = (*elem).ca_next;
    *fresh20 = *fresh19;
    let ref mut fresh21 = (*elem).ca_prev;
    *fresh21 = *fresh20;
    (*elem).ca_hits = 0 as libc::c_int as size_t;
    return elem;
}
unsafe extern "C" fn cache_elem_free(mut dbf: GDBM_FILE, mut elem: *mut cache_elem) {
    let mut h: size_t = adrhash((*elem).ca_adr, (*dbf).cache_bits as size_t);
    let mut pp: *mut *mut cache_elem = 0 as *mut *mut cache_elem;
    lru_unlink_elem(dbf, elem);
    let ref mut fresh22 = (*elem).ca_next;
    *fresh22 = (*dbf).cache_avail;
    let ref mut fresh23 = (*dbf).cache_avail;
    *fresh23 = elem;
    let ref mut fresh24 = (*dbf).cache_num;
    *fresh24 = (*fresh24).wrapping_sub(1);
    pp = &mut *((*dbf).cache).offset(h as isize) as *mut *mut cache_elem;
    while !(*pp).is_null() {
        if *pp == elem {
            *pp = (**pp).ca_coll;
            break;
        } else {
            pp = &mut (**pp).ca_coll;
        }
    }
}
#[inline]
unsafe extern "C" fn cache_lru_free(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut last: *mut cache_elem = (*dbf).cache_lru;
    if (*last).ca_changed != 0 {
        if _gdbm_write_bucket(dbf, last) != 0 {
            return -(1 as libc::c_int);
        }
    }
    cache_elem_free(dbf, last);
    return 0 as libc::c_int;
}
unsafe extern "C" fn log2i(mut v: libc::c_uint) -> libc::c_uint {
    static mut dbp: [libc::c_int; 32] = [
        0 as libc::c_int,
        1 as libc::c_int,
        28 as libc::c_int,
        2 as libc::c_int,
        29 as libc::c_int,
        14 as libc::c_int,
        24 as libc::c_int,
        3 as libc::c_int,
        30 as libc::c_int,
        22 as libc::c_int,
        20 as libc::c_int,
        15 as libc::c_int,
        25 as libc::c_int,
        17 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
        31 as libc::c_int,
        27 as libc::c_int,
        13 as libc::c_int,
        23 as libc::c_int,
        21 as libc::c_int,
        19 as libc::c_int,
        16 as libc::c_int,
        7 as libc::c_int,
        26 as libc::c_int,
        12 as libc::c_int,
        18 as libc::c_int,
        6 as libc::c_int,
        11 as libc::c_int,
        5 as libc::c_int,
        10 as libc::c_int,
        9 as libc::c_int,
    ];
    v = v.wrapping_sub(1);
    v |= v >> 1 as libc::c_int;
    v |= v >> 2 as libc::c_int;
    v |= v >> 4 as libc::c_int;
    v |= v >> 8 as libc::c_int;
    v |= v >> 16 as libc::c_int;
    v = v.wrapping_add(1);
    return dbp[(v.wrapping_mul(0x77cb531 as libc::c_uint) >> 27 as libc::c_int) as usize]
        as libc::c_uint;
}
unsafe extern "C" fn cache_tab_resize(
    mut dbf: GDBM_FILE,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut size: size_t = ((1 as libc::c_int) << bits) as size_t;
    if ((*dbf).cache).is_null() || size != (*dbf).cache_size {
        let mut n: size_t = size
            .wrapping_mul(::std::mem::size_of::<*mut cache_elem>() as libc::c_ulong);
        let mut p: *mut *mut cache_elem = 0 as *mut *mut cache_elem;
        let mut elem: *mut cache_elem = 0 as *mut cache_elem;
        if _gdbm_cache_flush(dbf) != 0 {
            return -(1 as libc::c_int);
        }
        p = realloc((*dbf).cache as *mut libc::c_void, n) as *mut *mut cache_elem;
        if p.is_null() {
            gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        let ref mut fresh25 = (*dbf).cache;
        *fresh25 = p;
        (*dbf).cache_size = size;
        (*dbf).cache_bits = bits;
        memset((*dbf).cache as *mut libc::c_void, 0 as libc::c_int, n);
        elem = (*dbf).cache_lru;
        while !elem.is_null() {
            let mut prev: *mut cache_elem = (*elem).ca_prev;
            let ref mut fresh26 = (*elem).ca_coll;
            *fresh26 = 0 as *mut cache_elem;
            if size < (*dbf).cache_num {
                cache_elem_free(dbf, elem);
            } else {
                p = cache_tab_lookup_slot(dbf, (*elem).ca_adr);
                if !(*p).is_null() {
                    abort();
                }
                *p = elem;
            }
            elem = prev;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cache_lookup(
    mut dbf: GDBM_FILE,
    mut adr: off_t,
    mut ref_0: *mut cache_elem,
    mut ret_elem: *mut *mut cache_elem,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut elp: *mut *mut cache_elem = 0 as *mut *mut cache_elem;
    let mut elem: *mut cache_elem = 0 as *mut cache_elem;
    let ref mut fresh27 = (*dbf).cache_access_count;
    *fresh27 = (*fresh27).wrapping_add(1);
    elp = cache_tab_lookup_slot(dbf, adr);
    if !(*elp).is_null() {
        elem = *elp;
        let ref mut fresh28 = (*elem).ca_hits;
        *fresh28 = (*fresh28).wrapping_add(1);
        let ref mut fresh29 = (*dbf).cache_hits;
        *fresh29 = (*fresh29).wrapping_add(1);
        lru_unlink_elem(dbf, elem);
        rc = cache_found as libc::c_int;
    } else {
        elem = cache_elem_new(dbf, adr);
        if elem.is_null() {
            return cache_failure as libc::c_int
        } else {
            rc = cache_new as libc::c_int;
            if (*dbf).cache_num == (*dbf).cache_size {
                if (*dbf).cache_auto() as libc::c_int != 0
                    && (*dbf).cache_bits < (*(*dbf).header).dir_bits
                    && cache_tab_resize(dbf, (*dbf).cache_bits + 1 as libc::c_int)
                        == 0 as libc::c_int
                {
                    elp = cache_tab_lookup_slot(dbf, adr);
                } else if cache_lru_free(dbf) != 0 {
                    rc = cache_failure as libc::c_int;
                }
            }
            if rc == cache_new as libc::c_int {
                *elp = elem;
                let ref mut fresh30 = (*dbf).cache_num;
                *fresh30 = (*fresh30).wrapping_add(1);
            }
        }
    }
    if ref_0.is_null() && (*elem).ca_changed == 0 {
        _gdbm_cache_flush(dbf);
    }
    lru_link_elem(dbf, elem, ref_0);
    if rc != cache_failure as libc::c_int {
        *ret_elem = elem;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_get_bucket(
    mut dbf: GDBM_FILE,
    mut dir_index: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut bucket_adr: off_t = 0;
    let mut file_pos: off_t = 0;
    let mut bucket: *mut hash_bucket = 0 as *mut hash_bucket;
    let mut elem: *mut cache_elem = 0 as *mut cache_elem;
    if gdbm_dir_entry_valid_p(dbf, dir_index) == 0 {
        gdbm_set_errno(dbf, GDBM_BAD_DIR_ENTRY as libc::c_int, 1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    (*dbf).bucket_dir = dir_index;
    bucket_adr = *((*dbf).dir).offset(dir_index as isize);
    match cache_lookup(dbf, bucket_adr, 0 as *mut cache_elem, &mut elem) {
        1 => {
            file_pos = gdbm_file_seek(dbf, bucket_adr, 0 as libc::c_int);
            if file_pos != bucket_adr {
                gdbm_set_errno(
                    dbf,
                    GDBM_FILE_SEEK_ERROR as libc::c_int,
                    1 as libc::c_int,
                );
                cache_elem_free(dbf, elem);
                _gdbm_fatal(
                    dbf,
                    dcgettext(
                        b"gdbm\0" as *const u8 as *const libc::c_char,
                        b"lseek error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return -(1 as libc::c_int);
            }
            rc = _gdbm_full_read(
                dbf,
                ((*elem).ca_bucket).as_mut_ptr() as *mut libc::c_void,
                (*(*dbf).header).bucket_size as size_t,
            );
            if rc != 0 {
                (*dbf).set_need_recovery(1 as libc::c_int as libc::c_uint);
                cache_elem_free(dbf, elem);
                _gdbm_fatal(dbf, gdbm_db_strerror(dbf));
                return -(1 as libc::c_int);
            }
            bucket = ((*elem).ca_bucket).as_mut_ptr();
            if !((*bucket).count >= 0 as libc::c_int
                && (*bucket).count <= (*(*dbf).header).bucket_elems
                && (*bucket).bucket_bits >= 0 as libc::c_int
                && (*bucket).bucket_bits <= (*(*dbf).header).dir_bits)
            {
                gdbm_set_errno(dbf, GDBM_BAD_BUCKET as libc::c_int, 1 as libc::c_int);
                cache_elem_free(dbf, elem);
                return -(1 as libc::c_int);
            }
            if gdbm_bucket_avail_table_validate(dbf, bucket) != 0 {
                cache_elem_free(dbf, elem);
                return -(1 as libc::c_int);
            }
            (*elem).ca_adr = bucket_adr;
            (*elem).ca_data.elem_loc = -(1 as libc::c_int);
            (*elem).ca_changed = 0 as libc::c_int as libc::c_char;
        }
        2 => return -(1 as libc::c_int),
        0 | _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_split_bucket(
    mut dbf: GDBM_FILE,
    mut next_insert: libc::c_int,
) -> libc::c_int {
    let mut old_adr: [off_t; 31] = [0; 31];
    let mut old_size: [libc::c_int; 31] = [0; 31];
    let mut old_count: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut index1: libc::c_int = 0;
    old_count = 0 as libc::c_int;
    while (*(*dbf).bucket).count == (*(*dbf).header).bucket_elems {
        let mut new_bits: libc::c_int = 0;
        let mut newcache: [*mut cache_elem; 2] = [0 as *mut cache_elem; 2];
        let mut adr_0: off_t = 0;
        let mut adr_1: off_t = 0;
        let mut old_bucket: avail_elem = avail_elem {
            av_size: 0,
            av_adr: 0,
        };
        let mut dir_start0: off_t = 0;
        let mut dir_start1: off_t = 0;
        let mut dir_end: off_t = 0;
        new_bits = (*(*dbf).bucket).bucket_bits + 1 as libc::c_int;
        adr_0 = _gdbm_alloc(dbf, (*(*dbf).header).bucket_size);
        match cache_lookup(
            dbf,
            adr_0,
            (*dbf).cache_mru,
            &mut *newcache.as_mut_ptr().offset(0 as libc::c_int as isize),
        ) {
            0 => {
                gdbm_set_errno(
                    dbf,
                    GDBM_BUCKET_CACHE_CORRUPTED as libc::c_int,
                    1 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            2 => return -(1 as libc::c_int),
            1 | _ => {}
        }
        _gdbm_new_bucket(
            dbf,
            ((*newcache[0 as libc::c_int as usize]).ca_bucket).as_mut_ptr(),
            new_bits,
        );
        adr_1 = _gdbm_alloc(dbf, (*(*dbf).header).bucket_size);
        match cache_lookup(
            dbf,
            adr_1,
            newcache[0 as libc::c_int as usize],
            &mut *newcache.as_mut_ptr().offset(1 as libc::c_int as isize),
        ) {
            0 => {
                gdbm_set_errno(
                    dbf,
                    GDBM_BUCKET_CACHE_CORRUPTED as libc::c_int,
                    1 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            2 => return -(1 as libc::c_int),
            1 | _ => {}
        }
        _gdbm_new_bucket(
            dbf,
            ((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr(),
            new_bits,
        );
        if (*(*dbf).header).dir_bits == (*(*dbf).bucket).bucket_bits {
            let mut new_dir: *mut off_t = 0 as *mut off_t;
            let mut dir_size: libc::c_int = 0;
            let mut dir_adr: off_t = 0;
            if (*(*dbf).header).dir_size >= 2147483647 as libc::c_int / 2 as libc::c_int
            {
                gdbm_set_errno(dbf, GDBM_DIR_OVERFLOW as libc::c_int, 1 as libc::c_int);
                _gdbm_fatal(
                    dbf,
                    dcgettext(
                        b"gdbm\0" as *const u8 as *const libc::c_char,
                        b"directory overflow\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return -(1 as libc::c_int);
            }
            dir_size = (*(*dbf).header).dir_size * 2 as libc::c_int;
            dir_adr = _gdbm_alloc(dbf, dir_size);
            if dir_adr == 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int);
            }
            new_dir = malloc(dir_size as libc::c_ulong) as *mut off_t;
            if new_dir.is_null() {
                gdbm_set_errno(dbf, GDBM_MALLOC_ERROR as libc::c_int, 1 as libc::c_int);
                _gdbm_fatal(
                    dbf,
                    dcgettext(
                        b"gdbm\0" as *const u8 as *const libc::c_char,
                        b"malloc error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return -(1 as libc::c_int);
            }
            index = 0 as libc::c_int;
            while (index as libc::c_ulong)
                < ((*(*dbf).header).dir_size as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<off_t>() as libc::c_ulong)
            {
                *new_dir
                    .offset(
                        (2 as libc::c_int * index) as isize,
                    ) = *((*dbf).dir).offset(index as isize);
                *new_dir
                    .offset(
                        (2 as libc::c_int * index + 1 as libc::c_int) as isize,
                    ) = *((*dbf).dir).offset(index as isize);
                index += 1;
            }
            old_adr[old_count as usize] = (*(*dbf).header).dir;
            (*(*dbf).header).dir = dir_adr;
            old_size[old_count as usize] = (*(*dbf).header).dir_size;
            (*(*dbf).header).dir_size = dir_size;
            (*(*dbf).header).dir_bits = new_bits;
            old_count += 1;
            (*dbf).set_header_changed(1 as libc::c_int as libc::c_uint);
            (*dbf).bucket_dir *= 2 as libc::c_int;
            free((*dbf).dir as *mut libc::c_void);
            let ref mut fresh31 = (*dbf).dir;
            *fresh31 = new_dir;
        }
        index = 0 as libc::c_int;
        while index < (*(*dbf).header).bucket_elems {
            let mut old_el: *mut bucket_element = &mut *((*(*dbf).bucket).h_table)
                .as_mut_ptr()
                .offset(index as isize) as *mut bucket_element;
            let mut bucket: *mut hash_bucket = 0 as *mut hash_bucket;
            let mut elem_loc: libc::c_int = 0;
            if (*old_el).hash_value < 0 as libc::c_int {
                gdbm_set_errno(dbf, GDBM_BAD_BUCKET as libc::c_int, 1 as libc::c_int);
                return -(1 as libc::c_int);
            }
            bucket = ((*newcache[((*old_el).hash_value >> 31 as libc::c_int - new_bits
                & 1 as libc::c_int) as usize])
                .ca_bucket)
                .as_mut_ptr();
            elem_loc = (*old_el).hash_value % (*(*dbf).header).bucket_elems;
            while (*((*bucket).h_table).as_mut_ptr().offset(elem_loc as isize))
                .hash_value != -(1 as libc::c_int)
            {
                elem_loc = (elem_loc + 1 as libc::c_int) % (*(*dbf).header).bucket_elems;
            }
            *((*bucket).h_table).as_mut_ptr().offset(elem_loc as isize) = *old_el;
            let ref mut fresh32 = (*bucket).count;
            *fresh32 += 1;
            index += 1;
        }
        (*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
            .bucket_avail[0 as libc::c_int as usize]
            .av_adr = _gdbm_alloc(dbf, (*(*dbf).header).block_size);
        if (*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
            .bucket_avail[0 as libc::c_int as usize]
            .av_adr == 0 as libc::c_int as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        (*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
            .bucket_avail[0 as libc::c_int as usize]
            .av_size = (*(*dbf).header).block_size;
        (*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
            .av_count = 1 as libc::c_int;
        (*((*newcache[0 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
            .av_count = (*(*dbf).bucket).av_count;
        index = 0 as libc::c_int;
        if (*((*newcache[0 as libc::c_int as usize]).ca_bucket).as_mut_ptr()).av_count
            == 6 as libc::c_int
        {
            _gdbm_put_av_elem(
                (*(*dbf).bucket).bucket_avail[0 as libc::c_int as usize],
                ((*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
                    .bucket_avail)
                    .as_mut_ptr(),
                &mut (*((**newcache.as_mut_ptr().offset(1 as libc::c_int as isize))
                    .ca_bucket)
                    .as_mut_ptr())
                    .av_count,
                (*dbf).coalesce_blocks() as libc::c_int,
            );
            index = 1 as libc::c_int;
            let ref mut fresh33 = (*((*newcache[0 as libc::c_int as usize]).ca_bucket)
                .as_mut_ptr())
                .av_count;
            *fresh33 -= 1;
        }
        index1 = 0 as libc::c_int;
        while index < (*(*dbf).bucket).av_count {
            let fresh34 = index1;
            index1 = index1 + 1;
            (*((*newcache[0 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
                .bucket_avail[fresh34
                as usize] = (*(*dbf).bucket).bucket_avail[index as usize];
            index += 1;
        }
        dir_start1 = ((*dbf).bucket_dir >> (*(*dbf).header).dir_bits - new_bits
            | 1 as libc::c_int) as off_t;
        dir_end = (dir_start1 + 1 as libc::c_int as libc::c_long)
            << (*(*dbf).header).dir_bits - new_bits;
        dir_start1 = dir_start1 << (*(*dbf).header).dir_bits - new_bits;
        dir_start0 = dir_start1 - (dir_end - dir_start1);
        index = dir_start0 as libc::c_int;
        while (index as libc::c_long) < dir_start1 {
            *((*dbf).dir).offset(index as isize) = adr_0;
            index += 1;
        }
        index = dir_start1 as libc::c_int;
        while (index as libc::c_long) < dir_end {
            *((*dbf).dir).offset(index as isize) = adr_1;
            index += 1;
        }
        (*newcache[0 as libc::c_int as usize])
            .ca_changed = 1 as libc::c_int as libc::c_char;
        (*newcache[1 as libc::c_int as usize])
            .ca_changed = 1 as libc::c_int as libc::c_char;
        (*dbf).set_directory_changed(1 as libc::c_int as libc::c_uint);
        (*dbf).bucket_dir = _gdbm_bucket_dir(dbf, next_insert);
        old_bucket.av_adr = (*(*dbf).cache_mru).ca_adr;
        old_bucket.av_size = (*(*dbf).header).bucket_size;
        cache_elem_free(dbf, (*dbf).cache_mru);
        if *((*dbf).dir).offset((*dbf).bucket_dir as isize) != adr_0 {
            let mut t: *mut cache_elem = newcache[0 as libc::c_int as usize];
            newcache[0 as libc::c_int as usize] = newcache[1 as libc::c_int as usize];
            newcache[1 as libc::c_int as usize] = t;
        }
        _gdbm_put_av_elem(
            old_bucket,
            ((*((*newcache[1 as libc::c_int as usize]).ca_bucket).as_mut_ptr())
                .bucket_avail)
                .as_mut_ptr(),
            &mut (*((**newcache.as_mut_ptr().offset(1 as libc::c_int as isize))
                .ca_bucket)
                .as_mut_ptr())
                .av_count,
            (*dbf).coalesce_blocks() as libc::c_int,
        );
        lru_unlink_elem(dbf, newcache[0 as libc::c_int as usize]);
        lru_link_elem(dbf, newcache[0 as libc::c_int as usize], 0 as *mut cache_elem);
    }
    index = 0 as libc::c_int;
    while index < old_count {
        if _gdbm_free(dbf, old_adr[index as usize], old_size[index as usize]) != 0 {
            return -(1 as libc::c_int);
        }
        index += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_write_bucket(
    mut dbf: GDBM_FILE,
    mut ca_entry: *mut cache_elem,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut file_pos: off_t = 0;
    file_pos = gdbm_file_seek(dbf, (*ca_entry).ca_adr, 0 as libc::c_int);
    if file_pos != (*ca_entry).ca_adr {
        gdbm_set_errno(dbf, GDBM_FILE_SEEK_ERROR as libc::c_int, 1 as libc::c_int);
        _gdbm_fatal(
            dbf,
            dcgettext(
                b"gdbm\0" as *const u8 as *const libc::c_char,
                b"lseek error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    rc = _gdbm_full_write(
        dbf,
        ((*ca_entry).ca_bucket).as_mut_ptr() as *mut libc::c_void,
        (*(*dbf).header).bucket_size as size_t,
    );
    if rc != 0 {
        _gdbm_fatal(dbf, gdbm_strerror(rc));
        return -(1 as libc::c_int);
    }
    (*ca_entry).ca_changed = 0 as libc::c_int as libc::c_char;
    (*ca_entry).ca_data.hash_val = -(1 as libc::c_int);
    (*ca_entry).ca_data.elem_loc = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_cache_init(
    mut dbf: GDBM_FILE,
    mut size: size_t,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut cache_auto: libc::c_int = 0;
    if size == 0 as libc::c_int as libc::c_ulong {
        cache_auto = 1 as libc::c_int;
        bits = if !((*dbf).cache).is_null() {
            (*dbf).cache_bits
        } else {
            9 as libc::c_int
        };
    } else if size
            > (-(1 as libc::c_int) as size_t)
                .wrapping_div(::std::mem::size_of::<*mut cache_elem>() as libc::c_ulong)
        {
        gdbm_set_errno(dbf, GDBM_OPT_BADVAL as libc::c_int, 0 as libc::c_int);
        return -(1 as libc::c_int);
    } else {
        cache_auto = 0 as libc::c_int;
        bits = log2i(
            (if size < 4 as libc::c_int as libc::c_ulong {
                4 as libc::c_int as libc::c_ulong
            } else {
                size
            }) as libc::c_uint,
        ) as libc::c_int;
    }
    (*dbf).set_cache_auto(cache_auto as libc::c_uint);
    return cache_tab_resize(dbf, bits);
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_cache_free(mut dbf: GDBM_FILE) {
    let mut elem: *mut cache_elem = 0 as *mut cache_elem;
    while !((*dbf).cache_lru).is_null() {
        cache_elem_free(dbf, (*dbf).cache_lru);
    }
    free((*dbf).cache as *mut libc::c_void);
    let ref mut fresh35 = (*dbf).cache;
    *fresh35 = 0 as *mut *mut cache_elem;
    loop {
        elem = (*dbf).cache_avail;
        if elem.is_null() {
            break;
        }
        let ref mut fresh36 = (*dbf).cache_avail;
        *fresh36 = (*elem).ca_next;
        free((*elem).ca_data.dptr as *mut libc::c_void);
        free(elem as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _gdbm_cache_flush(mut dbf: GDBM_FILE) -> libc::c_int {
    let mut elem: *mut cache_elem = 0 as *mut cache_elem;
    elem = (*dbf).cache_mru;
    while !elem.is_null() && (*elem).ca_changed as libc::c_int != 0 {
        if _gdbm_write_bucket(dbf, elem) != 0 {
            return -(1 as libc::c_int);
        }
        elem = (*elem).ca_next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_get_cache_stats(
    mut dbf: GDBM_FILE,
    mut access_count: *mut size_t,
    mut cache_hits: *mut size_t,
    mut cache_count: *mut size_t,
    mut bstat: *mut gdbm_cache_stat,
    mut nstat: size_t,
) {
    if !access_count.is_null() {
        *access_count = (*dbf).cache_access_count;
    }
    if !cache_hits.is_null() {
        *cache_hits = (*dbf).cache_hits;
    }
    if !cache_count.is_null() {
        *cache_count = (*dbf).cache_num;
    }
    if !bstat.is_null() {
        let mut i: size_t = 0;
        let mut elem: *mut cache_elem = 0 as *mut cache_elem;
        if nstat > (*dbf).cache_num {
            nstat = (*dbf).cache_num;
        }
        i = 0 as libc::c_int as size_t;
        elem = (*dbf).cache_mru;
        while i < nstat {
            (*bstat.offset(i as isize)).adr = (*elem).ca_adr;
            (*bstat.offset(i as isize)).hits = (*elem).ca_hits;
            i = i.wrapping_add(1);
            elem = (*elem).ca_next;
        }
    }
}