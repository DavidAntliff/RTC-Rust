use rust_rtc::add;
use rust_rtc::tuples::tuple;
use rust_rtc::colors::color;
use rust_rtc::matrices::identity4;

fn main() {
    println!("Hello, world!");
    println!("{}", add(1, 2));

    let _t = tuple(0.0, 0.0, 0.0, 0.0);
    let _c = color(0.5, 0.5, 0.5);
    let _m = identity4();

    println!("{:?}", _t);
    println!("{:?}", _c);
    println!("{:?}", _m);

}
