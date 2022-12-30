#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn gdbm_db_strerror(dbf: GDBM_FILE) -> *const libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn _gdbm_get_bucket(_: GDBM_FILE, _: libc::c_int) -> libc::c_int;
    fn _gdbm_fatal(_: GDBM_FILE, _: *const libc::c_char);
    fn _gdbm_full_read(_: GDBM_FILE, _: *mut libc::c_void, _: size_t) -> libc::c_int;
    fn _gdbm_mapped_lseek(_: GDBM_FILE, _: off_t, _: libc::c_int) -> off_t;
    fn _gdbm_file_size(dbf: GDBM_FILE, psize: *mut off_t) -> libc::c_int;
    fn _gdbm_hash_key(
        dbf: GDBM_FILE,
        key: datum,
        hash: *mut libc::c_int,
        bucket: *mut libc::c_int,
        offset: *mut libc::c_int,
    );
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn gdbm_file_seek(
    mut dbf: GDBM_FILE,
    mut off: off_t,
    mut whence: libc::c_int,
) -> off_t {
    return _gdbm_mapped_lseek(dbf, off, whence);
}
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
