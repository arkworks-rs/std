## Pending
- [\#47](https://github.com/arkworks-rs/std/pull/47) Fix incorrect use of atomic variable in `src/perf_trace.rs`

### Breaking changes

### Features

### Improvements

### Bug fixes

## v0.4.0

### Breaking changes

- [\#35](https://github.com/arkworks-rs/utils/pull/35) Change `test_rng` to return `impl Rng`, and make the output randomized by default when the `std` feature is set. Introduces a `DETERMINISTIC_TEST_RNG` environment variable that forces the old deterministic behavior when `DETERMINISTIC_TEST_RNG=1` is set.

### Features

### Improvements

### Bug fixes

## v0.3.0

### Breaking changes

- [\#32](https://github.com/arkworks-rs/utils/pull/32) Bump `rand` to 0.8 and remove the use of `rand_xorshift`.

### Features

- [\#34](https://github.com/arkworks-rs/utils/pull/34) Re-export `num_traits::{One, Zero}` from `ark-std`.

### Improvements

### Bug fixes
