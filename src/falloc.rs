#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn gdbm_db_strerror(dbf: GDBM_FILE) -> *const libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn _gdbm_fatal(_: GDBM_FILE, _: *const libc::c_char);
    fn _gdbm_mapped_lseek(_: GDBM_FILE, _: off_t, _: libc::c_int) -> off_t;
    fn gdbm_avail_block_validate(
        dbf: GDBM_FILE,
        avblk: *mut avail_block,
        size: size_t,
    ) -> libc::c_int;
    fn _gdbm_full_write(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
    fn _gdbm_full_read(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
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
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed_0 = 15;
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
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_0 = 4;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_0 = 3;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_0 = 2;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
pub const GDBM_NO_ERROR: C2RustUnnamed_0 = 0;
#[inline]
unsafe extern "C" fn _gdbm_current_bucket_changed(mut dbf: GDBM_FILE) {
    (*(*dbf).cache_mru).ca_changed = 1 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern "C" fn gdbm_file_seek(
    mut dbf: GDBM_FILE,
    mut off: off_t,
    mut whence: libc::c_int,
) -> off_t {
    return _gdbm_mapped_lseek(dbf, off, whence);
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
