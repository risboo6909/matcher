#[macro_use]
extern crate criterion;

use rand;

use criterion::Criterion;
use matcher::{Matcher, OrderType, Side};
use rand::Rng;

#[inline]
fn rand_range(start: u64, end: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(start, end)
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("matcher", move |b| {
        b.iter_with_setup(
            || {
                let mut matcher = Matcher::new(false);
                for _x in 0..7000 {
                    matcher.new_order(
                        Side::Sell,
                        rand_range(5, 10),
                        rand_range(1, 10),
                        rand_range(1, 13),
                        OrderType::Limit,
                    );
                }
                matcher
            },
            |mut matcher| {
                matcher.new_order(Side::Buy, rand_range(1, 10), 201, 12, OrderType::Limit);
            },
        )
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
