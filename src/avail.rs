#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn _gdbm_avail_block_read(
        dbf: GDBM_FILE,
        avblk: *mut avail_block,
        size: size_t,
    ) -> libc::c_int;
    fn gdbm_set_errno(dbf: GDBM_FILE, ec: gdbm_error, fatal: libc::c_int);
    fn _gdbm_mapped_lseek(_: GDBM_FILE, _: off_t, _: libc::c_int) -> off_t;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct off_map {
    pub map_base: *mut off_t,
    pub map_size: size_t,
    pub map_max: size_t,
}
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed_0 = 5;
pub const GDBM_BAD_AVAIL: C2RustUnnamed_0 = 34;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed_0 = 17;
pub const GDBM_NO_ERROR: C2RustUnnamed_0 = 0;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed_0 = 1;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed_0 = 29;
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
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed_0 = 4;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed_0 = 3;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed_0 = 2;
#[inline]
unsafe extern "C" fn gdbm_file_seek(
    mut dbf: GDBM_FILE,
    mut off: off_t,
    mut whence: libc::c_int,
) -> off_t {
    return _gdbm_mapped_lseek(dbf, off, whence);
}
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
