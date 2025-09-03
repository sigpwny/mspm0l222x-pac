# mspm0l222x-pac

[![Crates.io Version](https://img.shields.io/crates/v/mspm0l222x-pac)](https://crates.io/crates/mspm0l222x-pac)
[![docs.rs](https://img.shields.io/docsrs/mspm0l222x-pac)](https://docs.rs/mspm0l222x-pac)

This is a [Peripheral Access Crate](https://docs.rust-embedded.org/book/start/registers.html) for the Texas Instruments [MSPM0L2228](https://www.ti.com/product/MSPM0L2228) microcontroller.

This crate is automatically generated from the SVD file found in the [Arm Keil CMSIS Pack](https://www.keil.arm.com/devices/texas-instruments-mspm0l2228/boards/) for the MSPM0L222X family (revision 1.1.1). [svd2rust](https://github.com/rust-embedded/svd2rust) is used to automatically generate the PAC from the SVD file. The provided SVD file also has a few mistakes, which are patched using [svdtools](https://github.com/rust-embedded/svdtools).

## Building

> [!NOTE]  
> This section is only relevant if you need to rebuild the crate. Generated code is already included in the repository.

First, make sure you have Rust installed (via [rustup](https://rustup.rs/)). Then, install the following development dependencies.

```bash
cargo install svd2rust svdtools form
rustup component add --toolchain nightly rustfmt
```

Simply run `make` to generate the crate.

```bash
make
```

## License
This crate is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Copyright (c) 2025 SIGPwny

The [MSPM0L222X.svd](./svd/MSPM0L222X.svd) file is licensed under the [BSD-3-Clause](./svd/LICENSE) license ([SPDX](./svd/MSPM0L222X.svd.spdx)).

Copyright (c) 2023 Texas Instruments Incorporated