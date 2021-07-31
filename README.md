# Moore neighborhoods (8-connected) in _N_ dimensions

A little `no_std` capable library for generating indexes for [Moore neighborhoods]
(i.e., the surrounding cells of a single cell in a grid) of arbitrary range and dimensions.
Or, the red edge squares for the blue center square:

[![Moore neighborhood](docs/moore.png)](https://en.wikipedia.org/wiki/File:Moore_neighborhood_with_cardinal_directions.svg)

Here is what the `moore!` macro generates for the above example, a 2-dimensional
grid with a neighborhood size (range) of `1`.

```rust
fn example() {
    let neighborhood = moore!(1, 2);

    assert_eq!(neighborhood, [
        [-1,-1], [ 0,-1], [ 1,-1],
        [-1, 0],          [ 1, 0],
        [-1, 1], [ 0, 1], [ 1, 1]
    ]);
}
```

Please see more usage examples below.

## Using the library with `std` and `no_std`

To use the library in a regular environment, add the following to your `Cargo.toml`:

```toml
[dependencies]
moore-neighborhoods = "0.2"
```

To use the library in a `no_std` environment, use the following instead:

```toml
[dependencies]
moore-neighborhoods = { version = "0.2", default-features = false }
```

This will disable all `Vec` based functions, i.e., you'll want to use the `moore!` macro
to obtain the indexes.

[Moore neighborhoods]: https://en.wikipedia.org/wiki/Moore_neighborhood

## Usage example

Using the `moore!` macro, when both the dimensionality and range are statically known:

```rust
use moore_neighboorhood::moore;

fn example() {
    let mut result: [[isize; 2]; 8] = moore!(1, 2);

    let mut expected = [
        [-1,-1], [ 0,-1], [ 1,-1],
        [-1, 0],          [ 1, 0],
        [-1, 1], [ 0, 1], [ 1, 1]
    ];

    assert_eq!(result, expected);
}
```

When dynamic sizes are required:

```rust
use moore_neighboorhood::dynamic::moore;

fn example() {
    let mut result: Vec<Vec<isize>> = moore(1, 2);
    
    let mut expected = [
        [-1,-1], [ 0,-1], [ 1,-1],
        [-1, 0],          [ 1, 0],
        [-1, 1], [ 0, 1], [ 1, 1]
    ];

    result.sort();
    expected.sort();
    assert_eq!(result, expected);
}
```

When the dimensionality is statically known but the range may change:

```rust
use moore_neighboorhood::generic_dimension::moore;

fn example() {
    let mut result: Vec<[isize; 2]> = moore(1);
    
    let mut expected = [
        [-1,-1], [ 0,-1], [ 1,-1],
        [-1, 0],          [ 1, 0],
        [-1, 1], [ 0, 1], [ 1, 1]
    ];

    result.sort();
    expected.sort();
    assert_eq!(result, expected);
}
```

## License

The project is licensed under an MIT license (see the [LICENSE] file for more information).
The code was ported from [hughsk/moore] written by Hugh Kennedy and adjusted for Rust features such as const generics.

[LICENSE]: LICENSE
[hughsk/moore]: https://github.com/hughsk/moore
