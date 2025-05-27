use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;



#[derive(Clone)]
struct Currency(&'static str);

#[derive(Clone)]
struct Monetary<T> {
    amount: T,
    currency: Currency,
}

impl<T> Monetary<T> {
    fn new(amount: T, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

fn bench_money_f64(c: &mut Criterion) {
    let usd = Currency("USD");

    c.bench_function("f64 money addition", |b| {
        b.iter(|| {
            let m1 = Monetary::new(black_box(12345.6789_f64), usd.clone());
            let m2 = Monetary::new(black_box(98765.4321_f64), usd.clone());
            let _sum = m1.amount + m2.amount;
        })
    });
}

fn bench_money_decimal(c: &mut Criterion) {
    let usd = Currency("USD");

    c.bench_function("Decimal money addition", |b| {
        b.iter(|| {
            let m1 = Monetary::new(black_box(Decimal::from_f64(12345.6789).unwrap()), usd.clone());
            let m2 = Monetary::new(black_box(Decimal::from_f64(98765.4321).unwrap()), usd.clone());
            let _sum = m1.amount + m2.amount;
        })
    });
}

criterion_group!(benches, bench_money_f64, bench_money_decimal);
criterion_main!(benches);
