use crate::consts::DESPACE_MASK16;

use std::arch::x86_64 as x86;

pub fn count_spaces(string: &str) -> u32 {
    unsafe {
        let mut count = 0;
        let mut i = 0;

        let spaces = x86::_mm256_set1_epi8(' ' as i8);
        let newline = x86::_mm256_set1_epi8('\n' as i8);
        let carriage = x86::_mm256_set1_epi8('\r' as i8);

        let spaces128 = x86::_mm_set1_epi8(' ' as i8);
        let newline128 = x86::_mm_set1_epi8('\n' as i8);
        let carriage128 = x86::_mm_set1_epi8('\r' as i8);

        while i + 32 < string.len() {
            let ptr = string.as_ptr().add(i);

            let x = x86::_mm256_loadu_si256(ptr.cast());

            let xspaces = x86::_mm256_cmpeq_epi8(x, spaces);
            let xnewline = x86::_mm256_cmpeq_epi8(x, newline);
            let xcarriage = x86::_mm256_cmpeq_epi8(x, carriage);

            let anywhite = x86::_mm256_or_si256(x86::_mm256_or_si256(xspaces, xnewline), xcarriage);
            count += (x86::_mm256_movemask_epi8(anywhite)).count_ones();
            i += 32;
        }

        while i + 16 < string.len() {
            let ptr = string.as_ptr().add(i);

            let x = x86::_mm_loadu_si128(ptr.cast());

            let xspaces = x86::_mm_cmpeq_epi8(x, spaces128);
            let xnewline = x86::_mm_cmpeq_epi8(x, newline128);
            let xcarriage = x86::_mm_cmpeq_epi8(x, carriage128);

            let anywhite = x86::_mm_or_si128(x86::_mm_or_si128(xspaces, xnewline), xcarriage);
            count += (x86::_mm_movemask_epi8(anywhite)).count_ones();
            i += 16;
        }

        for pos in i..string.len() {
            let c = *string.as_ptr().add(pos);
            count += *crate::jump_tables::OPPO_JUMP_TABLE.get_unchecked(c as usize) as u32;
        }
        count
    }
}

#[inline]
unsafe fn cleanm128(
    x: x86::__m128i,
    spaces: x86::__m128i,
    newline: x86::__m128i,
    carriage: x86::__m128i,
    mask16: &mut i32,
) -> x86::__m128i {
    let xspaces = x86::_mm_cmpeq_epi8(x, spaces);
    let xnewline = x86::_mm_cmpeq_epi8(x, newline);
    let xcarriage = x86::_mm_cmpeq_epi8(x, carriage);

    let anywhite = x86::_mm_or_si128(x86::_mm_or_si128(xspaces, xnewline), xcarriage);

    *mask16 = x86::_mm_movemask_epi8(anywhite);

    let mask_16 = (*mask16 & 0x7fff) as u16;
    let addr: *const u8 = DESPACE_MASK16.get_unchecked(mask_16 as usize * 16);

    let mask = x86::_mm_loadu_si128(addr.cast());

    x86::_mm_shuffle_epi8(x, mask)
}

#[inline]
unsafe fn cleanm256(
    x: x86::__m256i,
    spaces: x86::__m256i,
    newline: x86::__m256i,
    carriage: x86::__m256i,
    mask1: &mut u32,
    mask2: &mut u32,
) -> x86::__m256i {
    let xspaces = x86::_mm256_cmpeq_epi8(x, spaces);
    let xnewline = x86::_mm256_cmpeq_epi8(x, newline);
    let xcarriage = x86::_mm256_cmpeq_epi8(x, carriage);

    let anywhite = x86::_mm256_or_si256(x86::_mm256_or_si256(xspaces, xnewline), xcarriage);

    let mask32 = x86::_mm256_movemask_epi8(anywhite) as u32;

    let mask_high = mask32 >> 16;
    let mask_low = mask32 & 0xFFFF;

    *mask1 = mask_low;
    *mask2 = mask_high;

    let hi_addr: *const u8 = DESPACE_MASK16.get_unchecked((mask_high & 0x7fff) as usize * 16);
    let lo_addr: *const u8 = DESPACE_MASK16.get_unchecked((mask_low & 0x7fff) as usize * 16);

    let mask = x86::_mm256_loadu2_m128i(
        hi_addr.cast::<x86::__m128i>(),
        lo_addr.cast::<x86::__m128i>(),
    );

    x86::_mm256_shuffle_epi8(x, mask)
}

#[inline]
pub unsafe fn de_space_str(string: &mut str) -> usize {
    let spaces128 = x86::_mm_set1_epi8(b' ' as i8);
    let newline128 = x86::_mm_set1_epi8(b'\n' as i8);
    let carriage128 = x86::_mm_set1_epi8(b'\r' as i8);

    let mut pos: usize = 0;
    let mut i = 0;

    while i + 16 - 1 < string.len() {
        let ptr = string.as_ptr().add(i);
        let mut x = x86::_mm_loadu_si128(ptr.cast());
        let mut mask_16 = 0;

        x = cleanm128(x, spaces128, newline128, carriage128, &mut mask_16);

        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x);

        pos += 16 - mask_16.count_ones() as usize;
        i += 16;
    }

    for pos_i in i..string.len() {
        let c = *string.as_ptr().add(pos_i);
        *string.as_mut_ptr().add(pos) = c;
        pos += *crate::jump_tables::JUMP_TABLE.get_unchecked(c as usize);
    }

    pos
}

#[inline]
pub unsafe fn de_space_str_u4(string: &mut str) -> usize {
    let spaces128 = x86::_mm_set1_epi8(b' ' as i8);
    let newline128 = x86::_mm_set1_epi8(b'\n' as i8);
    let carriage128 = x86::_mm_set1_epi8(b'\r' as i8);

    let mut pos: usize = 0;
    let mut i = 0;

    while i + 64 - 1 < string.len() {
        let ptr = string.as_ptr().add(i);

        let mut x1 = x86::_mm_loadu_si128(ptr.cast());
        let mut x2 = x86::_mm_loadu_si128(ptr.add(16).cast());
        let mut x3 = x86::_mm_loadu_si128(ptr.add(32).cast());
        let mut x4 = x86::_mm_loadu_si128(ptr.add(48).cast());

        let mut mask_16 = 0;

        x1 = cleanm128(x1, spaces128, newline128, carriage128, &mut mask_16);
        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x1);
        pos += 16 - mask_16.count_ones() as usize;

        x2 = cleanm128(x2, spaces128, newline128, carriage128, &mut mask_16);
        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x2);
        pos += 16 - mask_16.count_ones() as usize;

        x3 = cleanm128(x3, spaces128, newline128, carriage128, &mut mask_16);
        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x3);
        pos += 16 - mask_16.count_ones() as usize;

        x4 = cleanm128(x4, spaces128, newline128, carriage128, &mut mask_16);
        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x4);
        pos += 16 - mask_16.count_ones() as usize;

        i += 64;
    }

    while i + 16 - 1 < string.len() {
        let ptr = string.as_ptr().add(i);
        let mut x = x86::_mm_loadu_si128(ptr.cast());
        let mut mask_16 = 0;

        x = cleanm128(x, spaces128, newline128, carriage128, &mut mask_16);

        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x);

        pos += 16 - mask_16.count_ones() as usize;
        i += 16;
    }

    for pos_i in i..string.len() {
        let c = *string.as_ptr().add(pos_i);
        *string.as_mut_ptr().add(pos) = c;
        pos += *crate::jump_tables::JUMP_TABLE.get_unchecked(c as usize);
    }

    pos
}

#[inline]
pub unsafe fn de_space_str_avx(string: &mut str) -> usize {
    let spaces = x86::_mm256_set1_epi8(b' ' as i8);
    let newline = x86::_mm256_set1_epi8(b'\n' as i8);
    let carriage = x86::_mm256_set1_epi8(b'\r' as i8);

    let spaces128 = x86::_mm_set1_epi8(b' ' as i8);
    let newline128 = x86::_mm_set1_epi8(b'\n' as i8);
    let carriage128 = x86::_mm_set1_epi8(b'\r' as i8);

    let mut pos: usize = 0;
    let mut i = 0;

    while i + 32 - 1 < string.len() {
        let ptr = string.as_ptr().add(i);
        let mut x = x86::_mm256_loadu_si256(ptr.cast());

        let mut mask_low = 0;
        let mut mask_high = 0;

        x = cleanm256(x, spaces, newline, carriage, &mut mask_low, &mut mask_high);

        let offset1 = 16 - mask_low.count_ones();
        let offset2 = 16 - mask_high.count_ones();

        x86::_mm256_storeu2_m128i(
            string.as_mut_ptr().add(pos + offset1 as usize).cast(),
            string.as_mut_ptr().add(pos).cast(),
            x,
        );

        pos += offset1 as usize + offset2 as usize;
        i += 32;
    }

    while i + 16 - 1 < string.len() {
        let ptr = string.as_ptr().add(i);
        let mut x = x86::_mm_loadu_si128(ptr.cast());
        let mut mask_16 = 0;

        x = cleanm128(x, spaces128, newline128, carriage128, &mut mask_16);

        x86::_mm_storeu_si128(string.as_mut_ptr().add(pos).cast(), x);

        pos += 16 - mask_16.count_ones() as usize;
        i += 16;
    }

    for pos_i in i..string.len() {
        let c = *string.as_ptr().add(pos_i);
        *string.as_mut_ptr().add(pos) = c;
        pos += *crate::jump_tables::JUMP_TABLE.get_unchecked(c as usize);
    }
    pos
}
