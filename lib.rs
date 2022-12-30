#![crate_type = "staticlib"]

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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;

    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;

    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;

    fn gdbm_avail_verify(dbf: GDBM_FILE) -> libc::c_int;


    fn _gdbm_mapped_init(_: GDBM_FILE) -> libc::c_int;
    fn _gdbm_cache_init(_: GDBM_FILE, _: size_t) -> libc::c_int;
    fn _gdbm_full_read(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
    
    fn _gdbm_mapped_lseek(_: GDBM_FILE, _: off_t, _: libc::c_int) -> off_t;
    fn gdbm_avail_block_validate(
        dbf: GDBM_FILE,
        avblk: *mut avail_block,
        size: size_t,
    ) -> libc::c_int;

    
    fn _gdbm_file_extend(dbf: GDBM_FILE, size: off_t) -> libc::c_int;
    fn _gdbm_full_write(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
    fn _gdbm_new_bucket(_: GDBM_FILE, _: *mut hash_bucket, _: libc::c_int);

    fn _gdbm_lock_file(_: GDBM_FILE) -> libc::c_int;
    
    fn _gdbm_cache_free(dbf: GDBM_FILE);
    fn _gdbm_mapped_sync(_: GDBM_FILE) -> libc::c_int;
    fn _gdbm_unlock_file(_: GDBM_FILE);
    fn _gdbm_mapped_unmap(_: GDBM_FILE);
    fn _gdbm_end_update(_: GDBM_FILE) -> libc::c_int;
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;

    fn _gdbm_free(_: GDBM_FILE, _: off_t, _: libc::c_int) -> libc::c_int;
    fn _gdbm_get_bucket(_: GDBM_FILE, _: libc::c_int) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
//#[repr(C, align(1))]
//#[derive(BitfieldStruct)]
//#[derive(Copy, Clone, BitfieldStruct)]
//#[repr(C)]

#[macro_use]
extern crate c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type blksize_t = __blksize_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const LOCKING_FCNTL: C2RustUnnamed = 3;
pub const LOCKING_LOCKF: C2RustUnnamed = 2;
pub const LOCKING_FLOCK: C2RustUnnamed = 1;
pub const LOCKING_NONE: C2RustUnnamed = 0;
pub type gdbm_error = libc::c_int;
pub type GDBM_FILE = *mut gdbm_file_info;


pub const GDBM_BAD_HEADER: C2RustUnnamed_0 = 33;
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





/* GDBM_FAILURE_ATOMIC */
pub const GDBM_BACKUP_FAILED: C2RustUnnamed_0 = 30;
pub const GDBM_BAD_AVAIL: C2RustUnnamed_0 = 34;
pub const GDBM_BAD_BUCKET: C2RustUnnamed_0 = 32;
pub const GDBM_BAD_DIR_ENTRY: C2RustUnnamed_0 = 36;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed_0 = 22;
pub const GDBM_BAD_HASH_ENTRY: C2RustUnnamed_0 = 41;
pub const GDBM_BAD_HASH_TABLE: C2RustUnnamed_0 = 35;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed_0 = 7;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed_0 = 23;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_0 = 2;
pub const GDBM_BUCKET_CACHE_CORRUPTED: C2RustUnnamed_0 = 40;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed_0 = 21;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed_0 = 17;
pub const GDBM_CANT_BE_READER: C2RustUnnamed_0 = 9;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed_0 = 10;
pub const GDBM_DIR_OVERFLOW: C2RustUnnamed_0 = 31;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed_0 = 8;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed_0 = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed_0 = 27;
pub const GDBM_ERR_REALPATH: C2RustUnnamed_0 = 43;
pub const GDBM_ERR_SNAPSHOT_CLONE: C2RustUnnamed_0 = 42;
pub const GDBM_ERR_USAGE: C2RustUnnamed_0 = 44;
pub const GDBM_FILE_CLOSE_ERROR: C2RustUnnamed_0 = 37;
pub const GDBM_FILE_EOF: C2RustUnnamed_0 = 25;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_0 = 3;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed_0 = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed_0 = 5;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed_0 = 24;
pub const GDBM_FILE_SYNC_ERROR: C2RustUnnamed_0 = 38;
pub const GDBM_FILE_TRUNCATE_ERROR: C2RustUnnamed_0 = 39;
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_0 = 4;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_0 = 15;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_0 = 29;
pub const GDBM_NO_DBNAME: C2RustUnnamed_0 = 26;
pub const GDBM_NO_ERROR: C2RustUnnamed_0 = 0;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed_0 = 19;
pub const GDBM_OPT_BADVAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed_0 = 20;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed_0 = 11;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed_0 = 13;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed_0 = 12;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed_0 = 16;
pub const GDBM_SNAPSHOT_BAD: gdbm_latest_snapshot_status = 1;
pub const GDBM_SNAPSHOT_ERR: gdbm_latest_snapshot_status = 2;
pub const GDBM_SNAPSHOT_OK: gdbm_latest_snapshot_status = 0;
pub const GDBM_SNAPSHOT_SAME: gdbm_latest_snapshot_status = 3;
pub const GDBM_SNAPSHOT_SUSPICIOUS: gdbm_latest_snapshot_status = 4;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed_0 = 14;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type gdbm_latest_snapshot_status = libc::c_uint;


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
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}

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
/* gdbm.h  -  The include file for dbm users.  -*- c -*- */
/*  This file is part of GDBM, the GNU data base manager, by Philip A. Nelson.
    Copyright (C) 1990-2022 Free Software Foundation, Inc.

    GDBM is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2, or (at your option)
    any later version.

    GDBM is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with GDBM. If not, see <http://www.gnu.org/licenses/>.  

    You may contact the author by:
       e-mail:  phil@cs.wwu.edu
      us-mail:  Philip A. Nelson
                Computer Science Department
                Western Washington University
                Bellingham, WA 98226
       
*************************************************************************/
/* Protection for multiple includes. */
/* GDBM C++ support */
/* Parameters to gdbm_open for READERS, WRITERS, and WRITERS who
   can create the database. */
/* A reader. */
/* A writer. */
/* A writer.  Create the db if needed. */
/* A writer.  Always create a new db. */
/* Mask for the above. */
/* Write fast! => No fsyncs.  OBSOLETE. */
/* Sync operations to the disk. */
/* Don't do file locking operations. */
/* Don't use mmap(). */
/* Close the underlying fd on exec(3) */
/* Don't adjust block_size. Bail out with
				   GDBM_BLOCK_SIZE_ERROR error if unable to
				   set it. */
/* Only for gdbm_fd_open: close fd on error. */
/* Additional consistency checks. */
/* Enable pre-fault reading of mmapped regions. */
/* Enable the numsync extension */
/* Parameters to gdbm_store for simple insertion or replacement in the
   case that the key is already in the database. */
/* Never replace old data with new. */
/* Always replace old data with new. */
/* Parameters to gdbm_setopt, specifying the type of operation to perform. */
/* Set the cache size. */
/* Toggle fast mode.  OBSOLETE. */
/* Turn on or off sync operations. */
/* Keep all free blocks in the header. */
/* Attempt to coalesce free blocks. */
/* Set maximum mapped memory size */
/* Toggle mmap mode */
/* Compatibility defines: */
/* Get gdbm_open flags */
/* Get mmap status */
/* Get current cache side */
/* Get synch mode */
/* Get "centfree" status */
/* Get free block coalesce status */
/* Get maximum mapped memory size */
/* Return database file name */
/* Return block size */
/* Return the database format */
/* Directory depth: number of initial (most
				    significant) bits in hash interpreted as
				    index to the directory. */
/* Get number of elements per bucket */
/* Get the value of cache auto-adjustment */
/* Set the value of cache auto-adjustment */
/* The data and key structure. */
/* A pointer to the GDBM file. */
/* External variable, the gdbm build release string. */
/* GDBM external functions. */
/* Make sure the database is all on disk. */
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