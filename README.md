# apollo3-pac

Peripheral access crate for the Ambiq Apollo3 Blue microcontroller.

This crate is generated using [`chiptool`](https://github.com/embassy-rs/chiptool) from the manufacturer's SVD file.

## Regenerating the PAC

Due to licensing restrictions, the original Ambiq Apollo3 `.svd` file is **not distributed** in this repository. However, the custom SVD transformations (patches) used to clean up the register definitions are included in the `svd/ambiq.yaml` file.

To regenerate this crate yourself:

1. Download the Apollo3 SDK from Ambiq.
2. Locate the `apollo3.svd` file within the SDK.
3. Place the `apollo3.svd` file in the `svd/` directory of this crate.
4. Run the `./generate-pac.sh` script (requires `chiptool` and `form`):

```bash
./generate-pac.sh
```

This will apply all the patches defined in `svd/ambiq.yaml` to the base SVD and generate the Rust source files in the `src/` directory.

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
