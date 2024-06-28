#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::collapsible_else_if)]

use std::{self, ptr};

pub const STBI__SCAN_header: i32 = 2;
pub const STBI__SCAN_load: i32 = 0;
pub const STBI__SCAN_type: i32 = 1;
pub const STBI_default: i32 = 0;
pub const STBI_grey: i32 = 1;
pub const STBI_grey_alpha: i32 = 2;
pub const STBI_ORDER_BGR: i32 = 1;
pub const STBI_ORDER_RGB: i32 = 0;
pub const STBI_rgb: i32 = 3;
pub const STBI_rgb_alpha: i32 = 4;
pub const STBI__F_avg: i32 = 3;
pub const STBI__F_avg_first: i32 = 5;
pub const STBI__F_none: i32 = 0;
pub const STBI__F_paeth: i32 = 4;
pub const STBI__F_paeth_first: i32 = 6;
pub const STBI__F_sub: i32 = 1;
pub const STBI__F_up: i32 = 2;

pub static mut stbi__check_png_header_png_sig: [u8; 8] = [
    ((137) as u8),
    ((80) as u8),
    ((78) as u8),
    ((71) as u8),
    ((13) as u8),
    ((10) as u8),
    ((26) as u8),
    ((10) as u8),
];

#[derive(Debug, Copy, Clone)]
pub struct stbi__png {
    pub s: *mut stbi__context,
    pub idata: *mut u8,
    pub expanded: *mut u8,
    pub out: *mut u8,
    pub depth: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi__pngchunk {
    pub length: u32,
    pub _type_: u32,
}

impl std::default::Default for stbi__png {
    fn default() -> Self {
        stbi__png {
            s: std::ptr::null_mut(),
            idata: std::ptr::null_mut(),
            expanded: std::ptr::null_mut(),
            out: std::ptr::null_mut(),
            depth: 0,
        }
    }
}

impl std::default::Default for stbi__pngchunk {
    fn default() -> Self {
        stbi__pngchunk {
            length: 0,
            _type_: 0,
        }
    }
}

unsafe fn stbi__check_png_header(s: *mut stbi__context) -> i32 {
    let mut i: i32 = 0;
    i = (0) as i32;
    while i < 8 {
        if ((stbi__get8(s)) as i32) != ((stbi__check_png_header_png_sig[(i) as usize]) as i32) {
            return 0;
        }
        c_runtime::preInc(&mut i);
    }
    return (1) as i32;
}

unsafe fn stbi__compute_transparency(
    z: *mut stbi__png,
    tc: [u8; 3],
    out_n: i32,
) -> i32 {
    let s: *mut stbi__context = (*z).s;
    let mut i: u32 = 0;
    let pixel_count: u32 = (*s).img_x * (*s).img_y;
    let mut p: *mut u8 = (*z).out;

    if out_n == 2 {
        i = (0) as u32;
        while i < pixel_count {
            *p.offset((1) as isize) =
                (if ((*p.offset((0) as isize)) as i32) == ((tc[(0) as usize]) as i32) {
                    0
                } else {
                    255
                }) as u8;
            p = p.offset((2) as isize);
            c_runtime::preInc(&mut i);
        }
    } else {
        i = (0) as u32;
        while i < pixel_count {
            if ((*p.offset((0) as isize)) as i32) == ((tc[(0) as usize]) as i32)
                && ((*p.offset((1) as isize)) as i32) == ((tc[(1) as usize]) as i32)
                && ((*p.offset((2) as isize)) as i32) == ((tc[(2) as usize]) as i32)
            {
                *p.offset((3) as isize) = 0 as u8;
            }
            p = p.offset((4) as isize);
            c_runtime::preInc(&mut i);
        }
    }
    return 1 as i32;
}

unsafe fn stbi__compute_transparency16(
    z: *mut stbi__png,
    tc: [u16; 3],
    out_n: i32,
) -> i32 {
    let s: *mut stbi__context = (*z).s;
    let mut i: u32 = 0;
    let pixel_count: u32 = (*s).img_x * (*s).img_y;
    let mut p: *mut u16 = ((*z).out) as *mut u16;

    if out_n == 2 {
        i = 0 as u32;
        while i < pixel_count {
            *p.offset((1) as isize) =
                (if ((*p.offset((0) as isize)) as i32) == ((tc[(0) as usize]) as i32) {
                    0
                } else {
                    65535
                }) as u16;
            p = p.offset((2) as isize);
            c_runtime::preInc(&mut i);
        }
    } else {
        i = (0) as u32;
        while i < pixel_count {
            if ((*p.offset((0) as isize)) as i32) == ((tc[(0) as usize]) as i32)
                && ((*p.offset((1) as isize)) as i32) == ((tc[(1) as usize]) as i32)
                && ((*p.offset((2) as isize)) as i32) == ((tc[(2) as usize]) as i32)
            {
                *p.offset((3) as isize) = (0) as u16;
            }
            p = p.offset((4) as isize);
            c_runtime::preInc(&mut i);
        }
    }
    return (1) as i32;
}

unsafe fn stbi__create_png_image(
    a: *mut stbi__png,
    mut image_data: *mut u8,
    mut image_data_len: u32,
    out_n: i32,
    depth: i32,
    color: i32,
    interlaced: i32,
) -> i32 {
    let bytes: i32 = if depth == 16 { 2 } else { 1 };
    let out_bytes: i32 = out_n * bytes;
    let mut _final_: *mut u8 = std::ptr::null_mut();
    let mut p: i32 = 0;
    if interlaced == 0 {
        return (stbi__create_png_image_raw(
            a,
            image_data,
            image_data_len,
            out_n,
            (*(*a).s).img_x,
            (*(*a).s).img_y,
            depth,
            color,
        )) as i32;
    }
    _final_ = stbi__malloc_mad3(
        ((*(*a).s).img_x) as i32,
        ((*(*a).s).img_y) as i32,
        out_bytes,
        0,
    );
    if _final_ == std::ptr::null_mut() {
        return 0;
    }
    p = (0) as i32;
    while p < 7 {
        let xorig: [i32; 7] = [0, 4, 0, 2, 0, 1, 0];
        let yorig: [i32; 7] = [0, 0, 4, 0, 2, 0, 1];
        let xspc: [i32; 7] = [8, 8, 4, 4, 2, 2, 1];
        let yspc: [i32; 7] = [8, 8, 8, 4, 4, 2, 2];
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        x = (((*(*a).s).img_x - ((xorig[(p) as usize]) as u32) + ((xspc[(p) as usize]) as u32)
            - ((1) as u32))
            / ((xspc[(p) as usize]) as u32)) as i32;
        y = (((*(*a).s).img_y - ((yorig[(p) as usize]) as u32) + ((yspc[(p) as usize]) as u32)
            - ((1) as u32))
            / ((yspc[(p) as usize]) as u32)) as i32;
        if (x) != 0 && (y) != 0 {
            let img_len: u32 = ((((((*(*a).s).img_n * x * depth) + 7) >> 3) + 1) * y) as u32;
            if stbi__create_png_image_raw(
                a,
                image_data,
                image_data_len,
                out_n,
                (x) as u32,
                (y) as u32,
                depth,
                color,
            ) == 0
            {
                c_runtime::free(_final_);
                return (0) as i32;
            }
            j = (0) as i32;
            while j < y {
                i = (0) as i32;
                while i < x {
                    let out_y: i32 = j * yspc[(p) as usize] + yorig[(p) as usize];
                    let out_x: i32 = i * xspc[(p) as usize] + xorig[(p) as usize];
                    c_runtime::memcpy(
                        ((_final_).offset(
                            (((out_y) as u32) * (*(*a).s).img_x * ((out_bytes) as u32)) as isize,
                        ))
                        .offset((out_x * out_bytes) as isize),
                        ((*a).out).offset(((j * x + i) * out_bytes) as isize),
                        (out_bytes) as u64,
                    );
                    c_runtime::preInc(&mut i);
                }
                c_runtime::preInc(&mut j);
            }
            c_runtime::free((*a).out);
            image_data = image_data.offset((img_len) as isize);
            image_data_len -= (img_len) as u32;
        }
        c_runtime::preInc(&mut p);
    }
    (*a).out = _final_;
    return (1) as i32;
}

unsafe fn stbi__create_png_image_raw(
    a: *mut stbi__png,
    mut raw: *mut u8,
    raw_len: u32,
    out_n: i32,
    x: u32,
    y: u32,
    depth: i32,
    color: i32,
) -> i32 {
    let bytes: i32 = if depth == 16 { 2 } else { 1 };
    let s: *mut stbi__context = (*a).s;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let stride: u32 = x * ((out_n) as u32) * ((bytes) as u32);
    let mut img_len: u32 = 0;
    let mut img_width_bytes: u32 = 0;
    let mut k: i32 = 0;
    let img_n: i32 = (*s).img_n;
    let output_bytes: i32 = out_n * bytes;
    let mut filter_bytes: i32 = img_n * bytes;
    let mut width: i32 = (x) as i32;

    (*a).out = stbi__malloc_mad3((x) as i32, (y) as i32, output_bytes, 0);
    if (*a).out == std::ptr::null_mut() {
        return 0;
    }
    if stbi__mad3sizes_valid(img_n, (x) as i32, depth, 7) == 0 {
        return 0;
    }
    img_width_bytes = ((((img_n) as u32) * x * ((depth) as u32)) + ((7) as u32)) >> 3;
    img_len = (img_width_bytes + ((1) as u32)) * y;
    if raw_len < img_len {
        return 0;
    }
    j = (0) as u32;
    while j < y {
        let mut cur: *mut u8 = ((*a).out).offset((stride * j) as isize);
        let mut prior: *mut u8 = std::ptr::null_mut();
        let mut filter: i32 = (*c_runtime::postIncPtr(&mut raw)) as i32;
        if filter > 4 {
            return 0;
        }
        if depth < 8 {
            if img_width_bytes > x {
                return 0;
            }
            cur = cur.offset((x * ((out_n) as u32) - img_width_bytes) as isize);
            filter_bytes = (1) as i32;
            width = (img_width_bytes) as i32;
        }
        prior = (cur).offset(-((stride) as isize));
        if j == ((0) as u32) {
            filter = (first_row_filter[(filter) as usize]) as i32;
        }
        k = (0) as i32;
        while k < filter_bytes {
            {
                if filter == STBI__F_none {
                    *cur.offset((k) as isize) = (*raw.offset((k) as isize)) as u8;
                } else if filter == STBI__F_sub {
                    *cur.offset((k) as isize) = (*raw.offset((k) as isize)) as u8;
                } else if filter == STBI__F_up {
                    *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                        + ((*prior.offset((k) as isize)) as i32))
                        & 255) as u8;
                } else if filter == STBI__F_avg {
                    *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                        + (((*prior.offset((k) as isize)) as i32) >> 1))
                        & 255) as u8;
                } else if filter == STBI__F_paeth {
                    *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                        + stbi__paeth(0, (*prior.offset((k) as isize)) as i32, 0))
                        & 255) as u8;
                } else if filter == STBI__F_avg_first {
                    *cur.offset((k) as isize) = (*raw.offset((k) as isize)) as u8;
                } else if filter == STBI__F_paeth_first {
                    *cur.offset((k) as isize) = (*raw.offset((k) as isize)) as u8;
                }
            }
            c_runtime::preInc(&mut k);
        }
        if depth == 8 {
            if img_n != out_n {
                *cur.offset((img_n) as isize) = (255) as u8;
            }
            raw = raw.offset((img_n) as isize);
            cur = cur.offset((out_n) as isize);
            prior = prior.offset((out_n) as isize);
        } else {
            if depth == 16 {
                if img_n != out_n {
                    *cur.offset((filter_bytes) as isize) = (255) as u8;
                    *cur.offset((filter_bytes + 1) as isize) = (255) as u8;
                }
                raw = raw.offset((filter_bytes) as isize);
                cur = cur.offset((output_bytes) as isize);
                prior = prior.offset((output_bytes) as isize);
            } else {
                raw = raw.offset((1) as isize);
                cur = cur.offset((1) as isize);
                prior = prior.offset((1) as isize);
            }
        }
        if depth < 8 || img_n == out_n {
            let nk: i32 = (width - 1) * filter_bytes;
            {
                if filter == STBI__F_none {
                    c_runtime::memcpy(cur, raw, (nk) as u64);
                } else if filter == STBI__F_sub {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + ((*cur.offset((k - filter_bytes) as isize)) as i32))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                } else if filter == STBI__F_up {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + ((*prior.offset((k) as isize)) as i32))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                } else if filter == STBI__F_avg {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + ((((*prior.offset((k) as isize)) as i32)
                                + ((*cur.offset((k - filter_bytes) as isize)) as i32))
                                >> 1))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                } else if filter == STBI__F_paeth {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + stbi__paeth(
                                (*cur.offset((k - filter_bytes) as isize)) as i32,
                                (*prior.offset((k) as isize)) as i32,
                                (*prior.offset((k - filter_bytes) as isize)) as i32,
                            ))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                } else if filter == STBI__F_avg_first {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + (((*cur.offset((k - filter_bytes) as isize)) as i32) >> 1))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                } else if filter == STBI__F_paeth_first {
                    k = (0) as i32;
                    while k < nk {
                        *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                            + stbi__paeth(
                                (*cur.offset((k - filter_bytes) as isize)) as i32,
                                0,
                                0,
                            ))
                            & 255) as u8;
                        c_runtime::preInc(&mut k);
                    }
                }
            }
            raw = raw.offset((nk) as isize);
        } else {
            {
                if filter == STBI__F_none {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = (*raw.offset((k) as isize)) as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_sub {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + ((*cur.offset((k - output_bytes) as isize)) as i32))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_up {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + ((*prior.offset((k) as isize)) as i32))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_avg {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + ((((*prior.offset((k) as isize)) as i32)
                                    + ((*cur.offset((k - output_bytes) as isize)) as i32))
                                    >> 1))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_paeth {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + stbi__paeth(
                                    (*cur.offset((k - output_bytes) as isize)) as i32,
                                    (*prior.offset((k) as isize)) as i32,
                                    (*prior.offset((k - output_bytes) as isize)) as i32,
                                ))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_avg_first {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + (((*cur.offset((k - output_bytes) as isize)) as i32) >> 1))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                } else if filter == STBI__F_paeth_first {
                    i = x - ((1) as u32);
                    while i >= ((1) as u32) {
                        k = (0) as i32;
                        while k < filter_bytes {
                            *cur.offset((k) as isize) = ((((*raw.offset((k) as isize)) as i32)
                                + stbi__paeth(
                                    (*cur.offset((k - output_bytes) as isize)) as i32,
                                    0,
                                    0,
                                ))
                                & 255)
                                as u8;
                            c_runtime::preInc(&mut k);
                        }
                        c_runtime::preDec(&mut i);
                        *cur.offset((filter_bytes) as isize) = (255) as u8;
                        raw = raw.offset((filter_bytes) as isize);
                        cur = cur.offset((output_bytes) as isize);
                        prior = prior.offset((output_bytes) as isize);
                    }
                }
            }
            if depth == 16 {
                cur = ((*a).out).offset((stride * j) as isize);
                i = (0) as u32;
                while i < x {
                    *cur.offset((filter_bytes + 1) as isize) = (255) as u8;
                    c_runtime::preInc(&mut i);
                    cur = cur.offset((output_bytes) as isize);
                }
            }
        }
        c_runtime::preInc(&mut j);
    }
    if depth < 8 {
        j = (0) as u32;
        while j < y {
            let mut cur: *mut u8 = ((*a).out).offset((stride * j) as isize);
            let mut _in_: *mut u8 = ((((*a).out).offset((stride * j) as isize))
                .offset((x * ((out_n) as u32)) as isize))
            .offset(-((img_width_bytes) as isize));
            let scale: u8 = (if color == 0 {
                (stbi__depth_scale_table[(depth) as usize]) as i32
            } else {
                1
            }) as u8;
            if depth == 4 {
                k = (x * ((img_n) as u32)) as i32;
                while k >= 2 {
                    *c_runtime::postIncPtr(&mut cur) =
                        (((scale) as i32) * (((*_in_) as i32) >> 4)) as u8;
                    *c_runtime::postIncPtr(&mut cur) =
                        (((scale) as i32) * (((*_in_) as i32) & 0x0f)) as u8;
                    k -= (2) as i32;
                    c_runtime::preIncPtr(&mut _in_);
                }
                if k > 0 {
                    *c_runtime::postIncPtr(&mut cur) =
                        (((scale) as i32) * (((*_in_) as i32) >> 4)) as u8;
                }
            } else {
                if depth == 2 {
                    k = (x * ((img_n) as u32)) as i32;
                    while k >= 4 {
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * (((*_in_) as i32) >> 6)) as u8;
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * ((((*_in_) as i32) >> 4) & 0x03)) as u8;
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * ((((*_in_) as i32) >> 2) & 0x03)) as u8;
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * (((*_in_) as i32) & 0x03)) as u8;
                        k -= (4) as i32;
                        c_runtime::preIncPtr(&mut _in_);
                    }
                    if k > 0 {
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * (((*_in_) as i32) >> 6)) as u8;
                    }
                    if k > 1 {
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * ((((*_in_) as i32) >> 4) & 0x03)) as u8;
                    }
                    if k > 2 {
                        *c_runtime::postIncPtr(&mut cur) =
                            (((scale) as i32) * ((((*_in_) as i32) >> 2) & 0x03)) as u8;
                    }
                } else {
                    if depth == 1 {
                        k = (x * ((img_n) as u32)) as i32;
                        while k >= 8 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * (((*_in_) as i32) >> 7)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 6) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 5) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 4) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 3) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 2) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 1) & 0x01)) as u8;
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * (((*_in_) as i32) & 0x01)) as u8;
                            k -= (8) as i32;
                            c_runtime::preIncPtr(&mut _in_);
                        }
                        if k > 0 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * (((*_in_) as i32) >> 7)) as u8;
                        }
                        if k > 1 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 6) & 0x01)) as u8;
                        }
                        if k > 2 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 5) & 0x01)) as u8;
                        }
                        if k > 3 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 4) & 0x01)) as u8;
                        }
                        if k > 4 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 3) & 0x01)) as u8;
                        }
                        if k > 5 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 2) & 0x01)) as u8;
                        }
                        if k > 6 {
                            *c_runtime::postIncPtr(&mut cur) =
                                (((scale) as i32) * ((((*_in_) as i32) >> 1) & 0x01)) as u8;
                        }
                    }
                }
            }
            if img_n != out_n {
                let mut q: i32 = 0;
                cur = ((*a).out).offset((stride * j) as isize);
                if img_n == 1 {
                    q = (x - ((1) as u32)) as i32;
                    while q >= 0 {
                        *cur.offset((q * 2 + 1) as isize) = (255) as u8;
                        *cur.offset((q * 2 + 0) as isize) = (*cur.offset((q) as isize)) as u8;
                        c_runtime::preDec(&mut q);
                    }
                } else {
                    q = (x - ((1) as u32)) as i32;
                    while q >= 0 {
                        *cur.offset((q * 4 + 3) as isize) = (255) as u8;
                        *cur.offset((q * 4 + 2) as isize) =
                            (*cur.offset((q * 3 + 2) as isize)) as u8;
                        *cur.offset((q * 4 + 1) as isize) =
                            (*cur.offset((q * 3 + 1) as isize)) as u8;
                        *cur.offset((q * 4 + 0) as isize) =
                            (*cur.offset((q * 3 + 0) as isize)) as u8;
                        c_runtime::preDec(&mut q);
                    }
                }
            }
            c_runtime::preInc(&mut j);
        }
    } else {
        if depth == 16 {
            let mut cur: *mut u8 = (*a).out;
            let mut cur16: *mut u16 = (cur) as *mut u16;
            i = (0) as u32;
            while i < x * y * ((out_n) as u32) {
                *cur16 = ((((*cur.offset((0) as isize)) as i32) << 8)
                    | ((*cur.offset((1) as isize)) as i32)) as u16;
                c_runtime::preInc(&mut i);
                c_runtime::postIncPtr(&mut cur16);
                cur = cur.offset((2) as isize);
            }
        }
    }
    return (1) as i32;
}

unsafe fn stbi__de_iphone(z: *mut stbi__png) {
    let s: *mut stbi__context = (*z).s;
    let mut i: u32 = 0;
    let pixel_count: u32 = (*s).img_x * (*s).img_y;
    let mut p: *mut u8 = (*z).out;
    if (*s).img_out_n == 3 {
        i = (0) as u32;
        while i < pixel_count {
            let t: u8 = *p.offset((0) as isize);
            *p.offset((0) as isize) = (*p.offset((2) as isize)) as u8;
            *p.offset((2) as isize) = (t) as u8;
            p = p.offset((3) as isize);
            c_runtime::preInc(&mut i);
        }
    } else {
        if (if (stbi__unpremultiply_on_load_set) != 0 {
            stbi__unpremultiply_on_load_local
        } else {
            stbi__unpremultiply_on_load_global
        }) != 0
        {
            i = (0) as u32;
            while i < pixel_count {
                let a: u8 = *p.offset((3) as isize);
                let t: u8 = *p.offset((0) as isize);
                if (a) != 0 {
                    let half: u8 = (((a) as i32) / 2) as u8;
                    *p.offset((0) as isize) = ((((*p.offset((2) as isize)) as i32) * 255
                        + ((half) as i32))
                        / ((a) as i32)) as u8;
                    *p.offset((1) as isize) = ((((*p.offset((1) as isize)) as i32) * 255
                        + ((half) as i32))
                        / ((a) as i32)) as u8;
                    *p.offset((2) as isize) =
                        ((((t) as i32) * 255 + ((half) as i32)) / ((a) as i32)) as u8;
                } else {
                    *p.offset((0) as isize) = (*p.offset((2) as isize)) as u8;
                    *p.offset((2) as isize) = (t) as u8;
                }
                p = p.offset((4) as isize);
                c_runtime::preInc(&mut i);
            }
        } else {
            i = (0) as u32;
            while i < pixel_count {
                let t: u8 = *p.offset((0) as isize);
                *p.offset((0) as isize) = (*p.offset((2) as isize)) as u8;
                *p.offset((2) as isize) = (t) as u8;
                p = p.offset((4) as isize);
                c_runtime::preInc(&mut i);
            }
        }
    }
}

unsafe fn stbi__do_png(
    p: *mut stbi__png,
    x: *mut i32,
    y: *mut i32,
    n: *mut i32,
    req_comp: i32,
    ri: *mut stbi__result_info,
) -> *mut u8 {
    let mut result: *mut u8 = std::ptr::null_mut();
    if req_comp < 0 || req_comp > 4 {
        return ptr::null_mut();
    }
    if (stbi__parse_png_file(p, STBI__SCAN_load, req_comp)) != 0 {
        if (*p).depth <= 8 {
            (*ri).bits_per_channel = (8) as i32;
        } else {
            if (*p).depth == 16 {
                (*ri).bits_per_channel = (16) as i32;
            } else {
                return ptr::null_mut();
            }
        }
        result = (*p).out;
        (*p).out = std::ptr::null_mut();
        if (req_comp) != 0 && req_comp != (*(*p).s).img_out_n {
            if (*ri).bits_per_channel == 8 {
                result = stbi__convert_format(
                    result,
                    (*(*p).s).img_out_n,
                    req_comp,
                    (*(*p).s).img_x,
                    (*(*p).s).img_y,
                );
            } else {
                result = stbi__convert_format16(
                    ((result) as *mut u16) as *mut u16,
                    (*(*p).s).img_out_n,
                    req_comp,
                    (*(*p).s).img_x,
                    (*(*p).s).img_y,
                ) as *mut u8;
            }
            (*(*p).s).img_out_n = (req_comp) as i32;
            if result == std::ptr::null_mut() {
                return result;
            }
        }
        *x = ((*(*p).s).img_x) as i32;
        *y = ((*(*p).s).img_y) as i32;
        if (n) != std::ptr::null_mut() {
            *n = ((*(*p).s).img_n) as i32;
        }
    }
    c_runtime::free((*p).out);
    (*p).out = std::ptr::null_mut();
    c_runtime::free((*p).expanded);
    (*p).expanded = std::ptr::null_mut();
    c_runtime::free((*p).idata);
    (*p).idata = std::ptr::null_mut();
    return result;
}

unsafe fn stbi__expand_png_palette(
    a: *mut stbi__png,
    palette: *mut u8,
    len: i32,
    pal_img_n: i32,
) -> i32 {
    let mut i: u32 = 0;
    let pixel_count: u32 = (*(*a).s).img_x * (*(*a).s).img_y;
    let mut p: *mut u8 = std::ptr::null_mut();
    let mut temp_out: *mut u8 = std::ptr::null_mut();
    let orig: *mut u8 = (*a).out;
    p = stbi__malloc_mad2((pixel_count) as i32, pal_img_n, 0);
    if p == std::ptr::null_mut() {
        return 0;
    }
    temp_out = p;
    if pal_img_n == 3 {
        i = (0) as u32;
        while i < pixel_count {
            let n: i32 = ((*orig.offset((i) as isize)) as i32) * 4;
            *p.offset((0) as isize) = (*palette.offset((n) as isize)) as u8;
            *p.offset((1) as isize) = (*palette.offset((n + 1) as isize)) as u8;
            *p.offset((2) as isize) = (*palette.offset((n + 2) as isize)) as u8;
            p = p.offset((3) as isize);
            c_runtime::preInc(&mut i);
        }
    } else {
        i = (0) as u32;
        while i < pixel_count {
            let n: i32 = ((*orig.offset((i) as isize)) as i32) * 4;
            *p.offset((0) as isize) = (*palette.offset((n) as isize)) as u8;
            *p.offset((1) as isize) = (*palette.offset((n + 1) as isize)) as u8;
            *p.offset((2) as isize) = (*palette.offset((n + 2) as isize)) as u8;
            *p.offset((3) as isize) = (*palette.offset((n + 3) as isize)) as u8;
            p = p.offset((4) as isize);
            c_runtime::preInc(&mut i);
        }
    }
    c_runtime::free((*a).out);
    (*a).out = temp_out;

    return (1) as i32;
}

unsafe fn stbi__get_chunk_header(s: *mut stbi__context) -> stbi__pngchunk {
    let mut c: stbi__pngchunk = stbi__pngchunk::default();
    c.length = (stbi__get32be(s)) as u32;
    c._type_ = (stbi__get32be(s)) as u32;
    return (c) as stbi__pngchunk;
}

unsafe fn stbi__parse_png_file(z: *mut stbi__png, scan: i32, req_comp: i32) -> i32 {
    let mut palette: [u8; 1024] = [0; 1024];
    let mut pal_img_n: u8 = (0) as u8;
    let mut has_trans: u8 = (0) as u8;
    let mut tc: [u8; 3] = [0; 3];
    let mut tc16: [u16; 3] = [0; 3];
    let mut ioff: u32 = (0) as u32;
    let mut idata_limit: u32 = (0) as u32;
    let mut i: u32 = 0;
    let mut pal_len: u32 = (0) as u32;
    let mut first: i32 = 1;
    let mut k: i32 = 0;
    let mut interlace: i32 = 0;
    let mut color: i32 = 0;
    let mut is_iphone: i32 = 0;
    let s: *mut stbi__context = (*z).s;
    (*z).expanded = std::ptr::null_mut();
    (*z).idata = std::ptr::null_mut();
    (*z).out = std::ptr::null_mut();
    if stbi__check_png_header(s) == 0 {
        return (0) as i32;
    }
    if scan == STBI__SCAN_type {
        return (1) as i32;
    };
    loop {
        let c: stbi__pngchunk = stbi__get_chunk_header(s);
        {
            if c._type_
                == ((((67) as u32) << 24)
                    + (((103) as u32) << 16)
                    + (((66) as u32) << 8)
                    + ((73) as u32))
            {
                is_iphone = (1) as i32;
                stbi__skip(s, (c.length) as i32);
            } else if c._type_
                == ((((73) as u32) << 24)
                    + (((72) as u32) << 16)
                    + (((68) as u32) << 8)
                    + ((82) as u32))
            {
                {
                    let mut comp: i32 = 0;
                    let mut filter: i32 = 0;
                    if first == 0 {
                        return 0;
                    }
                    first = (0) as i32;
                    if c.length != ((13) as u32) {
                        return 0;
                    }
                    (*s).img_x = (stbi__get32be(s)) as u32;
                    (*s).img_y = (stbi__get32be(s)) as u32;
                    if (*s).img_y > ((1 << 24) as u32) {
                        return 0;
                    }
                    if (*s).img_x > ((1 << 24) as u32) {
                        return 0;
                    }
                    (*z).depth = (stbi__get8(s)) as i32;
                    if (*z).depth != 1
                        && (*z).depth != 2
                        && (*z).depth != 4
                        && (*z).depth != 8
                        && (*z).depth != 16
                    {
                        return 0;
                    }
                    color = (stbi__get8(s)) as i32;
                    if color > 6 {
                        return 0;
                    }
                    if color == 3 && (*z).depth == 16 {
                        return 0;
                    }
                    if color == 3 {
                        pal_img_n = (3) as u8;
                    } else {
                        if (color & 1) != 0 {
                            return 0;
                        }
                    }
                    comp = (stbi__get8(s)) as i32;
                    if (comp) != 0 {
                        return 0;
                    }
                    filter = (stbi__get8(s)) as i32;
                    if (filter) != 0 {
                        return 0;
                    }
                    interlace = (stbi__get8(s)) as i32;
                    if interlace > 1 {
                        return 0;
                    }
                    if (*s).img_x == 0 || (*s).img_y == 0 {
                        return 0;
                    }
                    if pal_img_n == 0 {
                        (*s).img_n = ((if (color & 2) != 0 { 3 } else { 1 })
                            + (if (color & 4) != 0 { 1 } else { 0 }))
                            as i32;
                        if ((1 << 30) as u32) / (*s).img_x / (((*s).img_n) as u32) < (*s).img_y {
                            return 0;
                        }
                        if scan == STBI__SCAN_header {
                            return (1) as i32;
                        }
                    } else {
                        (*s).img_n = (1) as i32;
                        if ((1 << 30) as u32) / (*s).img_x / ((4) as u32) < (*s).img_y {
                            return 0;
                        }
                    }
                }
            } else if c._type_
                == ((((80) as u32) << 24)
                    + (((76) as u32) << 16)
                    + (((84) as u32) << 8)
                    + ((69) as u32))
            {
                {
                    if (first) != 0 {
                        return 0;
                    }
                    if c.length > ((256 * 3) as u32) {
                        return 0;
                    }
                    pal_len = c.length / ((3) as u32);
                    if pal_len * ((3) as u32) != c.length {
                        return 0;
                    }
                    i = (0) as u32;
                    while i < pal_len {
                        palette[(i * ((4) as u32) + ((0) as u32)) as usize] =
                            (stbi__get8(s)) as u8;
                        palette[(i * ((4) as u32) + ((1) as u32)) as usize] =
                            (stbi__get8(s)) as u8;
                        palette[(i * ((4) as u32) + ((2) as u32)) as usize] =
                            (stbi__get8(s)) as u8;
                        palette[(i * ((4) as u32) + ((3) as u32)) as usize] = (255) as u8;
                        c_runtime::preInc(&mut i);
                    }
                }
            } else if c._type_
                == ((((116) as u32) << 24)
                    + (((82) as u32) << 16)
                    + (((78) as u32) << 8)
                    + ((83) as u32))
            {
                {
                    if (first) != 0 {
                        return 0;
                    }
                    if ((*z).idata) != std::ptr::null_mut() {
                        return 0;
                    }
                    if (pal_img_n) != 0 {
                        if scan == STBI__SCAN_header {
                            (*s).img_n = (4) as i32;
                            return (1) as i32;
                        }
                        if pal_len == ((0) as u32) {
                            return 0;
                        }
                        if c.length > pal_len {
                            return 0;
                        }
                        pal_img_n = (4) as u8;
                        i = (0) as u32;
                        while i < c.length {
                            palette[(i * ((4) as u32) + ((3) as u32)) as usize] =
                                (stbi__get8(s)) as u8;
                            c_runtime::preInc(&mut i);
                        }
                    } else {
                        if ((*s).img_n & 1) == 0 {
                            return 0;
                        }
                        if c.length != (((*s).img_n) as u32) * ((2) as u32) {
                            return 0;
                        }
                        has_trans = (1) as u8;
                        if (*z).depth == 16 {
                            k = (0) as i32;
                            while k < (*s).img_n {
                                tc16[(k) as usize] = (stbi__get16be(s)) as u16;
                                c_runtime::preInc(&mut k);
                            }
                        } else {
                            k = (0) as i32;
                            while k < (*s).img_n {
                                tc[(k) as usize] = ((((stbi__get16be(s) & 255) as u8) as i32)
                                    * ((stbi__depth_scale_table[((*z).depth) as usize]) as i32))
                                    as u8;
                                c_runtime::preInc(&mut k);
                            }
                        }
                    }
                }
            } else if c._type_
                == ((((73) as u32) << 24)
                    + (((68) as u32) << 16)
                    + (((65) as u32) << 8)
                    + ((84) as u32))
            {
                {
                    if (first) != 0 {
                        return 0;
                    }
                    if ((pal_img_n) as i32) != 0 && pal_len == 0 {
                        return 0;
                    }
                    if scan == STBI__SCAN_header {
                        (*s).img_n = (pal_img_n) as i32;
                        return (1) as i32;
                    }
                    if ((ioff + c.length) as i32) < ((ioff) as i32) {
                        return (0) as i32;
                    }
                    if ioff + c.length > idata_limit {
                        let idata_limit_old: u32 = idata_limit;
                        let mut p: *mut u8 = std::ptr::null_mut();
                        if idata_limit == ((0) as u32) {
                            idata_limit = if c.length > ((4096) as u32) {
                                c.length
                            } else {
                                (4096) as u32
                            };
                        }
                        while ioff + c.length > idata_limit {
                            idata_limit *= (2) as u32;
                        }
                        p = c_runtime::realloc((*z).idata, (idata_limit) as u64);
                        if p == std::ptr::null_mut() {
                            return 0;
                        }
                        (*z).idata = p;
                    }
                    if stbi__getn(s, ((*z).idata).offset((ioff) as isize), (c.length) as i32) == 0
                    {
                        return 0;
                    }
                    ioff += (c.length) as u32;
                }
            } else if c._type_
                == ((((73) as u32) << 24)
                    + (((69) as u32) << 16)
                    + (((78) as u32) << 8)
                    + ((68) as u32))
            {
                {
                    let mut raw_len: u32 = 0;
                    let mut bpl: u32 = 0;
                    if (first) != 0 {
                        return 0;
                    }
                    if scan != STBI__SCAN_load {
                        return (1) as i32;
                    }
                    if (*z).idata == std::ptr::null_mut() {
                        return 0;
                    }
                    bpl = ((*s).img_x * (((*z).depth) as u32) + ((7) as u32)) / ((8) as u32);
                    raw_len = bpl * (*s).img_y * (((*s).img_n) as u32) + (*s).img_y;
                    (*z).expanded = (stbi_zlib_decode_malloc_guesssize_headerflag(
                        ((*z).idata) as *mut i8,
                        (ioff) as i32,
                        (raw_len) as i32,
                        ((&mut raw_len) as *mut u32) as *mut i32,
                        !is_iphone,
                    )) as *mut u8;
                    if (*z).expanded == std::ptr::null_mut() {
                        return (0) as i32;
                    }
                    c_runtime::free((*z).idata);
                    (*z).idata = std::ptr::null_mut();
                    if (req_comp == (*s).img_n + 1 && req_comp != 3 && pal_img_n == 0)
                        || ((has_trans) as i32) != 0
                    {
                        (*s).img_out_n = ((*s).img_n + 1) as i32;
                    } else {
                        (*s).img_out_n = ((*s).img_n) as i32;
                    }
                    if stbi__create_png_image(
                        z,
                        (*z).expanded,
                        raw_len,
                        (*s).img_out_n,
                        (*z).depth,
                        color,
                        interlace,
                    ) == 0
                    {
                        return (0) as i32;
                    }
                    if (has_trans) != 0 {
                        if (*z).depth == 16 {
                            if stbi__compute_transparency16(z, tc16, (*s).img_out_n) == 0 {
                                return (0) as i32;
                            }
                        } else {
                            if stbi__compute_transparency(z, tc, (*s).img_out_n) == 0 {
                                return (0) as i32;
                            }
                        }
                    }
                    if (is_iphone) != 0
                        && (if (stbi__de_iphone_flag_set) != 0 {
                            stbi__de_iphone_flag_local
                        } else {
                            stbi__de_iphone_flag_global
                        }) != 0
                        && (*s).img_out_n > 2
                    {
                        stbi__de_iphone(z);
                    }
                    if (pal_img_n) != 0 {
                        (*s).img_n = (pal_img_n) as i32;
                        (*s).img_out_n = (pal_img_n) as i32;
                        if req_comp >= 3 {
                            (*s).img_out_n = (req_comp) as i32;
                        }
                        if stbi__expand_png_palette(
                            z,
                            (palette.as_mut_ptr()) as *mut u8,
                            (pal_len) as i32,
                            (*s).img_out_n,
                        ) == 0
                        {
                            return (0) as i32;
                        }
                    } else {
                        if (has_trans) != 0 {
                            c_runtime::preInc(&mut (*s).img_n);
                        }
                    }
                    c_runtime::free((*z).expanded);
                    (*z).expanded = std::ptr::null_mut();
                    stbi__get32be(s);
                    return (1) as i32;
                }
            } else {
                if (first) != 0 {
                    return 0;
                }
                if (c._type_ & ((1 << 29) as u32)) == ((0) as u32) {
                    stbi__parse_png_file_invalid_chunk[(0) as usize] =
                        (((c._type_ >> 24) & ((255) as u32)) as u8) as i8;
                    stbi__parse_png_file_invalid_chunk[(1) as usize] =
                        (((c._type_ >> 16) & ((255) as u32)) as u8) as i8;
                    stbi__parse_png_file_invalid_chunk[(2) as usize] =
                        (((c._type_ >> 8) & ((255) as u32)) as u8) as i8;
                    stbi__parse_png_file_invalid_chunk[(3) as usize] =
                        (((c._type_ >> 0) & ((255) as u32)) as u8) as i8;
                    return 0;
                }
                stbi__skip(s, (c.length) as i32);
            }
        }
        stbi__get32be(s);
    }

    return 0;
}

unsafe fn stbi__png_info(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
) -> i32 {
    let mut p: stbi__png = stbi__png::default();
    p.s = s;
    return (stbi__png_info_raw((&mut p) as *mut stbi__png, x, y, comp)) as i32;
}

unsafe fn stbi__png_info_raw(
    p: *mut stbi__png,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
) -> i32 {
    if stbi__parse_png_file(p, STBI__SCAN_header, 0) == 0 {
        stbi__rewind((*p).s);
        return (0) as i32;
    }
    if (x) != std::ptr::null_mut() {
        *x = ((*(*p).s).img_x) as i32;
    }
    if (y) != std::ptr::null_mut() {
        *y = ((*(*p).s).img_y) as i32;
    }
    if (comp) != std::ptr::null_mut() {
        *comp = ((*(*p).s).img_n) as i32;
    }
    return (1) as i32;
}

unsafe fn stbi__png_is16(s: *mut stbi__context) -> i32 {
    let mut p: stbi__png = stbi__png::default();
    p.s = s;
    if stbi__png_info_raw(
        (&mut p) as *mut stbi__png,
        (std::ptr::null_mut()) as *mut i32,
        (std::ptr::null_mut()) as *mut i32,
        (std::ptr::null_mut()) as *mut i32,
    ) == 0
    {
        return (0) as i32;
    }
    if p.depth != 16 {
        stbi__rewind((p.s) as *mut stbi__context);
        return (0) as i32;
    }
    return (1) as i32;
}

unsafe fn stbi__png_load(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
    ri: *mut stbi__result_info,
) -> *mut u8 {
    let mut p: stbi__png = stbi__png::default();
    p.s = s;
    return stbi__do_png((&mut p) as *mut stbi__png, x, y, comp, req_comp, ri);
}

unsafe fn stbi__png_test(s: *mut stbi__context) -> i32 {
    let mut r: i32 = 0;
    r = (stbi__check_png_header(s)) as i32;
    stbi__rewind(s);
    return (r) as i32;
}

#[derive(Debug, Copy, Clone)]
pub struct stbi__zbuf {
    pub zbuffer: *mut u8,
    pub zbuffer_end: *mut u8,
    pub num_bits: i32,
    pub code_buffer: u32,
    pub zout: *mut i8,
    pub zout_start: *mut i8,
    pub zout_end: *mut i8,
    pub z_expandable: i32,
    pub z_length: stbi__zhuffman,
    pub z_distance: stbi__zhuffman,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi__zhuffman {
    pub fast: [u16; 512],
    pub firstcode: [u16; 16],
    pub maxcode: [i32; 17],
    pub firstsymbol: [u16; 16],
    pub size: [u8; 288],
    pub value: [u16; 288],
}

impl std::default::Default for stbi__zbuf {
    fn default() -> Self {
        stbi__zbuf {
            zbuffer: std::ptr::null_mut(),
            zbuffer_end: std::ptr::null_mut(),
            num_bits: 0,
            code_buffer: 0,
            zout: std::ptr::null_mut(),
            zout_start: std::ptr::null_mut(),
            zout_end: std::ptr::null_mut(),
            z_expandable: 0,
            z_length: stbi__zhuffman::default(),
            z_distance: stbi__zhuffman::default(),
        }
    }
}

impl std::default::Default for stbi__zhuffman {
    fn default() -> Self {
        stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        }
    }
}

unsafe fn stbi__compute_huffman_codes(a: *mut stbi__zbuf) -> i32 {
    let mut z_codelength: stbi__zhuffman = stbi__zhuffman::default();
    let mut lencodes: [u8; 455] = [0; 455];
    let mut codelength_sizes: [u8; 19] = [0; 19];
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let hlit: i32 = (stbi__zreceive(a, 5) + ((257) as u32)) as i32;
    let hdist: i32 = (stbi__zreceive(a, 5) + ((1) as u32)) as i32;
    let hclen: i32 = (stbi__zreceive(a, 4) + ((4) as u32)) as i32;
    let ntot: i32 = hlit + hdist;
    c_runtime::memset(
        (codelength_sizes.as_mut_ptr()) as *mut u8,
        0,
        19 * std::mem::size_of::<u8>() as u64,
    );
    i = (0) as i32;
    while i < hclen {
        let s: i32 = (stbi__zreceive(a, 3)) as i32;
        codelength_sizes[(stbi__compute_huffman_codes_length_dezigzag[(i) as usize]) as usize] =
            (s) as u8;
        c_runtime::preInc(&mut i);
    }
    if stbi__zbuild_huffman(
        (&mut z_codelength) as *mut stbi__zhuffman,
        (codelength_sizes.as_mut_ptr()) as *mut u8,
        19,
    ) == 0
    {
        return (0) as i32;
    }
    n = (0) as i32;
    while n < ntot {
        let mut c: i32 = stbi__zhuffman_decode(a, (&mut z_codelength) as *mut stbi__zhuffman);
        if c < 0 || c >= 19 {
            return 0;
        }
        if c < 16 {
            lencodes[(c_runtime::postInc(&mut n)) as usize] = (c) as u8;
        } else {
            let mut fill: u8 = (0) as u8;
            if c == 16 {
                c = (stbi__zreceive(a, 2) + ((3) as u32)) as i32;
                if n == 0 {
                    return 0;
                }
                fill = (lencodes[(n - 1) as usize]) as u8;
            } else {
                if c == 17 {
                    c = (stbi__zreceive(a, 3) + ((3) as u32)) as i32;
                } else {
                    if c == 18 {
                        c = (stbi__zreceive(a, 7) + ((11) as u32)) as i32;
                    } else {
                        return 0;
                    }
                }
            }
            if ntot - n < c {
                return 0;
            }
            c_runtime::memset(
                (lencodes.as_mut_ptr()).offset((n) as isize),
                (fill) as i32,
                (c) as u64,
            );
            n += (c) as i32;
        }
    }
    if n != ntot {
        return 0;
    }
    if stbi__zbuild_huffman(
        (&mut (*a).z_length) as *mut stbi__zhuffman,
        (lencodes.as_mut_ptr()) as *mut u8,
        hlit,
    ) == 0
    {
        return (0) as i32;
    }
    if stbi__zbuild_huffman(
        (&mut (*a).z_distance) as *mut stbi__zhuffman,
        (lencodes.as_mut_ptr()).offset((hlit) as isize),
        hdist,
    ) == 0
    {
        return (0) as i32;
    }
    return (1) as i32;
}

unsafe fn stbi__do_zlib(
    a: *mut stbi__zbuf,
    obuf: *mut i8,
    olen: i32,
    exp: i32,
    parse_header: i32,
) -> i32 {
    (*a).zout_start = obuf;
    (*a).zout = obuf;
    (*a).zout_end = (obuf).offset((olen) as isize);
    (*a).z_expandable = (exp) as i32;
    return (stbi__parse_zlib(a, parse_header)) as i32;
}

unsafe fn stbi__fill_bits(z: *mut stbi__zbuf) {
    loop {
        if (*z).code_buffer >= (1 << (*z).num_bits) {
            (*z).zbuffer = (*z).zbuffer_end;
            return;
        }
        (*z).code_buffer |= ((stbi__zget8(z)) as u32) << (*z).num_bits;
        (*z).num_bits += (8) as i32;
        if !((*z).num_bits <= 24) {
            break;
        }
    }
}

unsafe fn stbi__parse_huffman_block(a: *mut stbi__zbuf) -> i32 {
    let mut zout: *mut i8 = (*a).zout;
    loop {
        let mut z: i32 = stbi__zhuffman_decode(a, (&mut (*a).z_length) as *mut stbi__zhuffman);
        if z < 256 {
            if z < 0 {
                return 0;
            }
            if zout >= (*a).zout_end {
                if stbi__zexpand(a, zout, 1) == 0 {
                    return (0) as i32;
                }
                zout = (*a).zout;
            }
            *c_runtime::postIncPtr(&mut zout) = (z) as i8;
        } else {
            let mut p: *mut u8 = std::ptr::null_mut();
            let mut len: i32 = 0;
            let mut dist: i32 = 0;
            if z == 256 {
                (*a).zout = zout;
                return (1) as i32;
            }
            z -= (257) as i32;
            len = (stbi__zlength_base[(z) as usize]) as i32;
            if (stbi__zlength_extra[(z) as usize]) != 0 {
                len += (stbi__zreceive(a, stbi__zlength_extra[(z) as usize])) as i32;
            }
            z = (stbi__zhuffman_decode(a, (&mut (*a).z_distance) as *mut stbi__zhuffman))
                as i32;
            if z < 0 {
                return 0;
            }
            dist = (stbi__zdist_base[(z) as usize]) as i32;
            if (stbi__zdist_extra[(z) as usize]) != 0 {
                dist += (stbi__zreceive(a, stbi__zdist_extra[(z) as usize])) as i32;
            }
            if ((zout) as usize) - (((*a).zout_start) as usize) < dist as usize {
                return 0;
            }
            if (zout).offset((len) as isize) > (*a).zout_end {
                if stbi__zexpand(a, zout, len) == 0 {
                    return (0) as i32;
                }
                zout = (*a).zout;
            }
            p = ((zout).offset(-((dist) as isize))) as *mut u8;
            if dist == 1 {
                let v: u8 = *p;
                if (len) != 0 {
                    loop {
                        *c_runtime::postIncPtr(&mut zout) = (v) as i8;
                        if !((c_runtime::preDec(&mut len)) != 0) {
                            break;
                        }
                    }
                }
            } else {
                if (len) != 0 {
                    loop {
                        *c_runtime::postIncPtr(&mut zout) =
                            (*c_runtime::postIncPtr(&mut p)) as i8;
                        if !((c_runtime::preDec(&mut len)) != 0) {
                            break;
                        }
                    }
                }
            }
        }
    }

    return 0;
}

unsafe fn stbi__parse_uncompressed_block(a: *mut stbi__zbuf) -> i32 {
    let mut header: [u8; 4] = [0; 4];
    let mut len: i32 = 0;
    let mut nlen: i32 = 0;
    let mut k: i32 = 0;
    if ((*a).num_bits & 7) != 0 {
        stbi__zreceive(a, (*a).num_bits & 7);
    }
    k = (0) as i32;
    while (*a).num_bits > 0 {
        header[(c_runtime::postInc(&mut k)) as usize] = ((*a).code_buffer & ((255) as u32)) as u8;
        (*a).code_buffer >>= 8;
        (*a).num_bits -= (8) as i32;
    }
    if (*a).num_bits < 0 {
        return 0;
    }
    while k < 4 {
        header[(c_runtime::postInc(&mut k)) as usize] = (stbi__zget8(a)) as u8;
    }
    len = ((header[(1) as usize]) as i32) * 256 + ((header[(0) as usize]) as i32);
    nlen = ((header[(3) as usize]) as i32) * 256 + ((header[(2) as usize]) as i32);
    if nlen != (len ^ 0xffff) {
        return 0;
    }
    if ((*a).zbuffer).offset((len) as isize) > (*a).zbuffer_end {
        return 0;
    }
    if ((*a).zout).offset((len) as isize) > (*a).zout_end {
        if stbi__zexpand(a, (*a).zout, len) == 0 {
            return (0) as i32;
        }
    }
    c_runtime::memcpy(((*a).zout) as *mut u8, (*a).zbuffer, (len) as u64);
    (*a).zbuffer = (*a).zbuffer.offset((len) as isize);
    (*a).zout = (*a).zout.offset((len) as isize);
    return (1) as i32;
}

unsafe fn stbi__parse_zlib(a: *mut stbi__zbuf, parse_header: i32) -> i32 {
    let mut _final_: i32 = 0;
    let mut _type_: i32 = 0;
    if (parse_header) != 0 {
        if stbi__parse_zlib_header(a) == 0 {
            return (0) as i32;
        }
    }
    (*a).num_bits = (0) as i32;
    (*a).code_buffer = (0) as u32;
    loop {
        _final_ = (stbi__zreceive(a, 1)) as i32;
        _type_ = (stbi__zreceive(a, 2)) as i32;
        if _type_ == 0 {
            if stbi__parse_uncompressed_block(a) == 0 {
                return (0) as i32;
            }
        } else {
            if _type_ == 3 {
                return (0) as i32;
            } else {
                if _type_ == 1 {
                    if stbi__zbuild_huffman(
                        (&mut (*a).z_length) as *mut stbi__zhuffman,
                        (stbi__zdefault_length.as_mut_ptr()) as *mut u8,
                        288,
                    ) == 0
                    {
                        return (0) as i32;
                    }
                    if stbi__zbuild_huffman(
                        (&mut (*a).z_distance) as *mut stbi__zhuffman,
                        (stbi__zdefault_distance.as_mut_ptr()) as *mut u8,
                        32,
                    ) == 0
                    {
                        return (0) as i32;
                    }
                } else {
                    if stbi__compute_huffman_codes(a) == 0 {
                        return (0) as i32;
                    }
                }
                if stbi__parse_huffman_block(a) == 0 {
                    return (0) as i32;
                }
            }
        }
        if !(_final_ == 0) {
            break;
        }
    }

    return (1) as i32;
}

unsafe fn stbi__parse_zlib_header(a: *mut stbi__zbuf) -> i32 {
    let cmf: i32 = (stbi__zget8(a)) as i32;
    let cm: i32 = cmf & 15;
    let flg: i32 = (stbi__zget8(a)) as i32;
    if (stbi__zeof(a)) != 0 {
        return 0;
    }
    if (cmf * 256 + flg) % 31 != 0 {
        return 0;
    }
    if (flg & 32) != 0 {
        return 0;
    }
    if cm != 8 {
        return 0;
    }
    return (1) as i32;
}

unsafe fn stbi__zbuild_huffman(
    z: *mut stbi__zhuffman,
    sizelist: *mut u8,
    num: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut code: i32 = 0;
    let mut next_code: [i32; 16] = [0; 16];
    let mut sizes: [i32; 17] = [0; 17];
    c_runtime::memset(
        (sizes.as_mut_ptr()) as *mut u8,
        0,
        17 * std::mem::size_of::<i32>() as u64,
    );
    c_runtime::memset(
        ((*z).fast.as_mut_ptr()) as *mut u8,
        0,
        512 * std::mem::size_of::<u16>() as u64,
    );
    i = (0) as i32;
    while i < num {
        c_runtime::preInc(&mut sizes[(*sizelist.offset((i) as isize)) as usize]);
        c_runtime::preInc(&mut i);
    }
    sizes[(0) as usize] = (0) as i32;
    i = (1) as i32;
    while i < 16 {
        if sizes[(i) as usize] > (1 << i) {
            return 0;
        }
        c_runtime::preInc(&mut i);
    }
    code = (0) as i32;
    i = (1) as i32;
    while i < 16 {
        next_code[(i) as usize] = (code) as i32;
        (*z).firstcode[(i) as usize] = (code) as u16;
        (*z).firstsymbol[(i) as usize] = (k) as u16;
        code = (code + sizes[(i) as usize]) as i32;
        if (sizes[(i) as usize]) != 0 {
            if code - 1 >= (1 << i) {
                return 0;
            }
        }
        (*z).maxcode[(i) as usize] = (code << (16 - i)) as i32;
        code <<= 1;
        k += (sizes[(i) as usize]) as i32;
        c_runtime::preInc(&mut i);
    }
    (*z).maxcode[(16) as usize] = (0x10000) as i32;
    i = (0) as i32;
    while i < num {
        let s: i32 = (*sizelist.offset((i) as isize)) as i32;
        if (s) != 0 {
            let c: i32 = next_code[(s) as usize] - (((*z).firstcode[(s) as usize]) as i32)
                + (((*z).firstsymbol[(s) as usize]) as i32);
            let fastv: u16 = ((s << 9) | i) as u16;
            (*z).size[(c) as usize] = (s) as u8;
            (*z).value[(c) as usize] = (i) as u16;
            if s <= 9 {
                let mut j: i32 = stbi__bit_reverse(next_code[(s) as usize], s);
                while j < (1 << 9) {
                    (*z).fast[(j) as usize] = (fastv) as u16;
                    j += (1 << s) as i32;
                }
            }
            c_runtime::preInc(&mut next_code[(s) as usize]);
        }
        c_runtime::preInc(&mut i);
    }
    return (1) as i32;
}

unsafe fn stbi__zeof(z: *mut stbi__zbuf) -> i32 {
    return (if (*z).zbuffer >= (*z).zbuffer_end {
        1
    } else {
        0
    }) as i32;
}

unsafe fn stbi__zexpand(z: *mut stbi__zbuf, zout: *mut i8, n: i32) -> i32 {
    let mut q: *mut i8 = std::ptr::null_mut();
    let mut cur: u32 = 0;
    let mut limit: u32 = 0;
    let mut old_limit: u32 = 0;
    (*z).zout = zout;
    if (*z).z_expandable == 0 {
        return 0;
    }
    cur = (((*z).zout).offset(-(((*z).zout_start) as isize))) as u32;
    let hebron_tmp0 = (((*z).zout_end).offset(-(((*z).zout_start) as isize))) as u32;
    limit = hebron_tmp0;
    old_limit = hebron_tmp0;
    if 0xffffffff - cur < ((n) as u32) {
        return 0;
    }
    while cur + ((n) as u32) > limit {
        if limit > 0xffffffff / ((2) as u32) {
            return 0;
        }
        limit *= (2) as u32;
    }
    q = (c_runtime::realloc(((*z).zout_start) as *mut u8, (limit) as u64)) as *mut i8;

    if q == std::ptr::null_mut() {
        return 0;
    }
    (*z).zout_start = q;
    (*z).zout = (q).offset((cur) as isize);
    (*z).zout_end = (q).offset((limit) as isize);
    return (1) as i32;
}

unsafe fn stbi__zget8(z: *mut stbi__zbuf) -> u8 {
    return (if (stbi__zeof(z)) != 0 {
        0
    } else {
        (*c_runtime::postIncPtr(&mut (*z).zbuffer)) as i32
    }) as u8;
}

unsafe fn stbi__zhuffman_decode(a: *mut stbi__zbuf, z: *mut stbi__zhuffman) -> i32 {
    let mut b: i32 = 0;
    let mut s: i32 = 0;
    if (*a).num_bits < 16 {
        if (stbi__zeof(a)) != 0 {
            return (-1) as i32;
        }
        stbi__fill_bits(a);
    }
    b = ((*z).fast[((*a).code_buffer & (((1 << 9) - 1) as u32)) as usize]) as i32;
    if (b) != 0 {
        s = (b >> 9) as i32;
        (*a).code_buffer >>= s;
        (*a).num_bits -= (s) as i32;
        return (b & 511) as i32;
    }
    return (stbi__zhuffman_decode_slowpath(a, z)) as i32;
}

unsafe fn stbi__zhuffman_decode_slowpath(
    a: *mut stbi__zbuf,
    z: *mut stbi__zhuffman,
) -> i32 {
    let mut b: i32 = 0;
    let mut s: i32 = 0;
    let mut k: i32 = 0;
    k = stbi__bit_reverse(((*a).code_buffer) as i32, 16);
    s = (9 + 1) as i32;
    loop {
        if k < (*z).maxcode[(s) as usize] {
            break;
        }
        c_runtime::preInc(&mut s);
    }
    if s >= 16 {
        return (-1) as i32;
    }
    b = (k >> (16 - s)) - (((*z).firstcode[(s) as usize]) as i32)
        + (((*z).firstsymbol[(s) as usize]) as i32);
    if b >= 288 {
        return (-1) as i32;
    }
    if (((*z).size[(b) as usize]) as i32) != s {
        return (-1) as i32;
    }
    (*a).code_buffer >>= s;
    (*a).num_bits -= (s) as i32;
    return ((*z).value[(b) as usize]) as i32;
}

unsafe fn stbi__zreceive(z: *mut stbi__zbuf, n: i32) -> u32 {
    let mut k: u32 = 0;
    if (*z).num_bits < n {
        stbi__fill_bits(z);
    }
    k = (*z).code_buffer & (((1 << n) - 1) as u32);
    (*z).code_buffer >>= n;
    (*z).num_bits -= (n) as i32;
    return (k) as u32;
}

unsafe fn stbi_zlib_decode_buffer(
    obuffer: *mut i8,
    olen: i32,
    ibuffer: *mut i8,
    ilen: i32,
) -> i32 {
    let mut a: stbi__zbuf = stbi__zbuf::default();
    a.zbuffer = (ibuffer) as *mut u8;
    a.zbuffer_end = ((ibuffer) as *mut u8).offset((ilen) as isize);
    if (stbi__do_zlib((&mut a) as *mut stbi__zbuf, obuffer, olen, 0, 1)) != 0 {
        return ((a.zout).offset(-((a.zout_start) as isize))) as i32;
    } else {
        return (-1) as i32;
    }
}

unsafe fn stbi_zlib_decode_malloc(
    buffer: *mut i8,
    len: i32,
    outlen: *mut i32,
) -> *mut i8 {
    return stbi_zlib_decode_malloc_guesssize(buffer, len, 16384, outlen);
}

unsafe fn stbi_zlib_decode_malloc_guesssize(
    buffer: *mut i8,
    len: i32,
    initial_size: i32,
    outlen: *mut i32,
) -> *mut i8 {
    let mut a: stbi__zbuf = stbi__zbuf::default();
    let p: *mut i8 = (stbi__malloc((initial_size) as u64)) as *mut i8;
    if p == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }
    a.zbuffer = (buffer) as *mut u8;
    a.zbuffer_end = ((buffer) as *mut u8).offset((len) as isize);
    if (stbi__do_zlib((&mut a) as *mut stbi__zbuf, p, initial_size, 1, 1)) != 0 {
        if (outlen) != std::ptr::null_mut() {
            *outlen = ((a.zout).offset(-((a.zout_start) as isize))) as i32;
        }
        return a.zout_start;
    } else {
        c_runtime::free((a.zout_start) as *mut u8);
        return std::ptr::null_mut();
    }
}

unsafe fn stbi_zlib_decode_malloc_guesssize_headerflag(
    buffer: *mut i8,
    len: i32,
    initial_size: i32,
    outlen: *mut i32,
    parse_header: i32,
) -> *mut i8 {
    let mut a: stbi__zbuf = stbi__zbuf::default();
    let p: *mut i8 = (stbi__malloc((initial_size) as u64)) as *mut i8;
    if p == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }
    a.zbuffer = (buffer) as *mut u8;
    a.zbuffer_end = ((buffer) as *mut u8).offset((len) as isize);
    if (stbi__do_zlib(
        (&mut a) as *mut stbi__zbuf,
        p,
        initial_size,
        1,
        parse_header,
    )) != 0
    {
        if (outlen) != std::ptr::null_mut() {
            *outlen = ((a.zout).offset(-((a.zout_start) as isize))) as i32;
        }
        return a.zout_start;
    } else {
        c_runtime::free((a.zout_start) as *mut u8);
        return std::ptr::null_mut();
    }
}

unsafe fn stbi_zlib_decode_noheader_buffer(
    obuffer: *mut i8,
    olen: i32,
    ibuffer: *mut i8,
    ilen: i32,
) -> i32 {
    let mut a: stbi__zbuf = stbi__zbuf::default();
    a.zbuffer = (ibuffer) as *mut u8;
    a.zbuffer_end = ((ibuffer) as *mut u8).offset((ilen) as isize);
    if (stbi__do_zlib((&mut a) as *mut stbi__zbuf, obuffer, olen, 0, 0)) != 0 {
        return ((a.zout).offset(-((a.zout_start) as isize))) as i32;
    } else {
        return (-1) as i32;
    }
}

unsafe fn stbi_zlib_decode_noheader_malloc(
    buffer: *mut i8,
    len: i32,
    outlen: *mut i32,
) -> *mut i8 {
    let mut a: stbi__zbuf = stbi__zbuf::default();
    let p: *mut i8 = (stbi__malloc((16384) as u64)) as *mut i8;
    if p == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }
    a.zbuffer = (buffer) as *mut u8;
    a.zbuffer_end = ((buffer) as *mut u8).offset((len) as isize);
    if (stbi__do_zlib((&mut a) as *mut stbi__zbuf, p, 16384, 1, 0)) != 0 {
        if (outlen) != std::ptr::null_mut() {
            *outlen = ((a.zout).offset(-((a.zout_start) as isize))) as i32;
        }
        return a.zout_start;
    } else {
        c_runtime::free((a.zout_start) as *mut u8);
        return std::ptr::null_mut();
    }
}

pub static mut first_row_filter: [u8; 5] = [
    ((STBI__F_none) as u8),
    ((STBI__F_sub) as u8),
    ((STBI__F_none) as u8),
    ((STBI__F_avg_first) as u8),
    ((STBI__F_paeth_first) as u8),
];
pub static mut stbi__bmask: [u32; 17] = [
    ((0) as u32),
    ((1) as u32),
    ((3) as u32),
    ((7) as u32),
    ((15) as u32),
    ((31) as u32),
    ((63) as u32),
    ((127) as u32),
    ((255) as u32),
    ((511) as u32),
    ((1023) as u32),
    ((2047) as u32),
    ((4095) as u32),
    ((8191) as u32),
    ((16383) as u32),
    ((32767) as u32),
    ((65535) as u32),
];
pub static mut stbi__de_iphone_flag_global: i32 = 0;
pub static mut stbi__de_iphone_flag_local: i32 = 0;
pub static mut stbi__de_iphone_flag_set: i32 = 0;
pub static mut stbi__depth_scale_table: [u8; 9] = [
    ((0) as u8),
    ((0xff) as u8),
    ((0x55) as u8),
    ((0) as u8),
    ((0x11) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0) as u8),
    ((0x01) as u8),
];
pub static mut stbi__h2l_gamma_i: f32 = 1.0f32 / 2.2f32;
pub static mut stbi__h2l_scale_i: f32 = 1.0f32;
pub static mut stbi__jbias: [i32; 16] = [
    0, -1, -3, -7, -15, -31, -63, -127, -255, -511, -1023, -2047, -4095, -8191, -16383, -32767,
];
pub static mut stbi__jpeg_dezigzag: [u8; 79] = [
    ((0) as u8),
    ((1) as u8),
    ((8) as u8),
    ((16) as u8),
    ((9) as u8),
    ((2) as u8),
    ((3) as u8),
    ((10) as u8),
    ((17) as u8),
    ((24) as u8),
    ((32) as u8),
    ((25) as u8),
    ((18) as u8),
    ((11) as u8),
    ((4) as u8),
    ((5) as u8),
    ((12) as u8),
    ((19) as u8),
    ((26) as u8),
    ((33) as u8),
    ((40) as u8),
    ((48) as u8),
    ((41) as u8),
    ((34) as u8),
    ((27) as u8),
    ((20) as u8),
    ((13) as u8),
    ((6) as u8),
    ((7) as u8),
    ((14) as u8),
    ((21) as u8),
    ((28) as u8),
    ((35) as u8),
    ((42) as u8),
    ((49) as u8),
    ((56) as u8),
    ((57) as u8),
    ((50) as u8),
    ((43) as u8),
    ((36) as u8),
    ((29) as u8),
    ((22) as u8),
    ((15) as u8),
    ((23) as u8),
    ((30) as u8),
    ((37) as u8),
    ((44) as u8),
    ((51) as u8),
    ((58) as u8),
    ((59) as u8),
    ((52) as u8),
    ((45) as u8),
    ((38) as u8),
    ((31) as u8),
    ((39) as u8),
    ((46) as u8),
    ((53) as u8),
    ((60) as u8),
    ((61) as u8),
    ((54) as u8),
    ((47) as u8),
    ((55) as u8),
    ((62) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
    ((63) as u8),
];
pub static mut stbi__l2h_gamma: f32 = 2.2f32;
pub static mut stbi__l2h_scale: f32 = 1.0f32;
pub static mut stbi__unpremultiply_on_load_global: i32 = 0;
pub static mut stbi__unpremultiply_on_load_local: i32 = 0;
pub static mut stbi__unpremultiply_on_load_set: i32 = 0;
pub static mut stbi__vertically_flip_on_load_global: i32 = 0;
pub static mut stbi__vertically_flip_on_load_local: i32 = 0;
pub static mut stbi__vertically_flip_on_load_set: i32 = 0;
pub static mut stbi__zdefault_distance: [u8; 32] = [
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
    ((5) as u8),
];
pub static mut stbi__zdefault_length: [u8; 288] = [
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((9) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((7) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
    ((8) as u8),
];
pub static mut stbi__zdist_base: [i32; 32] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 0, 0,
];
pub static mut stbi__zdist_extra: [i32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
pub static mut stbi__zlength_base: [i32; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258, 0, 0,
];
pub static mut stbi__zlength_extra: [i32; 31] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0, 0, 0,
];
pub static mut stbi__compute_huffman_codes_length_dezigzag: [u8; 19] = [
    ((16) as u8),
    ((17) as u8),
    ((18) as u8),
    ((0) as u8),
    ((8) as u8),
    ((7) as u8),
    ((9) as u8),
    ((6) as u8),
    ((10) as u8),
    ((5) as u8),
    ((11) as u8),
    ((4) as u8),
    ((12) as u8),
    ((3) as u8),
    ((13) as u8),
    ((2) as u8),
    ((14) as u8),
    ((1) as u8),
    ((15) as u8),
];
pub static mut stbi__parse_png_file_invalid_chunk: [i8; 25] = [0; 25];
pub static mut stbi__process_frame_header_rgb: [u8; 3] = [((82) as u8), ((71) as u8), ((66) as u8)];
pub static mut stbi__process_marker_tag: [u8; 6] = [
    ((65) as u8),
    ((100) as u8),
    ((111) as u8),
    ((98) as u8),
    ((101) as u8),
    ((0) as u8),
];
pub static mut stbi__shiftsigned_mul_table: [u32; 9] = [
    ((0) as u32),
    ((0xff) as u32),
    ((0x55) as u32),
    ((0x49) as u32),
    ((0x11) as u32),
    ((0x21) as u32),
    ((0x41) as u32),
    ((0x81) as u32),
    ((0x01) as u32),
];
pub static mut stbi__shiftsigned_shift_table: [u32; 9] = [
    ((0) as u32),
    ((0) as u32),
    ((0) as u32),
    ((1) as u32),
    ((0) as u32),
    ((2) as u32),
    ((4) as u32),
    ((6) as u32),
    ((0) as u32),
];

#[derive(Debug, Copy, Clone)]
pub struct stbi__context {
    pub img_x: u32,
    pub img_y: u32,
    pub img_n: i32,
    pub img_out_n: i32,
    pub io: stbi_io_callbacks,
    pub io_user_data: *mut u8,
    pub read_from_callbacks: i32,
    pub buflen: i32,
    pub buffer_start: [u8; 128],
    pub callback_already_read: i32,
    pub img_buffer: *const u8,
    pub img_buffer_end: *const u8,
    pub img_buffer_original: *const u8,
    pub img_buffer_original_end: *const u8,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi__result_info {
    pub bits_per_channel: i32,
    pub num_channels: i32,
    pub channel_order: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbi_io_callbacks {
    pub read: *mut fn(arg0: *mut u8, arg1: *mut i8, arg2: i32) -> i32,
    pub skip: *mut fn(arg0: *mut u8, arg1: i32),
    pub eof: *mut fn(arg0: *mut u8) -> i32,
}

impl std::default::Default for stbi__context {
    fn default() -> Self {
        stbi__context {
            img_x: 0,
            img_y: 0,
            img_n: 0,
            img_out_n: 0,
            io: stbi_io_callbacks::default(),
            io_user_data: std::ptr::null_mut(),
            read_from_callbacks: 0,
            buflen: 0,
            buffer_start: [0; 128],
            callback_already_read: 0,
            img_buffer: std::ptr::null_mut(),
            img_buffer_end: std::ptr::null_mut(),
            img_buffer_original: std::ptr::null_mut(),
            img_buffer_original_end: std::ptr::null_mut(),
        }
    }
}

impl std::default::Default for stbi__result_info {
    fn default() -> Self {
        stbi__result_info {
            bits_per_channel: 0,
            num_channels: 0,
            channel_order: 0,
        }
    }
}

impl std::default::Default for stbi_io_callbacks {
    fn default() -> Self {
        stbi_io_callbacks {
            read: std::ptr::null_mut(),
            skip: std::ptr::null_mut(),
            eof: std::ptr::null_mut(),
        }
    }
}

unsafe fn stbi__addsizes_valid(a: i32, b: i32) -> i32 {
    if b < 0 {
        return (0) as i32;
    }
    return (if a <= 2147483647 - b { 1 } else { 0 }) as i32;
}

unsafe fn stbi__at_eof(s: *mut stbi__context) -> i32 {
    if ((*s).io.read) != std::ptr::null_mut() {
        if (*(*s).io.eof)((*s).io_user_data) == 0 {
            return (0) as i32;
        }
        if (*s).read_from_callbacks == 0 {
            return (1) as i32;
        }
    }
    return (if (*s).img_buffer >= (*s).img_buffer_end {
        1
    } else {
        0
    }) as i32;
}

unsafe fn stbi__bit_reverse(v: i32, bits: i32) -> i32 {
    return (stbi__bitreverse16(v) >> (16 - bits)) as i32;
}

unsafe fn stbi__bitcount(mut a: u32) -> i32 {
    a = (a & ((0x55555555) as u32)) + ((a >> 1) & ((0x55555555) as u32));
    a = (a & ((0x33333333) as u32)) + ((a >> 2) & ((0x33333333) as u32));
    a = (a + (a >> 4)) & ((0x0f0f0f0f) as u32);
    a = (a + (a >> 8)) as u32;
    a = (a + (a >> 16)) as u32;
    return (a & ((0xff) as u32)) as i32;
}

unsafe fn stbi__bitreverse16(mut n: i32) -> i32 {
    n = (((n & 0xAAAA) >> 1) | ((n & 0x5555) << 1)) as i32;
    n = (((n & 0xCCCC) >> 2) | ((n & 0x3333) << 2)) as i32;
    n = (((n & 0xF0F0) >> 4) | ((n & 0x0F0F) << 4)) as i32;
    n = (((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)) as i32;
    return (n) as i32;
}

unsafe fn stbi__blinn_8x8(x: u8, y: u8) -> u8 {
    let t: u32 = (((x) as i32) * ((y) as i32) + 128) as u32;
    return ((t + (t >> 8)) >> 8) as u8;
}

unsafe fn stbi__clamp(x: i32) -> u8 {
    if ((x) as u32) > ((255) as u32) {
        if x < 0 {
            return (0) as u8;
        }
        if x > 255 {
            return (255) as u8;
        }
    }
    return (x) as u8;
}

unsafe fn stbi__compute_y(r: i32, g: i32, b: i32) -> u8 {
    return (((r * 77) + (g * 150) + (29 * b)) >> 8) as u8;
}

unsafe fn stbi__compute_y_16(r: i32, g: i32, b: i32) -> u16 {
    return (((r * 77) + (g * 150) + (29 * b)) >> 8) as u16;
}

unsafe fn stbi__convert_16_to_8(
    orig: *mut u16,
    w: i32,
    h: i32,
    channels: i32,
) -> *mut u8 {
    let mut i: i32 = 0;
    let img_len: i32 = w * h * channels;
    let mut reduced: *mut u8 = std::ptr::null_mut();
    reduced = stbi__malloc((img_len) as u64);
    if reduced == std::ptr::null_mut() {
        return ptr::null_mut();
    }
    i = (0) as i32;
    while i < img_len {
        *reduced.offset((i) as isize) =
            ((((*orig.offset((i) as isize)) as i32) >> 8) & 0xFF) as u8;
        c_runtime::preInc(&mut i);
    }
    c_runtime::free((orig) as *mut u8);
    return reduced;
}

unsafe fn stbi__convert_8_to_16(
    orig: *mut u8,
    w: i32,
    h: i32,
    channels: i32,
) -> *mut u16 {
    let mut i: i32 = 0;
    let img_len: i32 = w * h * channels;
    let mut enlarged: *mut u16 = std::ptr::null_mut();
    enlarged = (stbi__malloc((img_len * 2) as u64)) as *mut u16;
    if enlarged == std::ptr::null_mut() {
        return ptr::null_mut();
    }
    i = (0) as i32;
    while i < img_len {
        *enlarged.offset((i) as isize) = ((((*orig.offset((i) as isize)) as i32) << 8)
            + ((*orig.offset((i) as isize)) as i32))
            as u16;
        c_runtime::preInc(&mut i);
    }
    c_runtime::free(orig);
    return enlarged;
}

unsafe fn stbi__convert_format(
    data: *mut u8,
    img_n: i32,
    req_comp: i32,
    x: u32,
    y: u32,
) -> *mut u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut good: *mut u8 = std::ptr::null_mut();
    if req_comp == img_n {
        return data;
    }

    good = stbi__malloc_mad3(req_comp, (x) as i32, (y) as i32, 0);
    if good == std::ptr::null_mut() {
        c_runtime::free(data);
        return ptr::null_mut();
    }
    j = (0) as i32;
    while j < ((y) as i32) {
        let mut src: *mut u8 = (data).offset((((j) as u32) * x * ((img_n) as u32)) as isize);
        let mut dest: *mut u8 = (good).offset((((j) as u32) * x * ((req_comp) as u32)) as isize);
        {
            if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u8;
                    *dest.offset((1) as isize) = (255) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp1 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp1;
                    *dest.offset((1) as isize) = hebron_tmp1;
                    *dest.offset((2) as isize) = hebron_tmp1;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp3 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp3;
                    *dest.offset((1) as isize) = hebron_tmp3;
                    *dest.offset((2) as isize) = hebron_tmp3;
                    *dest.offset((3) as isize) = (255) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp5 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp5;
                    *dest.offset((1) as isize) = hebron_tmp5;
                    *dest.offset((2) as isize) = hebron_tmp5;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp7 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp7;
                    *dest.offset((1) as isize) = hebron_tmp7;
                    *dest.offset((2) as isize) = hebron_tmp7;
                    *dest.offset((3) as isize) = (*src.offset((1) as isize)) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u8;
                    *dest.offset((1) as isize) = (*src.offset((1) as isize)) as u8;
                    *dest.offset((2) as isize) = (*src.offset((2) as isize)) as u8;
                    *dest.offset((3) as isize) = (255) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u8;
                    *dest.offset((1) as isize) = (255) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u8;
                    *dest.offset((1) as isize) = (*src.offset((3) as isize)) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u8;
                    *dest.offset((1) as isize) = (*src.offset((1) as isize)) as u8;
                    *dest.offset((2) as isize) = (*src.offset((2) as isize)) as u8;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else {
                c_runtime::free(data);
                c_runtime::free(good);
                return ptr::null_mut();
            }
        }
        c_runtime::preInc(&mut j);
    }
    c_runtime::free(data);
    return good;
}

unsafe fn stbi__convert_format16(
    data: *mut u16,
    img_n: i32,
    req_comp: i32,
    x: u32,
    y: u32,
) -> *mut u16 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut good: *mut u16 = std::ptr::null_mut();
    if req_comp == img_n {
        return data;
    }

    good = (stbi__malloc((((req_comp) as u32) * x * y * ((2) as u32)) as u64)) as *mut u16;
    if good == std::ptr::null_mut() {
        c_runtime::free((data) as *mut u8);
        return ptr::null_mut();
    }
    j = (0) as i32;
    while j < ((y) as i32) {
        let mut src: *mut u16 = (data).offset((((j) as u32) * x * ((img_n) as u32)) as isize);
        let mut dest: *mut u16 = (good).offset((((j) as u32) * x * ((req_comp) as u32)) as isize);
        {
            if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u16;
                    *dest.offset((1) as isize) = (0xffff) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp1 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp1;
                    *dest.offset((1) as isize) = hebron_tmp1;
                    *dest.offset((2) as isize) = hebron_tmp1;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((1) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp3 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp3;
                    *dest.offset((1) as isize) = hebron_tmp3;
                    *dest.offset((2) as isize) = hebron_tmp3;
                    *dest.offset((3) as isize) = (0xffff) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((1) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp5 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp5;
                    *dest.offset((1) as isize) = hebron_tmp5;
                    *dest.offset((2) as isize) = hebron_tmp5;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((2) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    let hebron_tmp7 = *src.offset((0) as isize);
                    *dest.offset((0) as isize) = hebron_tmp7;
                    *dest.offset((1) as isize) = hebron_tmp7;
                    *dest.offset((2) as isize) = hebron_tmp7;
                    *dest.offset((3) as isize) = (*src.offset((1) as isize)) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((2) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (4)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u16;
                    *dest.offset((1) as isize) = (*src.offset((1) as isize)) as u16;
                    *dest.offset((2) as isize) = (*src.offset((2) as isize)) as u16;
                    *dest.offset((3) as isize) = (0xffff) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((4) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y_16(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((3) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y_16(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u16;
                    *dest.offset((1) as isize) = (0xffff) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((3) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (1)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y_16(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((1) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (2)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (stbi__compute_y_16(
                        (*src.offset((0) as isize)) as i32,
                        (*src.offset((1) as isize)) as i32,
                        (*src.offset((2) as isize)) as i32,
                    )) as u16;
                    *dest.offset((1) as isize) = (*src.offset((3) as isize)) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((2) as isize);
                }
            } else if ((img_n) * 8 + (req_comp)) == ((4) * 8 + (3)) {
                i = (x - ((1) as u32)) as i32;
                while i >= 0 {
                    *dest.offset((0) as isize) = (*src.offset((0) as isize)) as u16;
                    *dest.offset((1) as isize) = (*src.offset((1) as isize)) as u16;
                    *dest.offset((2) as isize) = (*src.offset((2) as isize)) as u16;
                    c_runtime::preDec(&mut i);
                    src = src.offset((4) as isize);
                    dest = dest.offset((3) as isize);
                }
            } else {
                c_runtime::free((data) as *mut u8);
                c_runtime::free((good) as *mut u8);
                return ptr::null_mut();
            }
        }
        c_runtime::preInc(&mut j);
    }
    c_runtime::free((data) as *mut u8);
    return good;
}

unsafe fn stbi__get16be(s: *mut stbi__context) -> i32 {
    let z: i32 = (stbi__get8(s)) as i32;
    return (z << 8) + ((stbi__get8(s)) as i32);
}

unsafe fn stbi__get16le(s: *mut stbi__context) -> i32 {
    let z: i32 = (stbi__get8(s)) as i32;
    return z + (((stbi__get8(s)) as i32) << 8);
}

unsafe fn stbi__get32be(s: *mut stbi__context) -> u32 {
    let z: u32 = (stbi__get16be(s)) as u32;
    return (z << 16) + ((stbi__get16be(s)) as u32);
}

unsafe fn stbi__get32le(s: *mut stbi__context) -> u32 {
    let mut z: u32 = (stbi__get16le(s)) as u32;
    z += ((stbi__get16le(s)) as u32) << 16;
    return (z) as u32;
}

unsafe fn stbi__get8(s: *mut stbi__context) -> u8 {
    if (*s).img_buffer < (*s).img_buffer_end {
        return (*c_runtime::postIncConstPtr(&mut (*s).img_buffer)) as u8;
    }
    if ((*s).read_from_callbacks) != 0 {
        stbi__refill_buffer(s);
        return (*c_runtime::postIncConstPtr(&mut (*s).img_buffer)) as u8;
    }
    return (0) as u8;
}

unsafe fn stbi__getn(s: *mut stbi__context, buffer: *mut u8, n: i32) -> i32 {
    if ((*s).io.read) != std::ptr::null_mut() {
        let blen: i32 = (((*s).img_buffer_end).offset(-(((*s).img_buffer) as isize))) as i32;
        if blen < n {
            let mut res: i32 = 0;
            let mut count: i32 = 0;
            c_runtime::memcpy(buffer, (*s).img_buffer, (blen) as u64);
            count = ((*(*s).io.read)(
                (*s).io_user_data,
                ((buffer) as *mut i8).offset((blen) as isize),
                n - blen,
            )) as i32;
            res = (if count == (n - blen) { 1 } else { 0 }) as i32;
            (*s).img_buffer = (*s).img_buffer_end;
            return (res) as i32;
        }
    }
    if ((*s).img_buffer).offset((n) as isize) <= (*s).img_buffer_end {
        c_runtime::memcpy(buffer, (*s).img_buffer, (n) as u64);
        (*s).img_buffer = (*s).img_buffer.offset((n) as isize);
        return (1) as i32;
    } else {
        return (0) as i32;
    }
}

unsafe fn stbi__high_bit(mut z: u32) -> i32 {
    let mut n: i32 = 0;
    if z == ((0) as u32) {
        return (-1) as i32;
    }
    if z >= ((0x10000) as u32) {
        n += (16) as i32;
        z >>= 16;
    }
    if z >= ((0x00100) as u32) {
        n += (8) as i32;
        z >>= 8;
    }
    if z >= ((0x00010) as u32) {
        n += (4) as i32;
        z >>= 4;
    }
    if z >= ((0x00004) as u32) {
        n += (2) as i32;
        z >>= 2;
    }
    if z >= ((0x00002) as u32) {
        n += (1) as i32;
    }
    return (n) as i32;
}

unsafe fn stbi__info_main(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
) -> i32 {
    if (stbi__png_info(s, x, y, comp)) != 0 {
        return (1) as i32;
    }
    return 0;
}

unsafe fn stbi__is_16_main(s: *mut stbi__context) -> i32 {
    if (stbi__png_is16(s)) != 0 {
        return (1) as i32;
    }
    return (0) as i32;
}

unsafe fn stbi__ldr_to_hdr(data: *mut u8, x: i32, y: i32, comp: i32) -> *mut f32 {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut output: *mut f32 = std::ptr::null_mut();
    if data == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }
    output = (stbi__malloc_mad4(x, y, comp, (std::mem::size_of::<f32>() as u64) as i32, 0))
        as *mut f32;
    if output == std::ptr::null_mut() {
        c_runtime::free(data);
        return ptr::null_mut();
    }
    if (comp & 1) != 0 {
        n = (comp) as i32;
    } else {
        n = (comp - 1) as i32;
    }
    i = (0) as i32;
    while i < x * y {
        k = (0) as i32;
        while k < n {
            *output.offset((i * comp + k) as isize) = (c_runtime::pow(
                ((((*data.offset((i * comp + k) as isize)) as i32) as f32) / 255.0f32) as f32,
                (stbi__l2h_gamma) as f32,
            ) * ((stbi__l2h_scale) as f32))
                as f32;
            c_runtime::preInc(&mut k);
        }
        c_runtime::preInc(&mut i);
    }
    if n < comp {
        i = (0) as i32;
        while i < x * y {
            *output.offset((i * comp + n) as isize) =
                (((*data.offset((i * comp + n) as isize)) as i32) as f32) / 255.0f32;
            c_runtime::preInc(&mut i);
        }
    }
    c_runtime::free(data);
    return output;
}

unsafe fn stbi__load_and_postprocess_16bit(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut u16 {
    let mut ri: stbi__result_info = stbi__result_info::default();
    let mut result: *mut u8 = stbi__load_main(
        s,
        x,
        y,
        comp,
        req_comp,
        (&mut ri) as *mut stbi__result_info,
        16,
    );
    if result == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }

    if ri.bits_per_channel != 16 {
        result = stbi__convert_8_to_16(
            (result) as *mut u8,
            *x,
            *y,
            if req_comp == 0 { *comp } else { req_comp },
        ) as *mut u8;
        ri.bits_per_channel = (16) as i32;
    }
    if (if (stbi__vertically_flip_on_load_set) != 0 {
        stbi__vertically_flip_on_load_local
    } else {
        stbi__vertically_flip_on_load_global
    }) != 0
    {
        let channels: i32 = if (req_comp) != 0 { req_comp } else { *comp };
        stbi__vertical_flip(
            result,
            *x,
            *y,
            (((channels) as u64) * std::mem::size_of::<u16>() as u64) as i32,
        );
    }
    return (result) as *mut u16;
}

unsafe fn stbi__load_and_postprocess_8bit(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut u8 {
    let mut ri: stbi__result_info = stbi__result_info::default();
    let mut result: *mut u8 = stbi__load_main(
        s,
        x,
        y,
        comp,
        req_comp,
        (&mut ri) as *mut stbi__result_info,
        8,
    );
    if result == std::ptr::null_mut() {
        return std::ptr::null_mut();
    }

    if ri.bits_per_channel != 8 {
        result = stbi__convert_16_to_8(
            ((result) as *mut u16) as *mut u16,
            *x,
            *y,
            if req_comp == 0 { *comp } else { req_comp },
        );
        ri.bits_per_channel = (8) as i32;
    }
    if (if (stbi__vertically_flip_on_load_set) != 0 {
        stbi__vertically_flip_on_load_local
    } else {
        stbi__vertically_flip_on_load_global
    }) != 0
    {
        let channels: i32 = if (req_comp) != 0 { req_comp } else { *comp };
        stbi__vertical_flip(
            result,
            *x,
            *y,
            (((channels) as u64) * std::mem::size_of::<u8>() as u64) as i32,
        );
    }
    return result;
}

unsafe fn stbi__load_main(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
    ri: *mut stbi__result_info,
    bpc: i32,
) -> *mut u8 {
    c_runtime::memset(
        (ri) as *mut u8,
        0,
        std::mem::size_of::<stbi__result_info>() as u64,
    );
    (*ri).bits_per_channel = (8) as i32;
    (*ri).channel_order = (STBI_ORDER_RGB) as i32;
    (*ri).num_channels = (0) as i32;
    if (stbi__png_test(s)) != 0 {
        return stbi__png_load(s, x, y, comp, req_comp, ri);
    }
    return ptr::null_mut();
}

unsafe fn stbi__loadf_main(
    s: *mut stbi__context,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut f32 {
    let mut data: *mut u8 = std::ptr::null_mut();
    data = stbi__load_and_postprocess_8bit(s, x, y, comp, req_comp);
    if (data) != std::ptr::null_mut() {
        return stbi__ldr_to_hdr(data, *x, *y, if (req_comp) != 0 { req_comp } else { *comp });
    }
    return ptr::null_mut();
}

unsafe fn stbi__mad2sizes_valid(a: i32, b: i32, add: i32) -> i32 {
    return (if (stbi__mul2sizes_valid(a, b)) != 0 && (stbi__addsizes_valid(a * b, add)) != 0 {
        1
    } else {
        0
    }) as i32;
}

unsafe fn stbi__mad3sizes_valid(a: i32, b: i32, c: i32, add: i32) -> i32 {
    return (if (stbi__mul2sizes_valid(a, b)) != 0
        && (stbi__mul2sizes_valid(a * b, c)) != 0
        && (stbi__addsizes_valid(a * b * c, add)) != 0
    {
        1
    } else {
        0
    }) as i32;
}

unsafe fn stbi__mad4sizes_valid(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    add: i32,
) -> i32 {
    return (if (stbi__mul2sizes_valid(a, b)) != 0
        && (stbi__mul2sizes_valid(a * b, c)) != 0
        && (stbi__mul2sizes_valid(a * b * c, d)) != 0
        && (stbi__addsizes_valid(a * b * c * d, add)) != 0
    {
        1
    } else {
        0
    }) as i32;
}

unsafe fn stbi__malloc(size: u64) -> *mut u8 {
    return c_runtime::malloc(size);
}

unsafe fn stbi__malloc_mad2(a: i32, b: i32, add: i32) -> *mut u8 {
    if stbi__mad2sizes_valid(a, b, add) == 0 {
        return std::ptr::null_mut();
    }
    return stbi__malloc((a * b + add) as u64);
}

unsafe fn stbi__malloc_mad3(a: i32, b: i32, c: i32, add: i32) -> *mut u8 {
    if stbi__mad3sizes_valid(a, b, c, add) == 0 {
        return std::ptr::null_mut();
    }
    return stbi__malloc((a * b * c + add) as u64);
}

unsafe fn stbi__malloc_mad4(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    add: i32,
) -> *mut u8 {
    if stbi__mad4sizes_valid(a, b, c, d, add) == 0 {
        return std::ptr::null_mut();
    }
    return stbi__malloc((a * b * c * d + add) as u64);
}

unsafe fn stbi__mul2sizes_valid(a: i32, b: i32) -> i32 {
    if a < 0 || b < 0 {
        return (0) as i32;
    }
    if b == 0 {
        return (1) as i32;
    }
    return (if a <= 2147483647 / b { 1 } else { 0 }) as i32;
}

unsafe fn stbi__paeth(a: i32, b: i32, c: i32) -> i32 {
    let p: i32 = a + b - c;
    let pa: i32 = c_runtime::abs(p - a);
    let pb: i32 = c_runtime::abs(p - b);
    let pc: i32 = c_runtime::abs(p - c);
    if pa <= pb && pa <= pc {
        return (a) as i32;
    }
    if pb <= pc {
        return (b) as i32;
    }
    return (c) as i32;
}

unsafe fn stbi__refill_buffer(s: *mut stbi__context) {
    let n: i32 = (*(*s).io.read)(
        (*s).io_user_data,
        (((*s).buffer_start.as_mut_ptr()) as *mut i8) as *mut i8,
        (*s).buflen,
    );
    (*s).callback_already_read +=
        (((*s).img_buffer).offset(-(((*s).img_buffer_original) as isize))) as i32;
    if n == 0 {
        (*s).read_from_callbacks = (0) as i32;
        (*s).img_buffer = (*s).buffer_start.as_mut_ptr();
        (*s).img_buffer_end = ((*s).buffer_start.as_mut_ptr()).offset((1) as isize);
        (*s).buffer_start[0] = 0;
    } else {
        (*s).img_buffer = (*s).buffer_start.as_mut_ptr();
        (*s).img_buffer_end = ((*s).buffer_start.as_mut_ptr()).offset((n) as isize);
    }
}

unsafe fn stbi__rewind(s: *mut stbi__context) {
    (*s).img_buffer = (*s).img_buffer_original;
    (*s).img_buffer_end = (*s).img_buffer_original_end;
}

unsafe fn stbi__shiftsigned(mut v: u32, shift: i32, bits: i32) -> i32 {
    if shift < 0 {
        v <<= -shift;
    } else {
        v >>= shift;
    }

    v >>= 8 - bits;

    return (((v * stbi__shiftsigned_mul_table[(bits) as usize]) as i32)
        >> stbi__shiftsigned_shift_table[(bits) as usize]) as i32;
}

unsafe fn stbi__skip(s: *mut stbi__context, n: i32) {
    if n == 0 {
        return;
    }
    if n < 0 {
        (*s).img_buffer = (*s).img_buffer_end;
        return;
    }
    if ((*s).io.read) != std::ptr::null_mut() {
        let blen: i32 = (((*s).img_buffer_end).offset(-(((*s).img_buffer) as isize))) as i32;
        if blen < n {
            (*s).img_buffer = (*s).img_buffer_end;
            (*(*s).io.skip)((*s).io_user_data, n - blen);
            return;
        }
    }
    (*s).img_buffer = (*s).img_buffer.offset((n) as isize);
}

unsafe fn stbi__start_callbacks(
    s: *mut stbi__context,
    c: *mut stbi_io_callbacks,
    user: *mut u8,
) {
    (*s).io = (*c) as stbi_io_callbacks;
    (*s).io_user_data = user;
    (*s).buflen = (128 * std::mem::size_of::<u8>() as u64) as i32;
    (*s).read_from_callbacks = (1) as i32;
    (*s).callback_already_read = (0) as i32;
    let hebron_tmp0 = (*s).buffer_start.as_mut_ptr();
    (*s).img_buffer = hebron_tmp0;
    (*s).img_buffer_original = hebron_tmp0;
    stbi__refill_buffer(s);
    (*s).img_buffer_original_end = (*s).img_buffer_end;
}

unsafe fn stbi__start_mem(s: *mut stbi__context, buffer: *const u8, len: i32) {
    (*s).io.read = std::ptr::null_mut();
    (*s).read_from_callbacks = (0) as i32;
    (*s).callback_already_read = (0) as i32;
    let hebron_tmp0 = buffer;
    (*s).img_buffer = hebron_tmp0;
    (*s).img_buffer_original = hebron_tmp0;
    let hebron_tmp1 = (buffer).offset((len) as isize);
    (*s).img_buffer_end = hebron_tmp1;
    (*s).img_buffer_original_end = hebron_tmp1;
}

unsafe fn stbi__unpremultiply_on_load_thread(flag_true_if_should_unpremultiply: i32) {
    stbi__unpremultiply_on_load_local = (flag_true_if_should_unpremultiply) as i32;
    stbi__unpremultiply_on_load_set = (1) as i32;
}

unsafe fn stbi__vertical_flip(
    image: *mut u8,
    w: i32,
    h: i32,
    bytes_per_pixel: i32,
) {
    let mut row: i32 = 0;
    let bytes_per_row: u64 = ((w) as u64) * ((bytes_per_pixel) as u64);
    let mut temp: [u8; 2048] = [0; 2048];
    let bytes: *mut u8 = image;
    row = (0) as i32;
    while row < (h >> 1) {
        let mut row0: *mut u8 = (bytes).offset((((row) as u64) * bytes_per_row) as isize);
        let mut row1: *mut u8 = (bytes).offset((((h - row - 1) as u64) * bytes_per_row) as isize);
        let mut bytes_left: u64 = bytes_per_row;
        while (bytes_left) != 0 {
            let bytes_copy: u64 = if bytes_left < 2048 * std::mem::size_of::<u8>() as u64 {
                bytes_left
            } else {
                2048 * std::mem::size_of::<u8>() as u64
            };
            c_runtime::memcpy((temp.as_mut_ptr()) as *mut u8, row0, bytes_copy);
            c_runtime::memcpy(row0, row1, bytes_copy);
            c_runtime::memcpy(row1, (temp.as_mut_ptr()) as *mut u8, bytes_copy);
            row0 = row0.offset((bytes_copy) as isize);
            row1 = row1.offset((bytes_copy) as isize);
            bytes_left -= (bytes_copy) as u64;
        }
        c_runtime::postInc(&mut row);
    }
}

unsafe fn stbi__vertical_flip_slices(
    image: *mut u8,
    w: i32,
    h: i32,
    z: i32,
    bytes_per_pixel: i32,
) {
    let mut slice: i32 = 0;
    let slice_size: i32 = w * h * bytes_per_pixel;
    let mut bytes: *mut u8 = image;
    slice = (0) as i32;
    while slice < z {
        stbi__vertical_flip(bytes, w, h, bytes_per_pixel);
        bytes = bytes.offset((slice_size) as isize);
        c_runtime::preInc(&mut slice);
    }
}

unsafe fn stbi_convert_iphone_png_to_rgb(flag_true_if_should_convert: i32) {
    stbi__de_iphone_flag_global = (flag_true_if_should_convert) as i32;
}

unsafe fn stbi_convert_iphone_png_to_rgb_thread(flag_true_if_should_convert: i32) {
    stbi__de_iphone_flag_local = (flag_true_if_should_convert) as i32;
    stbi__de_iphone_flag_set = (1) as i32;
}

unsafe fn stbi_hdr_to_ldr_gamma(gamma: f32) {
    stbi__h2l_gamma_i = ((1) as f32) / gamma;
}

unsafe fn stbi_hdr_to_ldr_scale(scale: f32) {
    stbi__h2l_scale_i = ((1) as f32) / scale;
}

unsafe fn stbi_image_free(retval_from_stbi_load: *mut u8) {
    c_runtime::free(retval_from_stbi_load);
}

unsafe fn stbi_info_from_callbacks(
    c: *mut stbi_io_callbacks,
    user: *mut u8,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
) -> i32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_callbacks(
        (&mut s) as *mut stbi__context,
        (c) as *mut stbi_io_callbacks,
        user,
    );
    return (stbi__info_main((&mut s) as *mut stbi__context, x, y, comp)) as i32;
}

unsafe fn stbi_info_from_memory(
    buffer: *const u8,
    len: i32,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
) -> i32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_mem((&mut s) as *mut stbi__context, buffer, len);
    return (stbi__info_main((&mut s) as *mut stbi__context, x, y, comp)) as i32;
}

unsafe fn stbi_is_16_bit_from_callbacks(c: *mut stbi_io_callbacks, user: *mut u8) -> i32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_callbacks(
        (&mut s) as *mut stbi__context,
        (c) as *mut stbi_io_callbacks,
        user,
    );
    return (stbi__is_16_main((&mut s) as *mut stbi__context)) as i32;
}

unsafe fn stbi_is_16_bit_from_memory(buffer: *const u8, len: i32) -> i32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_mem((&mut s) as *mut stbi__context, buffer, len);
    return (stbi__is_16_main((&mut s) as *mut stbi__context)) as i32;
}

unsafe fn stbi_is_hdr_from_callbacks(clbk: *mut stbi_io_callbacks, user: *mut u8) -> i32 {
    return (0) as i32;
}

unsafe fn stbi_is_hdr_from_memory(buffer: *const u8, len: i32) -> i32 {
    return (0) as i32;
}

unsafe fn stbi_ldr_to_hdr_gamma(gamma: f32) {
    stbi__l2h_gamma = (gamma) as f32;
}

unsafe fn stbi_ldr_to_hdr_scale(scale: f32) {
    stbi__l2h_scale = (scale) as f32;
}

unsafe fn stbi_load_16_from_callbacks(
    clbk: *mut stbi_io_callbacks,
    user: *mut u8,
    x: *mut i32,
    y: *mut i32,
    channels_in_file: *mut i32,
    desired_channels: i32,
) -> *mut u16 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_callbacks(
        (&mut s) as *mut stbi__context,
        (clbk) as *mut stbi_io_callbacks,
        user,
    );
    return stbi__load_and_postprocess_16bit(
        (&mut s) as *mut stbi__context,
        x,
        y,
        channels_in_file,
        desired_channels,
    );
}

unsafe fn stbi_load_16_from_memory(
    buffer: *const u8,
    len: i32,
    x: *mut i32,
    y: *mut i32,
    channels_in_file: *mut i32,
    desired_channels: i32,
) -> *mut u16 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_mem((&mut s) as *mut stbi__context, buffer, len);
    return stbi__load_and_postprocess_16bit(
        (&mut s) as *mut stbi__context,
        x,
        y,
        channels_in_file,
        desired_channels,
    );
}

unsafe fn stbi_load_from_callbacks(
    clbk: *mut stbi_io_callbacks,
    user: *mut u8,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut u8 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_callbacks(
        (&mut s) as *mut stbi__context,
        (clbk) as *mut stbi_io_callbacks,
        user,
    );
    return stbi__load_and_postprocess_8bit((&mut s) as *mut stbi__context, x, y, comp, req_comp);
}

pub unsafe fn stbi_load_from_memory(
    buffer: *const u8,
    len: i32,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut u8 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_mem((&mut s) as *mut stbi__context, buffer, len);
    return stbi__load_and_postprocess_8bit((&mut s) as *mut stbi__context, x, y, comp, req_comp);
}

unsafe fn stbi_loadf_from_callbacks(
    clbk: *mut stbi_io_callbacks,
    user: *mut u8,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut f32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_callbacks(
        (&mut s) as *mut stbi__context,
        (clbk) as *mut stbi_io_callbacks,
        user,
    );
    return stbi__loadf_main((&mut s) as *mut stbi__context, x, y, comp, req_comp);
}

unsafe fn stbi_loadf_from_memory(
    buffer: *const u8,
    len: i32,
    x: *mut i32,
    y: *mut i32,
    comp: *mut i32,
    req_comp: i32,
) -> *mut f32 {
    let mut s: stbi__context = stbi__context::default();
    stbi__start_mem((&mut s) as *mut stbi__context, buffer, len);
    return stbi__loadf_main((&mut s) as *mut stbi__context, x, y, comp, req_comp);
}

unsafe fn stbi_set_flip_vertically_on_load(flag_true_if_should_flip: i32) {
    stbi__vertically_flip_on_load_global = (flag_true_if_should_flip) as i32;
}

unsafe fn stbi_set_flip_vertically_on_load_thread(flag_true_if_should_flip: i32) {
    stbi__vertically_flip_on_load_local = (flag_true_if_should_flip) as i32;
    stbi__vertically_flip_on_load_set = (1) as i32;
}

unsafe fn stbi_set_unpremultiply_on_load(flag_true_if_should_unpremultiply: i32) {
    stbi__unpremultiply_on_load_global = (flag_true_if_should_unpremultiply) as i32;
}

mod c_runtime {
    use std;
    
    

    pub trait One {
        fn one() -> Self;
    }

    impl One for i32 {
        fn one() -> Self {
            1
        }
    }

    impl One for u32 {
        fn one() -> Self {
            1
        }
    }

    pub unsafe fn postInc<T: std::ops::AddAssign + One + Copy>(a: *mut T) -> T {
        let result: T = *a;
        *a += One::one();
        return result;
    }

    pub unsafe fn preInc<T: std::ops::AddAssign + One + Copy>(a: *mut T) -> T {
        *a += One::one();
        return *a;
    }

    pub unsafe fn postDec<T: std::ops::SubAssign + One + Copy>(a: *mut T) -> T {
        let result: T = *a;
        *a -= One::one();
        return result;
    }

    pub unsafe fn preDec<T: std::ops::SubAssign + One + Copy>(a: *mut T) -> T {
        *a -= One::one();
        return *a;
    }

    pub unsafe fn preIncPtr<T>(a: *mut *mut T) -> *mut T {
        *a = (*a).offset(1);
        return *a;
    }

    pub unsafe fn preDecPtr<T>(a: *mut *mut T) -> *mut T {
        *a = (*a).offset(-1);
        return *a;
    }

    pub unsafe fn postIncPtr<T>(a: *mut *mut T) -> *mut T {
        let result: *mut T = *a;
        *a = (*a).offset(1);
        return result;
    }

    pub unsafe fn postDecPtr<T>(a: *mut *mut T) -> *mut T {
        let result: *mut T = *a;
        *a = (*a).offset(-1);
        return result;
    }

    pub unsafe fn preIncConstPtr<T>(a: *mut *const T) -> *const T {
        *a = (*a).offset(1);
        return *a;
    }

    pub unsafe fn preDecConstPtr<T>(a: *mut *const T) -> *const T {
        *a = (*a).offset(-1);
        return *a;
    }

    pub unsafe fn postIncConstPtr<T>(a: *mut *const T) -> *const T {
        let result: *const T = *a;
        *a = (*a).offset(1);
        return result;
    }

    pub unsafe fn postDecConstPtr<T>(a: *mut *const T) -> *const T {
        let result: *const T = *a;
        *a = (*a).offset(-1);
        return result;
    }

    pub unsafe fn memcpy(src: *mut u8, dest: *const u8, count: u64) {
        std::ptr::copy_nonoverlapping(dest, src, count as usize);
    }

    pub unsafe fn memset(src: *mut u8, value: i32, count: u64) {
        std::ptr::write_bytes(src, value as u8, count as usize);
    }

    pub unsafe fn malloc(count: u64) -> *mut u8 {
        let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

        return std::alloc::alloc(layout);
    }

    pub unsafe fn realloc<T>(data: *mut T, count: u64) -> *mut u8 {
        if data == std::ptr::null_mut() {
            return malloc(count);
        }

        let layout = std::alloc::Layout::from_size_align(count as usize, 1).expect("Bad layout");

        return std::alloc::realloc(data as *mut u8, layout, count as usize);
    }

    pub unsafe fn free<T>(data: *mut T) {
        let layout = std::alloc::Layout::from_size_align(1, 1).expect("Bad layout");

        std::alloc::dealloc(data as *mut u8, layout);
    }

    pub fn _lrotl(x: u32, y: i32) -> u32 {
        return (x << y) | (x >> (32 - y));
    }

    pub fn abs(x: i32) -> i32 {
        return i32::abs(x);
    }

    pub fn pow(x: f32, p: f32) -> f32 {
        return x.powf(p);
    }
}
