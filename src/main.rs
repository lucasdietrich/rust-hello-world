fn main() {
    let hello = ws_hello::hello();
    let world = ws_world::world();
    println!("{} {}!", hello, world);

    let mut bufs = vec![vec![]; 5];

    rmp::encode::write_pfix(&mut bufs[0], 42).unwrap();
    rmp::encode::write_u8(&mut bufs[1], 42).unwrap();
    rmp::encode::write_u16(&mut bufs[2], 42).unwrap();
    rmp::encode::write_u32(&mut bufs[3], 42).unwrap();
    rmp::encode::write_u64(&mut bufs[4], 42).unwrap();

    println!("{:?}", bufs);

    assert_eq!([0x2a], bufs[0][..]);
    assert_eq!([0xcc, 0x2a], bufs[1][..]);
    assert_eq!([0xcd, 0x00, 0x2a], bufs[2][..]);
    assert_eq!([0xce, 0x00, 0x00, 0x00, 0x2a], bufs[3][..]);
    assert_eq!([0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a], bufs[4][..]);

    #[cfg(feature = "mylib")]
    mylib::MyLib::new();
}
