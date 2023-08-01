# maplestory phantom hexacore levelling order calculations

If you're just looking for results, go [here](RESULTS.md).

## quick start
As long as you have Rust installed, you can clone this repo, change the various
values in [src/config.rs](src/config.rs) to your liking, and run:
```
cargo run --release
```
If you want the calculations to be based on Sol Erda rather than Sol Erda
Fragments, you can run the following instead:
```
cargo run --release --features sol_erda
```

## methodology
The logic applied here greedily chooses the next most efficient upgrade at each
step (effectively fd / cost), with some lookahead past the 1/10/20 breakpoints.

The damage calculation is based on a pre-New Age BA, with the damage of the
origin skill calculated from the contribution of Luck of the Draw given in the
BA, and the damage of Tempest VI calculated as a lower effective cooldown +
filling in the remainder of the compressed time with Mille.
