#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, register_tool)]
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut gdbm_version: *const libc::c_char;
    fn gdbm_fetch(_: GDBM_FILE, _: datum) -> datum;
    fn gdbm_firstkey(_: GDBM_FILE) -> datum;
    fn gdbm_nextkey(_: GDBM_FILE, _: datum) -> datum;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn gdbm_errno_location() -> *mut libc::c_int;
    fn gdbm_clear_error(dbf: GDBM_FILE);
    fn gdbm_last_errno(dbf: GDBM_FILE) -> gdbm_error;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
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
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_0 = 4;
pub const GDBM_NO_ERROR: C2RustUnnamed_0 = 0;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_0 = 15;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_0 = 29;
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
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed_0 = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed_0 = 27;
pub const GDBM_NO_DBNAME: C2RustUnnamed_0 = 26;
pub const GDBM_FILE_EOF: C2RustUnnamed_0 = 25;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed_0 = 24;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed_0 = 23;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed_0 = 22;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed_0 = 21;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_BADVAL: C2RustUnnamed_0 = 20;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed_0 = 19;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed_0 = 18;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed_0 = 18;
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
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed_0 = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed_0 = 5;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_0 = 2;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
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
