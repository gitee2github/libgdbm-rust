#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gdbm_open(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    ) -> GDBM_FILE;
    fn gdbm_close(_: GDBM_FILE) -> libc::c_int;
    fn gdbm_store(_: GDBM_FILE, _: datum, _: datum, _: libc::c_int) -> libc::c_int;
    fn gdbm_convert(dbf: GDBM_FILE, flag: libc::c_int) -> libc::c_int;
    fn gdbm_fdesc(_: GDBM_FILE) -> libc::c_int;
    fn gdbm_import_from_file(
        dbf: GDBM_FILE,
        fp: *mut FILE,
        flag: libc::c_int,
    ) -> libc::c_int;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn gdbm_errno_location() -> *mut libc::c_int;
    fn _gdbm_base64_decode(
        input: *const libc::c_uchar,
        input_len: size_t,
        output: *mut *mut libc::c_uchar,
        output_size: *mut size_t,
        inbytes: *mut size_t,
        outbytes: *mut size_t,
    ) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub type FILE = _IO_FILE;
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
pub struct datbuf {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
}
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed_0 = 27;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed_0 = 24;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
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
pub const GDBM_MALFORMED_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed_0 = 6;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_0 = 15;
pub const GDBM_NO_DBNAME: C2RustUnnamed_0 = 26;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_0 = 3;
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
pub const GDBM_BACKUP_FAILED: C2RustUnnamed_0 = 30;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_0 = 29;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed_0 = 28;
pub const GDBM_FILE_EOF: C2RustUnnamed_0 = 25;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed_0 = 23;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed_0 = 22;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed_0 = 21;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_BADVAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed_0 = 19;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed_0 = 17;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed_0 = 16;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed_0 = 14;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed_0 = 13;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed_0 = 12;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed_0 = 11;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed_0 = 10;
pub const GDBM_CANT_BE_READER: C2RustUnnamed_0 = 9;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed_0 = 8;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed_0 = 7;
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
