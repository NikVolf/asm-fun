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
            :
            );
        }
        (result, overflow != 0)
    }

    #[cfg(target_arch = "x86_64")]
    fn add_setc(p1: [u64; 4], p2: [u64; 4]) -> ([u64; 4], bool) {
        let mut result: [u64; 4] = unsafe { mem::uninitialized() };
        let overflow: u8;
        unsafe {
            asm!("
                adc $9, %r8
                adc $10, %r9
                adc $11, %r10
                adc $12, %r11
                setc %al
                "
            : "={r8}"(result[0]), "={r9}"(result[1]), "={r10}"(result[2]), "={r11}"(result[3]), "={al}"(overflow)
            : "{r8}"(p1[0]), "{r9}"(p1[1]), "{r10}"(p1[2]), "{r11}"(p1[3]), "m"(p2[0]), "m"(p2[1]), "m"(p2[2]), "m"(p2[3])
            :
            :
            );
        }
        (result, overflow != 0)
    }

	fn sub(p1: [u64; 4], p2: [u64; 4]) -> ([u64; 4], bool) {
        let mut result: [u64; 4] = unsafe { mem::uninitialized() };
        let overflow: u8;
        unsafe {
            asm!("
                sbb $9, %r8
                sbb $10, %r9
                sbb $11, %r10
                sbb $12, %r11
                setb %al
                "
            : "={r8}"(result[0]), "={r9}"(result[1]), "={r10}"(result[2]), "={r11}"(result[3]), "={al}"(overflow)
            : "{r8}"(p1[0]), "{r9}"(p1[1]), "{r10}"(p1[2]), "{r11}"(p1[3]), "m"(p2[0]), "m"(p2[1]), "m"(p2[2]), "m"(p2[3])
            :
            :
            );
        }
        (result, overflow != 0)
	}

	fn mul(p1: [u64; 4], p2: [u64; 4]) -> ([u64; 4], bool) {
        let mut result: [u64; 4] = unsafe { mem::uninitialized() };
        let overflow: u64;
        unsafe {
            asm!("
				mov $5, %rax
				mulq $9
				mov %rax, %r8
				adc $6, %rdx
				pushf

				mov %rdx, %rax
				mulq $9
				popf
				adc $$0, %rax
				adc $7, %rdx
				pushf
				mov %rax, %r9


				mov %rdx, %rax
				mulq $9
				popf
				adc $$0, %rax
				adc $8, %rdx
				pushf
				mov %rax, %r10

				mov %rdx, %rax
				mulq $9
				popf
				adc $$0, %rax
				mov %rax, %r11
				mov %rdx, %rcx

				mov $5, %rax
				mulq $10
				adc %rax, %r9
				adc $6, %rdx
				pushf

				mov %rdx, %rax
				mulq $10
				popf
				adc %rax, %r10
				adc $7, %rdx
				pushf

				mov %rdx, %rax
				mulq $10
				popf
				adc %rax, %r11
				pushf
				or %rax, %rcx

				mov $5, %rax
				mulq $11
				popf
				adc %rax, %r10
				adc $6, %rdx
				pushf

				mov %rdx, %rax
				mulq $11
				popf
				adc %rax, %r11
				pushf
				or %rdx, %rcx

				mov $5, %rax
				mulq $12
				popf
				adc %rax, %r11
			    or %rdx, %rcx
                "
            : /* $0 */ "={r8}"(result[0]), /* $1 */ "={r9}"(result[1]), /* $2 */ "={r10}"(result[2]),
			  /* $3 */ "={r11}"(result[3]), /* $4 */  "={rcx}"(overflow)

            : /* $5 */ "m"(p1[0]), /* $6 */ "m"(p1[1]), /* $7 */  "m"(p1[2]),
			  /* $8 */ "m"(p1[3]), /* $9 */ "m"(p2[0]), /* $10 */ "m"(p2[1]),
			  /* $11 */ "m"(p2[2]), /* $12 */ "m"(p2[3])
            : "rax", "rdx"
            :
            );
        }
        (result, overflow > 0)
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


    #[test]
    fn it_substracts() {
        let (result, _) = sub([0, 0, 0, 0], [0, 0, 0, 0]);
        assert_eq!(result, [0, 0, 0, 0]);

        let (result, _) = sub([0, 0, 0, 1], [0, 0, 0, 1]);
        assert_eq!(result, [0, 0, 0, 0]);

        let (_, overflow) = sub([0, 0, 2, 1], [0, 0, 3, 1]);
        assert!(overflow);

        let (result, overflow) = sub([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
                                [::std::u64::MAX/2, ::std::u64::MAX/2, ::std::u64::MAX/2, ::std::u64::MAX/2]);
        assert!(!overflow);
        assert_eq!([::std::u64::MAX/2+1, ::std::u64::MAX/2+1, ::std::u64::MAX/2+1, ::std::u64::MAX/2+1], result);

        let (result, overflow) = sub([0, 0, 0, 1], [0, 0, 1, 0]);
        assert!(!overflow);
        assert_eq!([0, 0, ::std::u64::MAX, 0], result);

        let (result, overflow) = sub([0, 0, 0, 1], [1, 0, 0, 0]);
        assert!(!overflow);
        assert_eq!([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0], result);
    }

	#[test]
	fn it_multiplies() {
        let (result, _) = mul([0, 0, 0, 0], [0, 0, 0, 0]);
        assert_eq!([0, 0, 0, 0], result);

        let (result, _) = mul([1, 0, 0, 0], [1, 0, 0, 0]);
        assert_eq!([1, 0, 0, 0], result);

        let (result, _) = mul([5, 0, 0, 0], [5, 0, 0, 0]);
        assert_eq!([25, 0, 0, 0], result);

        let (result, _) = mul([0, 5, 0, 0], [0, 5, 0, 0]);
        assert_eq!([0, 0, 25, 0], result);

        let (result, _) = mul([0, 0, 0, 1], [1, 0, 0, 0]);
        assert_eq!([0, 0, 0, 1], result);

        let (result, _) = mul([0, 0, 0, 5], [2, 0, 0, 0]);
        assert_eq!([0, 0, 0, 10], result);

        let (result, _) = mul([0, 0, 1, 0], [0, 5, 0, 0]);
        assert_eq!([0, 0, 0, 5], result);

        let (result, _) = mul([2, 0, 0, 0], [0, 5, 0, 0]);
        assert_eq!([0, 10, 0, 0], result);
	}

    #[bench]
    fn add_oldschool_u256(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256::zero(), |old, new| { old.overflowing_add(U256::from(new)).0 })
        });
    }

    #[bench]
    fn mul_oldschool_u256(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256::from(12345u64), |old, new| { old.overflowing_mul(U256::from(new)).0 })
        });
    }

    #[bench]
    fn sub_oldschool_u256(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]),
				|old, new| { old.overflowing_sub(U256::from(new)).0 })
        });
    }

    #[bench]
    fn add_asm_xor(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([0u64, 0u64, 0u64, 0u64], |old, new| { add(old, [0, 0, 0, new]).0 })
        });
    }

    #[bench]
    fn add_asm_setc(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([0u64, 0u64, 0u64, 0u64], |old, new| { add_setc(old, [0, 0, 0, new]).0 })
        });
    }

    #[bench]
    fn asm_sub_setc(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX], |old, new| { sub(old, [0, 0, 0, new]).0 })
        });
    }

    #[bench]
    fn asm_mul(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([12345u64, 0u64, 0u64, 0u64], |old, new| { mul(old, [0, 0, 0, new]).0 })
        });
    }
}
