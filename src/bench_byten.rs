use byten::EncodeToVec;
use criterion::{black_box, Criterion};

use crate::datasets::BorrowableData;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: byten::Encode + byten::EncodeToVec + byten::DecodeOwned + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/byten", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            data.encode(black_box(serialize_buffer.as_mut_slice()), black_box(&mut 0))
                .unwrap();
            black_box(());
        })
    });

    let deserialize_buffer = data.encode_to_vec().unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::decode(black_box(&deserialize_buffer), black_box(&mut 0)).unwrap());
        })
    });

    crate::bench_size(name, "byten", deserialize_buffer.as_slice());

    assert!(T::decode(black_box(&deserialize_buffer), black_box(&mut 0)).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BorrowableData + byten::Encode + byten::DecodeOwned + EncodeToVec,
    for<'encoded> T::Borrowed<'encoded>: byten::Encode + byten::Decode<'encoded>,
{
    bench(name, c, data);

    use byten::Decode;

    let mut group = c.benchmark_group(format!("{}/byten", name));

    let deserialize_buffer = data.encode_to_vec().unwrap();

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(T::Borrowed::decode(black_box(&deserialize_buffer), black_box(&mut 0)).unwrap());
        })
    });

    group.finish();
}
