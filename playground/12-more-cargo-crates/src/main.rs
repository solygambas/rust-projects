// use cargo_crates::kinds::PrimaryColor;
use cargo_crates::PrimaryColor;
// use cargo_crates::utils::mix;
use cargo_crates::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
