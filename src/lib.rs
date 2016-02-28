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
                add $9, %r8
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
                sub $9, %r8
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

    #[inline(always)]
    fn add_512(self_t: [u64; 8], other_t: [u64; 8]) -> ([u64; 8], bool) {
        let mut result: [u64; 8] = unsafe { mem::uninitialized() };
        let overflow: u64;

        unsafe {
            asm!("
            	add $15, $0
            	adc $16, $1
            	adc $17, $2
            	adc $18, $3
            	lodsq
            	adc $11, %rax
            	stosq
            	lodsq
            	adc $12, %rax
            	stosq
            	lodsq
            	adc $13, %rax
            	stosq
            	lodsq
            	adc $14, %rax
            	stosq
            	setc %al
                "
            : "=r"(result[0]), "=r"(result[1]), "=r"(result[2]), "=r"(result[3]),

			  "={al}"(overflow) /* $0 - $4 */

            : "{rdi}"(&result[4] as *const u64) /* $5 */
			  "{rsi}"(&other_t[4] as *const u64) /* $6 */
			  "0"(self_t[0]), "1"(self_t[1]), "2"(self_t[2]), "3"(self_t[3]),
              "m"(self_t[4]), "m"(self_t[5]), "m"(self_t[6]), "m"(self_t[7]),
			  /* $7 - $14 */

			  "m"(other_t[0]), "m"(other_t[1]), "m"(other_t[2]), "m"(other_t[3]),
              "m"(other_t[4]), "m"(other_t[5]), "m"(other_t[6]), "m"(other_t[7]) /* $15 - $22 */
            : "rdi", "rsi"
            :
            );
        }
        (result, overflow != 0)
    }

    #[inline(always)]
    fn sub_512(self_t: [u64; 8], other_t: [u64; 8]) -> ([u64; 8], bool) {
        let mut result: [u64; 8] = unsafe { mem::uninitialized() };
        let overflow: u64;

        unsafe {
            asm!("
            	sub $15, $0
            	sbb $16, $1
            	sbb $17, $2
            	sbb $18, $3
            	lodsq
            	sbb $19, %rax
            	stosq
            	lodsq
            	sbb $20, %rax
            	stosq
            	lodsq
            	sbb $21, %rax
            	stosq
            	lodsq
            	sbb $22, %rax
            	stosq
            	setb %al
                "
            : "=r"(result[0]), "=r"(result[1]), "=r"(result[2]), "=r"(result[3]),

			  "={al}"(overflow) /* $0 - $4 */

            : "{rdi}"(&result[4] as *const u64) /* $5 */
			  "{rsi}"(&self_t[4] as *const u64) /* $6 */
			  "0"(self_t[0]), "1"(self_t[1]), "2"(self_t[2]), "3"(self_t[3]),
              "m"(self_t[4]), "m"(self_t[5]), "m"(self_t[6]), "m"(self_t[7]),
			  /* $7 - $14 */

			  "m"(other_t[0]), "m"(other_t[1]), "m"(other_t[2]), "m"(other_t[3]),
              "m"(other_t[4]), "m"(other_t[5]), "m"(other_t[6]), "m"(other_t[7]) /* $15 - $22 */
            : "rdi", "rsi"
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
				mov %rax, $0
				mov %rdx, $1

				mov $5, %rax
				mulq $10
				add %rax, $1
				adc $$0, %rdx
				mov %rdx, $2

				mov $5, %rax
				mulq $11
				add %rax, $2
				adc $$0, %rdx
				mov %rdx, $3

				mov $5, %rax
				mulq $12
				add %rax, $3
				adc $$0, %rdx
				mov %rdx, %rcx

				mov $6, %rax
				mulq $9
				add %rax, $1
				adc %rdx, $2
				adc $$0, $3
				adc $$0, %rcx

				mov $6, %rax
				mulq $10
				add %rax, $2
				adc %rdx, $3
				adc $$0, %rcx
				adc $$0, $3
				adc $$0, %rcx

				mov $6, %rax
				mulq $11
				add %rax, $3
				adc $$0, %rdx
				or %rdx, %rcx

				mov $7, %rax
				mulq $9
				add %rax, $2
				adc %rdx, $3
				adc $$0, %rcx

				mov $7, %rax
				mulq $10
				add %rax, $3
				adc $$0, %rdx
				or %rdx, %rcx

				mov $8, %rax
				mulq $9
				add %rax, $3
				or %rdx, %rcx

				cmpq $$0, %rcx
				jne 2f

				popcnt $8, %rcx
				jrcxz 12f

				popcnt $12, %rcx
				popcnt $11, %rax
				add %rax, %rcx
				popcnt $10, %rax
				add %rax, %rcx
				jmp 2f

				12:
				popcnt $12, %rcx
				jrcxz 11f

				popcnt $7, %rcx
				popcnt $6, %rax
				add %rax, %rcx

				cmpq $$0, %rcx
				jne 2f

				11:
				popcnt $11, %rcx
				jrcxz 2f
				popcnt $7, %rcx

				2:
				"
            : /* $0 */ "={r8}"(result[0]), /* $1 */ "={r9}"(result[1]), /* $2 */ "={r10}"(result[2]),
			  /* $3 */ "={r11}"(result[3]), /* $4 */  "={rcx}"(overflow)

            : /* $5 */ "m"(p1[0]), /* $6 */ "m"(p1[1]), /* $7 */  "m"(p1[2]),
			  /* $8 */ "m"(p1[3]), /* $9 */ "m"(p2[0]), /* $10 */ "m"(p2[1]),
			  /* $11 */ "m"(p2[2]), /* $12 */ "m"(p2[3])
            : "rax", "rdx", "rbx"
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
    fn it_adds_512() {
        let (result, _) = add_512([0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0]);

        let (result, _) = add_512([1, 0, 0, 0, 0, 0, 0, 1], [1, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, [2, 0, 0, 0, 0, 0, 0, 2]);

        let (result, _) = add_512([0, 0, 0, 0, 0, 0, 0, 1], [0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 2]);

        let (result, _) = add_512([0, 0, 0, 0, 0, 0, 2, 1], [0, 0, 0, 0, 0, 0, 3, 1]);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 5, 2]);

        let (result, _) = add_512([1, 2, 3, 4, 5, 6, 7, 8], [9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(result, [10, 12, 14, 16, 18, 20, 22, 24]);

        let (result, overflow) = add_512([0, 0, 0, 0, 0, 0, 2, 1], [0, 0, 0, 0, 0, 0, 3, 1]);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 5, 2]);
        assert!(!overflow);

        let (_, overflow) = add_512([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
                                    [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert!(overflow);

        let (_, overflow) = add_512([0, 0, 0, 0, 0, 0, 0, ::std::u64::MAX],
                                    [0, 0, 0, 0, 0, 0, 0, ::std::u64::MAX]);
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
    fn it_substracts_512() {
        let (result, _) = sub_512([0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0]);

        let (result, _) = sub_512([10, 9, 8, 7, 6, 5, 4, 3], [9, 8, 7, 6, 5, 4, 3, 2]);
        assert_eq!(result, [1, 1, 1, 1, 1, 1, 1, 1]);

        let (_, overflow) = sub_512([10, 9, 8, 7, 6, 5, 4, 3], [9, 8, 7, 6, 5, 4, 3, 2]);
        assert!(!overflow);

        let (_, overflow) = sub_512([9, 8, 7, 6, 5, 4, 3, 2], [10, 9, 8, 7, 6, 5, 4, 3]);
        assert!(overflow);

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

        let (result, _) = mul([0, 0, 0, 4], [4, 0, 0, 0]);
        assert_eq!([0, 0, 0, 16], result);

        let (result, _) = mul([0, 0, 0, 5], [2, 0, 0, 0]);
        assert_eq!([0, 0, 0, 10], result);

        let (result, _) = mul([0, 0, 2, 0], [0, 5, 0, 0]);
        assert_eq!([0, 0, 0, 10], result);

        let (result, _) = mul([0, 3, 0, 0], [0, 0, 3, 0]);
        assert_eq!([0, 0, 0, 9], result);

        let (result, _) = mul([0, 0, 8, 0], [0, 0, 6, 0]);
        assert_eq!([0, 0, 0, 0], result);

        let (result, _) = mul([9, 0, 0, 0], [0, 3, 0, 0]);
        assert_eq!([0, 27, 0, 0], result);

        let (result, _) = mul([::std::u64::MAX, 0, 0, 0], [::std::u64::MAX, 0, 0, 0]);
        assert_eq!([1, ::std::u64::MAX-1, 0, 0], result);

        let (result, _) = mul([0, ::std::u64::MAX, 0, 0], [::std::u64::MAX, 0, 0, 0]);
        assert_eq!([0, 1, ::std::u64::MAX-1, 0], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, 0, 0], [::std::u64::MAX, 0, 0, 0]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX-1, 0], result);

        let (result, _) = mul([::std::u64::MAX, 0, 0, 0], [::std::u64::MAX, ::std::u64::MAX, 0, 0]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX-1, 0], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, 0, 0],
							  [::std::u64::MAX, ::std::u64::MAX, 0, 0]);
        assert_eq!([1, 0, ::std::u64::MAX-1, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, 0, 0, 0], [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX-1], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0], [::std::u64::MAX, 0, 0, 0]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX-1], result);

        let (result, _) = mul([::std::u64::MAX, 0, 0, 0], [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX], [::std::u64::MAX, 0, 0, 0]);
        assert_eq!([1, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0],
							  [::std::u64::MAX, ::std::u64::MAX, 0, 0]);
        assert_eq!([1, 0, ::std::u64::MAX, ::std::u64::MAX-1], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, 0, 0],
							  [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0]);
        assert_eq!([1, 0, ::std::u64::MAX, ::std::u64::MAX-1], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
                              [::std::u64::MAX, ::std::u64::MAX, 0, 0]);
        assert_eq!([1, 0, ::std::u64::MAX, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, 0, 0],
							  [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert_eq!([1, 0, ::std::u64::MAX, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0],
							  [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0]);
        assert_eq!([1, 0, 0, ::std::u64::MAX-1], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0],
							  [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert_eq!([1, 0, 0, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
							  [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, 0]);
        assert_eq!([1, 0, 0, ::std::u64::MAX], result);

        let (result, _) = mul([0, 0, 0, ::std::u64::MAX], [0, 0, 0, ::std::u64::MAX]);
        assert_eq!([0, 0, 0, 0], result);

        let (result, _) = mul([1, 0, 0, 0], [0, 0, 0, ::std::u64::MAX]);
        assert_eq!([0, 0, 0, ::std::u64::MAX], result);

        let (result, _) = mul([::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX],
                              [::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX, ::std::u64::MAX]);
        assert_eq!([1, 0, 0, 0], result);

	}

    #[test]
    fn it_multiplies_overflow_correct() {
        let (_, overflow) = mul([1, 0, 0, 0], [0, 0, 0, 0]);
        assert!(!overflow);

        let (_, overflow) = mul([1, 0, 0, 0], [0, 0, 0, ::std::u64::MAX]);
        assert!(!overflow);

        let (_, overflow) = mul([0, 1, 0, 0], [0, 1, 0, ::std::u64::MAX]);
        assert!(overflow);

        let (_, overflow) = mul([0, 1, 0, 0], [0, 1, 0, 0]);
        assert!(!overflow);

        let (_, overflow) = mul([0, 1, 0, ::std::u64::MAX], [0, 1, 0, ::std::u64::MAX]);
        assert!(overflow);

        let (_, overflow) = mul([0, ::std::u64::MAX, 0, 0], [0, ::std::u64::MAX, 0, 0]);
        assert!(!overflow);

        let (_, overflow) = mul([1, 0, 0, 0], [10, 0, 0, 0]);
        assert!(!overflow);

        let (_, overflow) = mul([2, 0, 0, 0], [10, 0, 0, ::std::u64::MAX / 2]);
        assert!(!overflow);

        let (_, overflow) = mul([0, 0, 0, 8], [0, 0, 0, 8]);
        assert!(overflow);

        let (_, overflow) = mul([0, 0, 8, 0], [0, 0, 6, 0]);
        assert!(overflow);
    }

    #[bench]
    fn add_oldschool_u256(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256::zero(), |old, new| { old.overflowing_add(U256::from(new)).0 })
        });
    }

    #[bench]
    fn add_oldschool_u512(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U512([12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64]),
                             |old, new| { old.overflowing_add(
                                 U512([9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64])).0 })
        });
    }

    #[bench]
    fn sub_oldschool_u512(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U512([12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64]),
                             |old, new| { old.overflowing_sub(
                                 U512([9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64])).0 })
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
    fn oldschool_combined(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold(U256([12345u64, 0u64, 0u64, 0u64]), |old, new| {
				let old = old.overflowing_add(U256::from(2*new)).0;
				let old = old.overflowing_sub(U256::from(new)).0;
				old.overflowing_mul(U256::from(new/3)).0
			})
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


    #[bench]
    fn asm_add_512(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64],
                        |old, new| { add_512(old, [9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64]).0 })
        });
    }

    #[bench]
    fn asm_sub_512(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64, 12345u64],
                        |old, new| { sub_512(old, [9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64, 9321u64]).0 })
        });
    }

    #[bench]
    fn asm_combined(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10000);
            (0..n).fold([12345u64, 0u64, 0u64, 0u64], |old, new| {
				let add1 = add ([0, 0, 0, 2 * new], old).0;
				let sub1 = sub (add1, [0, 0, 0, new]).0;
				mul(sub1, [0, 0, 0, new/3]).0
			})
        });
    }

}
