# speculate.rs [![Build Status](https://github.com/altertable-ai/speculate.rs/workflows/CI/badge.svg)](https://github.com/altertable-ai/speculate.rs/actions)

> An RSpec inspired minimal testing framework for Rust.

This is a fork of the original [speculate.rs](https://github.com/utkarshkukreti/speculate.rs) by [@utkarshkukreti](https://github.com/utkarshkukreti), now maintained by the [Altertable.ai](https://altertable.ai) team.

## Installation

Since this crate is not yet published to crates.io, add `speculate` to the `dev-dependencies` section of your `Cargo.toml` using the GitHub repository:

```toml
[dev-dependencies]
speculate = { git = "https://github.com/altertable-ai/speculate.rs" }
```

And add the following to the top of the Rust file you want to add tests for:

```rust
#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;  // Must be imported into the current scope.
```

## Usage

Speculate provides the `speculate!` syntax extension.
Inside `speculate! { ... }`, you can have any "Item", like `static`, `const`,
`fn`, etc, and 5 special types of blocks:

- `describe` (or its alias `context`) - to group tests in a hierarchy, for
  readability. Can be arbitrarily nested.

- `before` - contains setup code that's inserted before every sibling and nested
  `it` blocks.

- `after` - contains teardown code that's inserted after every sibling and
  nested `it` blocks.

- `it` (or its alias `test`) - contains tests.

  For example:

  ```rust
  it can_add_1_and_2 {
    assert_eq!(1 + 2, 3);
  }
  ```

  You can optionally add attributes to this block:

  ```rust
  #[ignore]
  test ignore {
      assert_eq!(1, 2);
  }

  #[should_panic]
  test should_panic {
      assert_eq!(1, 2);
  }

  #[should_panic(expected = "foo")]
  test should_panic_with_foo {
      panic!("foo");
  }
  ```

## Complete Example (from `tests/example.rs`)

```rust
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
```

## Code Formatting

**Important:** Rust's `rustfmt` cannot automatically format code inside procedural macro invocations like `speculate!`. This is a known limitation - rustfmt only formats inside declarative macros (`macro_rules!`), not procedural macros.

Therefore, you must **manually ensure proper indentation** inside `speculate!` blocks:

- Use 4 spaces for indentation (no tabs)
- Maintain consistent indentation levels
- Follow the examples in the README and test files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Code Style Requirements

When contributing, please ensure:

1. **Indentation inside `speculate!` blocks**: Use exactly 4 spaces (no tabs)

   - This cannot be automatically checked by rustfmt
   - Review your code carefully before submitting
   - Follow the indentation pattern in existing test files

2. **Run `cargo +nightly fmt`** before committing to format code outside macros

3. **All tests pass**: Run `cargo test` to verify

4. **No clippy warnings**: Run `cargo clippy` to check for issues

The CI will check formatting and run tests, but **manual indentation inside macros must be verified by the contributor**.

## License

MIT License - see the [LICENSE](LICENSE) file for details.
