#[cfg(any(feature = "ring", feature = "aws-lc-rs"))]
mod bogo_shim_impl;

fn main() {
    #[cfg(any(feature = "ring", feature = "aws-lc-rs"))]
    bogo_shim_impl::main();

    #[cfg(not(any(feature = "ring", feature = "aws-lc-rs")))]
    panic!("requires ring or aws_lc_rs feature");
}
