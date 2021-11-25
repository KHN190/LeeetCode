pub mod a1;
pub mod a1044;
pub mod a1044_s2;
pub mod a11;
pub mod a13;
pub mod a133;
pub mod a139;
pub mod a139_s2;
pub mod a139_s3;
pub mod a15;
pub mod a1805;
pub mod a187;
pub mod a187_s2;
pub mod a2;
pub mod a20;
pub mod a21;
pub mod a22;
pub mod a23;
pub mod a239;
pub mod a23_s2;
pub mod a240;
pub mod a240_s2;
pub mod a295;
pub mod a295_s2;
pub mod a295_s3;
pub mod a32;
pub mod a347;
pub mod a347_s2;
pub mod a4;
pub mod a45;
pub mod a480;
pub mod a480_s2;
pub mod a5;
pub mod a53;
pub mod a535;
pub mod a55;
pub mod a62;
pub mod a621;
pub mod a62_s2;
pub mod a64;
pub mod a7;
pub mod a705;
pub mod a705_s2;
pub mod a72;
pub mod a72_s2;
pub mod a75;
pub mod a79;
pub mod a9;
pub mod a98;
pub mod a9_s2;
pub mod a9_s3;
mod types;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    println!("compile success!");
}
