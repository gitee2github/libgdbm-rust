#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gdbm_fd_open(
        fd: libc::c_int,
        file_name: *const libc::c_char,
        block_size: libc::c_int,
        flags: libc::c_int,
        fatal_func: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    ) -> GDBM_FILE;
    fn gdbm_close(_: GDBM_FILE) -> libc::c_int;
    fn gdbm_store(_: GDBM_FILE, _: datum, _: datum, _: libc::c_int) -> libc::c_int;
    fn gdbm_sync(_: GDBM_FILE) -> libc::c_int;
    fn gdbm_clear_error(dbf: GDBM_FILE);
    fn _gdbm_get_bucket(_: GDBM_FILE, _: libc::c_int) -> libc::c_int;
    fn gdbm_file_sync(dbf: GDBM_FILE) -> libc::c_int;
    fn _gdbm_cache_free(dbf: GDBM_FILE);
    fn _gdbm_cache_flush(dbf: GDBM_FILE) -> libc::c_int;
    fn _gdbm_unlock_file(_: GDBM_FILE);
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn _gdbm_cache_init(_: GDBM_FILE, _: size_t) -> libc::c_int;
    fn gdbm_errno_location() -> *mut libc::c_int;
    fn _gdbm_mapped_unmap(_: GDBM_FILE);
    fn _gdbm_end_update(_: GDBM_FILE) -> libc::c_int;
    fn gdbm_db_strerror(dbf: GDBM_FILE) -> *const libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn gdbm_last_errno(dbf: GDBM_FILE) -> gdbm_error;
    fn _gdbm_read_entry(_: GDBM_FILE, _: libc::c_int) -> *mut libc::c_char;
    fn _gdbm_hash_key(
        dbf: GDBM_FILE,
        key: datum,
        hash: *mut libc::c_int,
        bucket: *mut libc::c_int,
        offset: *mut libc::c_int,
    );
    fn _gdbm_validate_header(dbf: GDBM_FILE) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct datum {
    pub dptr: *mut libc::c_char,
    pub dsize: libc::c_int,
}
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
#[derive(Copy, Clone)]
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
pub type gdbm_recovery = gdbm_recovery_s;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed_0 = 16;
pub const GDBM_BACKUP_FAILED: C2RustUnnamed_0 = 30;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed_0 = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed_0 = 27;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed_0 = 24;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed_0 = 17;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_0 = 3;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed_0 = 13;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const GDBM_ERR_USAGE: C2RustUnnamed_0 = 44;
pub const GDBM_ERR_REALPATH: C2RustUnnamed_0 = 43;
pub const GDBM_ERR_SNAPSHOT_CLONE: C2RustUnnamed_0 = 42;
pub const GDBM_BAD_HASH_ENTRY: C2RustUnnamed_0 = 41;
pub const GDBM_BUCKET_CACHE_CORRUPTED: C2RustUnnamed_0 = 40;
pub const GDBM_FILE_TRUNCATE_ERROR: C2RustUnnamed_0 = 39;
pub const GDBM_FILE_SYNC_ERROR: C2RustUnnamed_0 = 38;
pub const GDBM_FILE_CLOSE_ERROR: C2RustUnnamed_0 = 37;
pub const GDBM_BAD_DIR_ENTRY: C2RustUnnamed_0 = 36;
pub const GDBM_BAD_HASH_TABLE: C2RustUnnamed_0 = 35;
pub const GDBM_BAD_AVAIL: C2RustUnnamed_0 = 34;
pub const GDBM_BAD_HEADER: C2RustUnnamed_0 = 33;
pub const GDBM_BAD_BUCKET: C2RustUnnamed_0 = 32;
pub const GDBM_DIR_OVERFLOW: C2RustUnnamed_0 = 31;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_0 = 29;
pub const GDBM_NO_DBNAME: C2RustUnnamed_0 = 26;
pub const GDBM_FILE_EOF: C2RustUnnamed_0 = 25;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed_0 = 23;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed_0 = 22;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed_0 = 21;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_BADVAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed_0 = 19;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_0 = 15;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed_0 = 14;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed_0 = 12;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed_0 = 11;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed_0 = 10;
pub const GDBM_CANT_BE_READER: C2RustUnnamed_0 = 9;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed_0 = 8;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed_0 = 7;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed_0 = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed_0 = 5;
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_0 = 4;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_0 = 2;
pub const GDBM_NO_ERROR: C2RustUnnamed_0 = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
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
