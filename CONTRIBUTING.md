# contributing to rebox

most likely, where you want to work is either in the `coreutils` directory, or in the `utils` directory. the `coreutils` directory contains the coreutils remake, and the `utils` directory contains `boxutils`: utilities and functions for developing rebox without much jank.

if you want to add a new command, you should:

1. add a new file in `coreutils/src/commands` with the name of the command you want to add.
2. add a new module in `coreutils/src/commands/mod.rs` with the name of the command you want to add.
3. add a new function in the module you just created that implements the `Command` trait.
4. add the command to the registry via `src/registry.rs`.
5. done!