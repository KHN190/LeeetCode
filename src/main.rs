pub mod a1;
pub mod a1044;
pub mod a13;
pub mod a133;
pub mod a1805;
pub mod a187;
pub mod a187_s2;
pub mod a20;
pub mod a21;
pub mod a239;
pub mod a295;
pub mod a295_s2;
pub mod a295_s3;
pub mod a32;
pub mod a4;
pub mod a480;
pub mod a480_s2;
pub mod a5;
pub mod a53;
pub mod a535;
pub mod a7;
pub mod a705;
pub mod a705_s2;
pub mod a72;
pub mod a72_s2;
pub mod a9;
pub mod a9_s2;
pub mod a9_s3;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    println!("compile success!");
}
