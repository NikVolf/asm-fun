#![feature(asm)]
#![feature(test)]

extern crate test;
extern crate ethcore_util as util;

#[cfg(test)]
mod h256tests {

    use std::mem;
    use test::*;
    use util::hash::*;
    use util::uint::*;

    #[cfg(target_arch = "x86_64")]
    fn add(p1: [u64; 4], p2: [u64; 4]) -> ([u64; 4], bool) {
        let mut result: [u64; 4] = unsafe { mem::uninitialized() };
        let overflow: u8;
        unsafe {
            asm!("
                xor %al, %al
                adc $9, %r8
                adc $10, %r9
                adc $11, %r10
                adc $12, %r11
                adc $$0, %al
                "
            : "={r8}"(result[0]), "={r9}"(result[1]), "={r10}"(result[2]), "={r11}"(result[3]), "={al}"(overflow)
            : "{r8}"(p1[0]), "{r9}"(p1[1]), "{r10}"(p1[2]), "{r11}"(p1[3]), "m"(p2[0]), "m"(p2[1]), "m"(p2[2]), "m"(p2[3])
            :
            : "volatile"
            );
        }
        (result, overflow != 0)
    }

    #[test]
    fn it_adds() {
        let (result, _) = add([0, 0, 0, 0], [0, 0, 0, 0]);
        assert_eq!(result, [0, 0, 0, 0]);

        let (result, _) = add([0, 0, 0, 1], [0, 0, 0, 1]);
        assert_eq!(result, [0, 0, 0, 2]);

        let (result, _) = add([0, 0, 2, 1], [0, 0, 3, 1]);
        assert_eq!(result, [0, 0, 5, 2]);

        let (result, overflow) = add([0, 0, 2, 1], [0, 0, 3, 1]);
        assert_eq!(result, [0, 0, 5, 2]);
        assert!(!overflow);

        let (_, overflow) = add([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
                                [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert!(overflow);

        let (_, overflow) = add([0, 0, 0, ::std::u64::MAX],
                                [0, 0, 0, ::std::u64::MAX]);
        assert!(overflow);
    }

    #[bench]
    fn add_u256_regular(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256::zero(), |old, new| { old.overflowing_add(U256::from(new)).0 })
        });
    }

    #[bench]
    fn add_u256_asm(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([0u64, 0u64, 0u64, 0u64], |old, new| { add(old, [0, 0, 0, new]).0 })
        });
    }

}
