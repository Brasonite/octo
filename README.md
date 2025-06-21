# Octo

Octo is a simple program for blazingly fast correction of Python olympiad questions.

## Usage

> **NOTICE:** This program was built with [OBI (the Brazilian Olympiad of Informatics)](https://olimpiada.ic.unicamp.br/) in mind, but should work as long as questions follow the format described below.

In order to use the program, you will first need to create a question manifest. Questions are defined using the format described in the [`preset.toml`](./preset.toml) file. Keep in mind `solver`, `input_folders` and `output_folders` are relative to the defined `root`.

Another important thing to remember is that the filenames of a, input and its respective output must be equal, minus the extension. For example, following the file formats in `preset.toml`, if you have an input file called `3.in`, it will attempt to search for an output file called `3.sol`. Tests with missing outputs are automatically considered failures.

Once the manifest is defined and the inputs and outputs are in place, one can execute the program using the following command:

```Shell
octo --question "PATH_TO_QUESTION_MANIFEST"
```

Or, if running from Cargo:

```Shell
cargo run --release -- --question "PATH_TO_QUESTION_MANIFEST"
```

Alternatively, one can replace the `--question` argument with it's shortcut `-q`.

## Requirements

This program was built using stable Rust (2024 edition). Therefore, a compatible compiler version is necessary if you wish to compile from source. Aside from that, there should be no extra compile-time dependencies.

As for the runtime dependencies, Octo relies on the system's Python 3 runtime. Therefore, it is necessary to install Python 3.0 or above, ensuring it is in the system's `PATH` variable or equivalent.