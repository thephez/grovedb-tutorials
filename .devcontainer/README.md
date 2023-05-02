# Development Container

The development container config ([.devcontainer.json](./devcontainer.json)) enables starting a
pre-configured [GitHub Codespaces](https://github.com/features/codespaces) environment ready to
build and run the tutorials in this repository.

## Building the tutorials

Note: it currently takes a long time (5+ minutes) to start a new Codespace for this project
depending on the machine type selected. Only 4+ core machine types are supported due to the
resources required to build GroveDB.

Once the Codespace has started and finished running rust-analyzer, run the following command
to build everything required. This may take an additional 10 minutes depending on the
Codespace machine selected:

```shell
cargo build
```

Note: you may need go to the main menu, click `View`, then `Terminal` if the terminal isn't visible.

## Running the tutorials

To run the tutorials, follow the same instructions as found in the main [README file](../README.md). For example:

```shell
cargo run --bin open
```
