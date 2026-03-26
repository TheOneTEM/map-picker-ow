# OW map picker

A quick-and-dirty map pick built for [CCLAN](https://cclan.se) and [NymbLAN](https://nymblan.se).

## Map pick rules

* Each round, players choose from a random pool of 9 maps.
* The same game mode cannot be chosen twice in a row unless there are no maps of another game mode available in the pool.
* Picked maps are removed from the pool.

## Usage guide

Using Cargo:

```powershell
cargo run --release -- <your-map-pool.csv>
```

The program takes in a map pool in CSV format. Make sure that it has >=9 entries.

## Map pool

Use `ow-maps.csv` as an example when creating your own map pool.

The default `ow-maps.csv` includes the following maps:

* All maps playable in Overwatch Competitive
* All maps playable in the Assault Maps arcade game mode
* Paris
* Horizon Lunar Colony

The following maps are excluded:

* Deathmatch maps
* Stadium-only maps
* Antarctic Peninsula
