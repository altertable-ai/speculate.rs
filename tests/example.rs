extern crate speculate;
use speculate::speculate;

speculate! {
    const ZERO: i32 = 0;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    describe "math" {
        const ONE: i32 = 1;

        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }

        before {
            let two = ONE + ONE;
        }

        it can_add_stuff {
            assert_eq!(ONE, add(ZERO, ONE));
            assert_eq!(two, add(ONE, ONE));
        }

        it can_subtract_stuff {
            assert_eq!(ZERO, sub(ONE, ONE));
            assert_eq!(ONE, sub(two, ONE));
        }

        context "nested context with additional details" {
            before {
              let three = two + ONE;
            }

            it can_add_stuff_in_nested_context {
                    assert_eq!(three, add(two, ONE));
            }
        }
    }
}
