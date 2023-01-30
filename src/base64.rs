#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const GDBM_ERR_USAGE: C2RustUnnamed = 44;
pub const GDBM_ERR_REALPATH: C2RustUnnamed = 43;
pub const GDBM_ERR_SNAPSHOT_CLONE: C2RustUnnamed = 42;
pub const GDBM_BAD_HASH_ENTRY: C2RustUnnamed = 41;
pub const GDBM_BUCKET_CACHE_CORRUPTED: C2RustUnnamed = 40;
pub const GDBM_FILE_TRUNCATE_ERROR: C2RustUnnamed = 39;
pub const GDBM_FILE_SYNC_ERROR: C2RustUnnamed = 38;
pub const GDBM_FILE_CLOSE_ERROR: C2RustUnnamed = 37;
pub const GDBM_BAD_DIR_ENTRY: C2RustUnnamed = 36;
pub const GDBM_BAD_HASH_TABLE: C2RustUnnamed = 35;
pub const GDBM_BAD_AVAIL: C2RustUnnamed = 34;
pub const GDBM_BAD_HEADER: C2RustUnnamed = 33;
pub const GDBM_BAD_BUCKET: C2RustUnnamed = 32;
pub const GDBM_DIR_OVERFLOW: C2RustUnnamed = 31;
pub const GDBM_BACKUP_FAILED: C2RustUnnamed = 30;
pub const GDBM_NEED_RECOVERY: C2RustUnnamed = 29;
pub const GDBM_ERR_FILE_MODE: C2RustUnnamed = 28;
pub const GDBM_ERR_FILE_OWNER: C2RustUnnamed = 27;
pub const GDBM_NO_DBNAME: C2RustUnnamed = 26;
pub const GDBM_FILE_EOF: C2RustUnnamed = 25;
pub const GDBM_FILE_STAT_ERROR: C2RustUnnamed = 24;
pub const GDBM_BAD_OPEN_FLAGS: C2RustUnnamed = 23;
pub const GDBM_BAD_FILE_OFFSET: C2RustUnnamed = 22;
pub const GDBM_BYTE_SWAPPED: C2RustUnnamed = 21;
pub const GDBM_OPT_ILLEGAL: C2RustUnnamed = 20;
pub const GDBM_OPT_BADVAL: C2RustUnnamed = 20;
pub const GDBM_OPT_ALREADY_SET: C2RustUnnamed = 19;
pub const GDBM_ILLEGAL_DATA: C2RustUnnamed = 18;
pub const GDBM_MALFORMED_DATA: C2RustUnnamed = 18;
pub const GDBM_CANNOT_REPLACE: C2RustUnnamed = 17;
pub const GDBM_REORGANIZE_FAILED: C2RustUnnamed = 16;
pub const GDBM_ITEM_NOT_FOUND: C2RustUnnamed = 15;
pub const GDBM_UNKNOWN_ERROR: C2RustUnnamed = 14;
pub const GDBM_READER_CANT_REORGANIZE: C2RustUnnamed = 13;
pub const GDBM_READER_CANT_STORE: C2RustUnnamed = 12;
pub const GDBM_READER_CANT_DELETE: C2RustUnnamed = 11;
pub const GDBM_CANT_BE_WRITER: C2RustUnnamed = 10;
pub const GDBM_CANT_BE_READER: C2RustUnnamed = 9;
pub const GDBM_EMPTY_DATABASE: C2RustUnnamed = 8;
pub const GDBM_BAD_MAGIC_NUMBER: C2RustUnnamed = 7;
pub const GDBM_FILE_READ_ERROR: C2RustUnnamed = 6;
pub const GDBM_FILE_SEEK_ERROR: C2RustUnnamed = 5;
pub const GDBM_FILE_WRITE_ERROR: C2RustUnnamed = 4;
pub const GDBM_FILE_OPEN_ERROR: C2RustUnnamed = 3;
pub const GDBM_BLOCK_SIZE_ERROR: C2RustUnnamed = 2;
pub const GDBM_MALLOC_ERROR: C2RustUnnamed = 1;
pub const GDBM_NO_ERROR: C2RustUnnamed = 0;
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
