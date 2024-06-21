pub unsafe fn image_resize_linear(
    mut dst: *mut u32,
    dst_width: u32,
    dst_height: u32,
    src: *const u32,
    src_width: u32,
    src_height: u32,
    src_stride: u32,
) {
    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;
    let step_x: i32 = (x_ratio * 1024.0f32) as i32;
    let step_y: i32 = (y_ratio * 1024.0f32) as i32;
    let mut fixed_y: i32 = 0 as i32;
    let mut i: u32 = 0 as i32 as u32;
    while i < dst_height {
        let y: i32 = ((fixed_y >> 10 as i32) as u32).wrapping_mul(src_stride) as i32;
        let mut fixed_x: i32 = 0 as i32;
        let mut j: u32 = 0 as i32 as u32;
        while j < dst_width {
            let x: i32 = fixed_x >> 10 as i32;
            let index: i32 = y + x;
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = *src.offset(index as isize);
            fixed_x += step_x;
            j = j.wrapping_add(1);
        }
        fixed_y += step_y;
        i = i.wrapping_add(1);
    }
}

unsafe fn image_resize_linear_stride(
    mut dst: *mut u32,
    dst_width: u32,
    dst_height: u32,
    src: *const u32,
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    stride: u32,
) {
    let x_ratio: f32 = src_width as f32 / dst_width as f32;
    let y_ratio: f32 = src_height as f32 / dst_height as f32;
    let step_x: i32 = (x_ratio * 1024.0f32) as i32;
    let step_y: i32 = (y_ratio * 1024.0f32) as i32;
    let stride_step: i32 = stride.wrapping_sub(dst_width) as i32;
    let mut fixed_y: i32 = 0 as i32;
    let mut i: u32 = 0 as i32 as u32;
    while i < dst_height {
        let y: i32 = ((fixed_y >> 10 as i32) as u32).wrapping_mul(src_stride) as i32;
        let mut fixed_x: i32 = 0 as i32;
        let mut j: u32 = 0 as i32 as u32;
        while j < dst_width {
            let x: i32 = fixed_x >> 10 as i32;
            let index: i32 = y + x;
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = *src.offset(index as isize);
            fixed_x += step_x;
            j = j.wrapping_add(1);
        }
        dst = dst.offset(stride_step as isize);
        fixed_y += step_y;
        i = i.wrapping_add(1);
    }
}

pub unsafe fn image_resize_linear_aspect_fill(
    dst: *mut u32,
    dst_width: u32,
    dst_height: u32,
    src: *const u32,
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    bg_clear: u32,
) {
    let mut i: u32 = 0 as i32 as u32;
    while i < dst_width.wrapping_mul(dst_height) {
        *dst.offset(i as isize) = bg_clear;
        i = i.wrapping_add(1);
    }
    let buffer_aspect: f32 = src_width as f32 / src_height as f32;
    let win_aspect: f32 = dst_width as f32 / dst_height as f32;
    if buffer_aspect > win_aspect {
        let new_height: u32 = (dst_width as f32 / buffer_aspect) as u32;
        let offset: i32 = new_height
            .wrapping_sub(dst_height)
            .wrapping_div(-(2 as i32) as u32) as i32;
        image_resize_linear(
            dst.offset((offset as u32).wrapping_mul(dst_width) as isize),
            dst_width,
            new_height,
            src,
            src_width,
            src_height,
            src_stride,
        );
    } else {
        let new_width: u32 = (dst_height as f32 * buffer_aspect) as u32;
        let offset_0: i32 = new_width
            .wrapping_sub(dst_width)
            .wrapping_div(-(2 as i32) as u32) as i32;
        image_resize_linear_stride(
            dst.offset(offset_0 as isize),
            dst_height,
            dst_width,
            src,
            src_width,
            src_height,
            src_stride,
            new_width,
        );
    };
}

#[no_mangle]
pub unsafe fn image_center(
    mut dst: *mut u32,
    dst_width: u32,
    dst_height: u32,
    mut src: *const u32,
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    bg_clear: u32,
) {
    let mut i: u32 = 0 as i32 as u32;
    while i < dst_width.wrapping_mul(dst_height) {
        *dst.offset(i as isize) = bg_clear;
        i = i.wrapping_add(1);
    }
    if src_height > dst_height {
        let y_offset: i32 = src_height
            .wrapping_sub(dst_height)
            .wrapping_div(2 as i32 as u32) as i32;
        let mut new_height: u32 = src_height.wrapping_sub(y_offset as u32);
        src = src.offset((y_offset as u32).wrapping_mul(src_stride) as isize);
        if new_height > dst_height {
            new_height = dst_height;
        }
        if src_width > dst_width {
            let x_offset: i32 = src_width
                .wrapping_sub(dst_width)
                .wrapping_div(2 as i32 as u32) as i32;
            src = src.offset(x_offset as isize);
            let mut y: u32 = 0 as i32 as u32;
            while y < dst_height {
                let mut x: u32 = 0 as i32 as u32;
                while x < dst_width {
                    let fresh2 = src;
                    src = src.offset(1);
                    let fresh3 = dst;
                    dst = dst.offset(1);
                    *fresh3 = *fresh2;
                    x = x.wrapping_add(1);
                }
                src = src.offset(src_stride.wrapping_sub(dst_width) as isize);
                y = y.wrapping_add(1);
            }
        } else {
            let x_offset_0: i32 = dst_width
                .wrapping_sub(src_width)
                .wrapping_div(2 as i32 as u32) as i32;
            let mut y_0: u32 = 0 as i32 as u32;
            while y_0 < new_height {
                dst = dst.offset(x_offset_0 as isize);
                let mut x_0: u32 = 0 as i32 as u32;
                while x_0 < src_width {
                    let fresh4 = src;
                    src = src.offset(1);
                    let fresh5 = dst;
                    dst = dst.offset(1);
                    *fresh5 = *fresh4;
                    x_0 = x_0.wrapping_add(1);
                }
                dst = dst.offset(
                    dst_width.wrapping_sub(src_width.wrapping_add(x_offset_0 as u32)) as isize,
                );
                src = src.offset(src_stride.wrapping_sub(src_width) as isize);
                y_0 = y_0.wrapping_add(1);
            }
        }
    } else {
        let y_offset_0: i32 = dst_height
            .wrapping_sub(src_height)
            .wrapping_div(2 as i32 as u32) as i32;
        dst = dst.offset((y_offset_0 as u32).wrapping_mul(dst_width) as isize);
        if src_width > dst_width {
            let x_offset_1: i32 = src_width
                .wrapping_sub(dst_width)
                .wrapping_div(2 as i32 as u32) as i32;
            src = src.offset(x_offset_1 as isize);
            let mut y_1: u32 = 0 as i32 as u32;
            while y_1 < src_height {
                let mut x_1: u32 = 0 as i32 as u32;
                while x_1 < dst_width {
                    let fresh6 = src;
                    src = src.offset(1);
                    let fresh7 = dst;
                    dst = dst.offset(1);
                    *fresh7 = *fresh6;
                    x_1 = x_1.wrapping_add(1);
                }
                src = src.offset(src_stride.wrapping_sub(dst_width) as isize);
                y_1 = y_1.wrapping_add(1);
            }
        } else {
            let x_offset_2: i32 = dst_width
                .wrapping_sub(src_width)
                .wrapping_div(2 as i32 as u32) as i32;
            dst = dst.offset(x_offset_2 as isize);
            let mut y_2: u32 = 0 as i32 as u32;
            while y_2 < src_height {
                let mut x_2: u32 = 0 as i32 as u32;
                while x_2 < src_width {
                    let fresh8 = src;
                    src = src.offset(1);
                    let fresh9 = dst;
                    dst = dst.offset(1);
                    *fresh9 = *fresh8;
                    x_2 = x_2.wrapping_add(1);
                }
                dst = dst.offset(dst_width.wrapping_sub(src_width) as isize);
                src = src.offset(src_stride.wrapping_sub(src_width) as isize);
                y_2 = y_2.wrapping_add(1);
            }
        }
    };
}

#[no_mangle]
pub unsafe fn image_upper_left(
    mut dst: *mut u32,
    dst_width: u32,
    dst_height: u32,
    mut src: *const u32,
    src_width: u32,
    src_height: u32,
    src_stride: u32,
    bg_clear: u32,
) {
    let mut i: u32 = 0 as i32 as u32;
    while i < dst_width.wrapping_mul(dst_height) {
        *dst.offset(i as isize) = bg_clear;
        i = i.wrapping_add(1);
    }
    if src_height > dst_height {
        let y_offset: i32 = src_height
            .wrapping_sub(dst_height)
            .wrapping_div(2 as i32 as u32) as i32;
        let new_height: u32 = src_height.wrapping_sub(y_offset as u32);
        if src_width > dst_width {
            let mut y: u32 = 0 as i32 as u32;
            while y < dst_height {
                let mut x: u32 = 0 as i32 as u32;
                while x < dst_width {
                    let fresh10 = src;
                    src = src.offset(1);
                    let fresh11 = dst;
                    dst = dst.offset(1);
                    *fresh11 = *fresh10;
                    x = x.wrapping_add(1);
                }
                src = src.offset(src_stride.wrapping_sub(dst_width) as isize);
                y = y.wrapping_add(1);
            }
        } else {
            let mut y_0: u32 = 0 as i32 as u32;
            while y_0 < new_height {
                let mut x_0: u32 = 0 as i32 as u32;
                while x_0 < src_width {
                    let fresh12 = src;
                    src = src.offset(1);
                    let fresh13 = dst;
                    dst = dst.offset(1);
                    *fresh13 = *fresh12;
                    x_0 = x_0.wrapping_add(1);
                }
                dst = dst.offset(dst_width.wrapping_sub(src_width) as isize);
                src = src.offset(src_stride.wrapping_sub(src_width) as isize);
                y_0 = y_0.wrapping_add(1);
            }
        }
    } else if src_width > dst_width {
        let mut y_1: u32 = 0 as i32 as u32;
        while y_1 < src_height {
            let mut x_1: u32 = 0 as i32 as u32;
            while x_1 < dst_width {
                let fresh14 = src;
                src = src.offset(1);
                let fresh15 = dst;
                dst = dst.offset(1);
                *fresh15 = *fresh14;
                x_1 = x_1.wrapping_add(1);
            }
            src = src.offset(src_stride.wrapping_sub(dst_width) as isize);
            y_1 = y_1.wrapping_add(1);
        }
    } else {
        let mut y_2: u32 = 0 as i32 as u32;
        while y_2 < src_height {
            let mut x_2: u32 = 0 as i32 as u32;
            while x_2 < src_width {
                let fresh16 = src;
                src = src.offset(1);
                let fresh17 = dst;
                dst = dst.offset(1);
                *fresh17 = *fresh16;
                x_2 = x_2.wrapping_add(1);
            }
            dst = dst.offset(dst_width.wrapping_sub(src_width) as isize);
            src = src.offset(src_stride.wrapping_sub(src_width) as isize);
            y_2 = y_2.wrapping_add(1);
        }
    };
}
