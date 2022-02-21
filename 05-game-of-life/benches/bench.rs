// to use feature, you need to run: rustup default nightly
// cargo bench | tee before.txt
// cargo bench | tee after.txt
// cargo benchcmp before.txt after.txt // x 4.19
// rustup default stable
#![feature(test)]

extern crate game_of_life;
extern crate test;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = game_of_life::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
