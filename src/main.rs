mod a1;
mod a13;
mod a1805;
mod a295;
mod a295_s2;
mod a295_s3;
mod a480;
mod a480_s2;
mod a7;
mod a9;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    println!("compile success!");
}
