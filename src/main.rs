extern crate permutohedron;
extern crate atty;

static WORDLIST : &str = include_str!("wordlist");

use std::io::Write;
use std::collections::BTreeSet;

fn main() {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut words = BTreeSet::new();
    for word in WORDLIST.split('\n') {
        words.insert(word);
    }

    let isatty = atty::is(atty::Stream::Stdin);

    loop {
        if isatty {
            out.write(b"> ");
            out.flush();
        }
        let mut buf = String::with_capacity(10);  // I apologize
        if std::io::stdin().read_line(&mut buf).expect("IO error reading input!") == 0 { break; }
        let mut buf = buf.trim().to_owned().into_bytes(); // I apologize even harder
        
        for permutation in permutohedron::Heap::new(&mut buf) {
            if words.contains(unsafe { std::str::from_utf8_unchecked(&permutation) }.trim()) {
                out.write(&permutation);
                out.write(b"\n");
            }
        }
    }
}
