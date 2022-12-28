# Abort-if crate

The `abort_if` procedural macro guarantees that a specific function panics if a condition is met.

## Installing

Put this in your `Cargo.toml` file:

```
[dependencies]
abort-if = "0.1.2"
```

## Example

You can assure that a function won't be used if feature `x` is enabled

```
use abort_if::abort_if;
#[abort_if(feature = x)]
fn foo() {
	using_that_feature();
}

fn main() {
	foo();
}
```

This code will panic if that feature is enabled.

## Features

The default is panicking using `compiler_error!`. This will output the following information:

```
error: Condition was met.
 --> src/main.rs:5:1
  |
5 | #[abort_if(feature = "x")]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the attribute macro `abort_if` (in Nightly builds, run with -Z macro-backtrace for more info)
```

You can use the feature `custom_abort` to write a custom abort macro. When using this feature, make sure to have a `custom_abort_error!` macro with an `expr` as the argument.

---

If you use the `custom_abort` feature, you can also use the `keep_going` one. This feature functions that, if your `custom_abort_error` macro works as a warning instead of a hard error, the code will keep going.