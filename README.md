# CBL experiments

Experiments for the [CBL](https://github.com/imartayan/CBL) paper

You can clone the repository and its submodules with
```sh
git clone --recursive https://github.com/imartayan/CBL_experiments.git
```

If you did not use the `--recursive` flag, make sure to load the submodules with
```sh
git submodule update --init --recursive
```

## Setup

The installation script is written for Debian-based distributions.
If you are using something else, you should change `install_apt_dependencies.sh` to use the package manager of your choice.

If you have not installed Rust yet, please visit [rustup.rs](https://rustup.rs/) to install it.
Then install the latest nightly version with
```sh
rustup install nightly
```

Once this is done, you can build all the tools with
```sh
bash install_all.sh
```

## Running the experiments

The experiments are divided into two parts: building indexes from FASTA files and querying these indexes.
The files to build must be written in `fof_build.txt` (one file per line) and those to query in `fof_query.txt` (one file per line, each linked to the corresponding line in `fof_build.txt`).

You can run all the experiments with
```sh
python3 main.py
```

This will write the results of the experiments in the `data` folder, please leave it untouched if you want to generate plots with them.

## Generating the plots

You can generate all the plots with
```sh
python3 plots.py
```

This will save the plots in the `plot` folder, and create an archive from it.
