extern crate permutohedron;
extern crate twoway;

static WORDLIST : &[u8] = include_bytes!("wordlist");

use std::io::Write;

fn main() {
    let mut buf = String::with_capacity(10);
    std::io::stdin().read_line(&mut buf).expect("IO error reading input!");
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    
    let mut buf = buf.into_bytes();
    for permutation in permutohedron::Heap::new(&mut buf) {
        match twoway::find_bytes(WORDLIST, &permutation) {
            Some(idx) => { out.write(&WORDLIST[idx .. idx+permutation.len()+1]); }
            None => { }
        }
    }
}
