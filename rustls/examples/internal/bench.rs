#[cfg(any(feature = "ring", feature = "aws-lc-rs"))]
mod bench_impl;

fn main() {
    #[cfg(any(feature = "ring", feature = "aws-lc-rs"))]
    bench_impl::main();

    #[cfg(not(any(feature = "ring", feature = "aws-lc-rs")))]
    panic!("no provider to test");
}
