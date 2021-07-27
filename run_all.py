# generate main.rs

import re, os

template = '''

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    println!("compile success!");
}

'''

# get *.rs except main
fs, uniq_fs = [], set()
for x in os.listdir('src'):
    m = re.findall(r'(a\d+[_]?.*)\.rs', x)
    n = re.findall(r'a(\d+)\.rs', x)
    if m: fs.append(m[0])
    if n: uniq_fs.add(n[0])


print("Generating {} solutions, {} unique.".format(len(fs), len(uniq_fs)))

# write mods
header = '\n'.join(['mod ' + x + ';' for x in fs])

with open('src/main.rs', 'w+') as f:
    f.write(header + template)

# format main.rs
os.system('rustfmt src/main.rs')
os.system('cargo test')
