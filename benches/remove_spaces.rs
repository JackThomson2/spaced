use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
const TEST_STRING: &str = "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            ";

fn bench_remove(c: &mut Criterion) {
    println!("Count is {}", spaced::count_spaces(black_box(TEST_STRING)));

    c.bench_function("Base remove approach", |b| {
        b.iter(|| {
            let mut stri = TEST_STRING.to_string();
            remove_whitespace(&mut black_box(stri))
        })
    });
}

fn simd_rm(c: &mut Criterion) {
    println!("Count is {}", spaced::count_spaces(black_box(TEST_STRING)));

    c.bench_function("SIMD remove approach", |b| {
        b.iter(|| {
            let mut stri = TEST_STRING.to_string();
            spaced::despace_string(&mut black_box(stri))
        })
    });
}

fn jmp_rm(c: &mut Criterion) {
    println!("Count is {}", spaced::count_spaces(black_box(TEST_STRING)));

    c.bench_function("Jump table remove approach", |b| {
        b.iter(|| {
            let mut stri = TEST_STRING.to_string();
            unsafe { spaced::jump_tables::de_space_str(&mut black_box(stri)) }
        })
    });
}

fn avx_512_rm(c: &mut Criterion) {
    println!("Count is {}", spaced::count_spaces(black_box(TEST_STRING)));

    c.bench_function("AVX 512 remove approach", |b| {
        b.iter(|| {
            let mut stri = TEST_STRING.to_string();
            unsafe { spaced::de_space_str_avx(&mut black_box(stri)) }
        })
    });
}

fn simd_u4_rm(c: &mut Criterion) {
    println!("Count is {}", spaced::count_spaces(black_box(TEST_STRING)));

    c.bench_function("Simd unrolled 4 remove approach", |b| {
        b.iter(|| {
            let mut stri = TEST_STRING.to_string();
            unsafe { spaced::de_space_str_u4(&mut black_box(stri)) }
        })
    });
}

criterion_group!(
    benches,
    bench_remove,
    simd_rm,
    jmp_rm,
    avx_512_rm,
    simd_u4_rm
);
criterion_main!(benches);
