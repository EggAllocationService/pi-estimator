# Pi Estimator

Ever wanted to estimate the value of pi? Now you can!

## Usage

```
./pi-estimator <num_iters>
```

## Algorithm

The algorithm compares the ratio of randomly distributed points in the x,y range (-1, 1) landing inside and outside a circle with radius 1.

The number of points is controlled by the num_iters parameter

## Building
1. Install rust
2. `cargo build --release`
3. Binary is in target/release