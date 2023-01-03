# Selectoor

Selectoor is a CLI tool that generates optimal function names so that the solidity function selector has a good number of zero bytes. This helps reduce the gas cost of calling the function due to cheaper zero byte calldata.

![selectoor](/docs/example.png?raw=true)

## Installation

To install this tool, first clone the repository.

```shell
git clone git@github.com:AdithyaNarayan/selectoor.git
```

Install the tool globally using `cargo`.

```shell
cargo install --path selectoor
```

## Usage

After installation, the binary will be available as `selectoor`.

Pull up the help screen with `--help`

```shell
selectoor --help
selectoor generate --help
```

Currently, the tool supports providing human-readable abi or function signature using the `-s` flag and the generated ABI JSON artifact using the `-a` flag.

```shell
selectoor generate -s "transfer(address _to, uint256 _amount)"
```

The CLI will automatically strip the variable names and generate a list of function signatures with optimal selectors.

The CLI does this by generating prefixes of fixed length to the function name, so as to not compromise on readability while maintaining low gas cost. Example: `transfer(address,uint256)` -> `transfer_XXXX(address,uint256)`

## Features

-   [x] Generate signature with suffixes 
-   [x] Parse human-readable signatures 
-   [x] Read abi file and generate optimal signatures for all the external functions
-   [ ] Stop searching when the first matching signature is found
-   [ ] `selectoor fmt` that modifies all `*.sol` files and replaces the function singatures

Please feel free to raise a pull request or issues for more features.
