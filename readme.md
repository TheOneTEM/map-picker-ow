# Overwatch map picker

A quick-and-dirty map picker for Overwatch, built for [CCLAN](https://cclan.se) and [NymbLAN](https://nymblan.se).

## Map pick rules

* Each round, players choose from a random pool of 9 maps.
* The same game mode cannot be chosen twice in a row unless there are no maps of another game mode available in the pool.
* Picked maps are removed from the pool.

## Install guide

Using [Git](https://git-scm.com/):

```powershell
git clone https://github.com/theonetem/map-picker-ow.git
cd map-picker-ow
```

## Basic usage guide

Using Cargo:

```powershell
cargo run --release -- <your-map-pool.csv>
```

Replace `<your-map-pool.csv>` with `ow-maps.csv` for the NymbLAN map pool as used for NymbLAN 2026.

The program takes in a map pool in CSV format. Make sure that it has >=9 entries and that there are no spaces between the entries in the CSV.

Wrong:

```csv
Hanamura, Assault
```

Correct:

```csv
Hanamura,Assault
```

For the default 

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
