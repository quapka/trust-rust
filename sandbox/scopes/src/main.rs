fn main() {
    let x = 0xdead;
    for i in 0..2 {
        // will always print 'x: 0xdead'
        println!("outer: x: 0x{:3x} @: {:p}", x, &x);
        if i < 0 {
            break;
        }
        let x = 0xbeef;
        // will always print 'x: beef'
        println!("inner: x: 0x{:3x} @: {:p}", x, &x);
    }
}
