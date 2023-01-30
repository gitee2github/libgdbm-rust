#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gdbm_errno_location() -> *mut libc::c_int;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn _gdbm_file_size(dbf: GDBM_FILE, psize: *mut off_t) -> libc::c_int;
    fn _gdbm_file_extend(dbf: GDBM_FILE, size: off_t) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn msync(
        __addr: *mut libc::c_void,
        __len: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
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
    pub lock_type: C2RustUnnamed_0,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOCKING_FCNTL: C2RustUnnamed_0 = 3;
pub const LOCKING_LOCKF: C2RustUnnamed_0 = 2;
pub const LOCKING_FLOCK: C2RustUnnamed_0 = 1;
pub const LOCKING_NONE: C2RustUnnamed_0 = 0;
pub type gdbm_error = libc::c_int;
pub type GDBM_FILE = *mut gdbm_file_info;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GDBM_ERR_USAGE: C2RustUnnamed_1 = 44;
pub const GDBM_ERR_REALPATH: C2RustUnnamed_1 = 43;
pub const GDBM_ERR_SNAPSHOT_CLONE: C2RustUnnamed_1 = 42;
pub const GDBM_BAD_HASH_ENTRY: C2RustUnnamed_1 = 41;
pub const GDBM_BUCKET_CACHE_CORRUPTED: C2RustUnnamed_1 = 40;
pub const GDBM_FILE_TRUNCATE_ERROR: C2RustUnnamed_1 = 39;
pub const GDBM_FILE_SYNC_ERROR: C2RustUnnamed_1 = 38;
pub const GDBM_FILE_CLOSE_ERROR: C2RustUnnamed_1 = 37;
pub const GDBM_BAD_DIR_ENTRY: C2RustUnnamed_1 = 36;
pub const GDBM_BAD_HASH_TABLE: C2RustUnnamed_1 = 35;
pub const GDBM_BAD_AVAIL: C2RustUnnamed_1 = 34;
pub const GDBM_BAD_HEADER: C2RustUnnamed_1 = 33;
pub const GDBM_BAD_BUCKET: C2RustUnnamed_1 = 32;
pub const GDBM_DIR_OVERFLOW: C2RustUnnamed_1 = 31;
pub const GDBM_BACKUP_FAILED: C2RustUnnamed_1 = 30;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_1 = 29;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed_1 = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed_1 = 27;
pub const GDBM_NO_DBNAME: C2RustUnnamed_1 = 26;
pub const GDBM_FILE_EOF: C2RustUnnamed_1 = 25;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed_1 = 24;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed_1 = 23;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed_1 = 22;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed_1 = 21;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed_1 = 20;
pub const GDBM_OPT_BADVAL: C2RustUnnamed_1 = 20;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed_1 = 19;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed_1 = 18;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed_1 = 18;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed_1 = 17;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed_1 = 16;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_1 = 15;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed_1 = 14;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed_1 = 13;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed_1 = 12;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed_1 = 11;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed_1 = 10;
pub const GDBM_CANT_BE_READER: C2RustUnnamed_1 = 9;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed_1 = 8;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed_1 = 7;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed_1 = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed_1 = 5;
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_1 = 4;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_1 = 3;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_1 = 2;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_1 = 1;
pub const GDBM_NO_ERROR: C2RustUnnamed_1 = 0;
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
