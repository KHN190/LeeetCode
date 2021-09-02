mod a1;
mod a13;
mod a133;
mod a1805;
mod a187;
mod a187_s2;
mod a20;
mod a295;
mod a295_s2;
mod a295_s3;
mod a32;
mod a480;
mod a480_s2;
mod a535;
mod a7;
mod a705;
mod a9;
mod a9_s2;
mod a9_s3;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    println!("compile success!");
}
