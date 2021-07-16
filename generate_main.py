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
fs = []
for x in os.listdir('src'):
    m = re.findall(r'(a\d+[_]?.*)\.rs', x)
    if m:
        fs.append(m[0])

# write mods
header = '\n'.join(['mod ' + x + ';' for x in fs])

with open('src/main.rs', 'w+') as f:
    f.write(header + template)

# format main.rs
os.system('rustfmt src/main.rs')
