# GroveDB Tutorials

This repository contains a set of tutorials for GroveDB, the first database to enable cryptographic proofs for queries more complex than single-key retrievals. GroveDB was built for Dash Platform but exists as a standalone product, so it may be integrated into other projects as well.

## Tutorials

Each tutorial contains just the example code snippets taken from the in-depth explanatory article found [https://docs.google.com/document/d/1V-h21lX1vovVQYMaPuzNcrYuhhrZVY5WobKdkgxhY-U/edit?usp=sharing]here.

- [Open](open) - Covers how to open a GroveDB instance and perform basic operations.

- [Insert](insert) - Explains how to insert data into GroveDB, including inserting items and subtrees.

- [Delete](delete) - Demonstrates how to delete items and subtrees from GroveDB.

- [Query Simple](query-simple) - Introduces simple querying capabilities in GroveDB, including retrieving a set of items.

- [Query Complex](query-complex) - Covers more advanced querying in GroveDB, including conditional subqueries and sized queries.

- [Proofs](proofs) - Demonstrates how to generate an inclusion proof for a simple query.

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

This project is licensed under the MIT License - see the LICENSE file for details.
