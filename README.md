# GroveDB Tutorials

This repository contains a set of tutorials for GroveDB, the first database to enable cryptographic proofs for queries more complex than single-key retrievals. GroveDB was built for [Dash Platform](https://www.dash.org/platform/) but exists as a standalone product, so it may be integrated into other projects as well.

## Tutorials

Each tutorial contains just the commented code snippets taken from the in-depth explanatory article that can be found [here](https://docs.google.com/document/d/1V-h21lX1vovVQYMaPuzNcrYuhhrZVY5WobKdkgxhY-U/edit?usp=sharing).

- [Open](tutorials/src/bin/open.rs) - Covers how to open a GroveDB instance and perform basic operations.

- [Insert](tutorials/src/bin/insert.rs) - Explains how to insert data into GroveDB, including inserting items and subtrees.

- [Delete](tutorials/src/bin/delete.rs) - Demonstrates how to delete items and subtrees from GroveDB.

- [Query Simple](tutorials/src/bin/query-simple.rs) - Introduces simple querying capabilities in GroveDB, including retrieving a set of items.

- [Query Complex](tutorials/src/bin/query-complex.rs) - Covers more advanced querying in GroveDB, including conditional subqueries and sized queries.

- [Proofs](tutorials/src/bin/proofs.rs) - Demonstrates how to generate an inclusion proof for a simple query.

## Getting Started

To get started with GroveDB, follow these steps:

1. Clone this repository to your local machine:

   ```shell
   git clone https://github.com/thephez/grovedb-tutorials.git
   ```
   
2. Navigate to the tutorials folder within the repo and build:

   ```shell
   cd grovedb-tutorials/tutorials
   cargo build
   ```

3. Run a tutorial:

   ```shell
   cargo run --bin <tutorial name>
   ```
   
   Where valid tutorial names are: open, insert, delete, query-simple, query-complex, proofs

## Contributing

Contributions from the community are always welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE file](LICENSE.md) for details.
