#![feature(can_vector)]
#![feature(exclusive_range_pattern)]
#![feature(allocator_api)]
#![feature(const_mut_refs)]
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]

use crate::sword::SWord;

mod sword;

fn main() {
    let sword = SWord::new("Hello World!");
    println!("{}", sword); // hello_world.
    assert_eq!(sword, SWord::new("hello-world?")); // true

    println!("{}", SWord::new("0123456789ABC")); //
    println!("{}", SWord::new("0123456789ABCD"));
    println!("{}", SWord::new("0123456789ABCDE"));
    assert_eq!(
        SWord::new("0123456789ABCDE"),
        SWord::new("0123456789ABCDEF")
    );

    assert!(SWord::new("a") > SWord::new(""));
    assert!(SWord::new("ab") > SWord::new("aa"));
    assert!(SWord::new("ab") > SWord::new("a"));
    assert!(SWord::new("aa") > SWord::new("a0"));
}
