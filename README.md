> [!WARNING]
> Для сборки проекта используйте эти команды
#
> rustup toolchain install nightly-2022-11-06-i686-pc-windows-msvc

> cargo build --release
# Для сборки определенного пакета
> cargo build --release --package name

# Также измение в файле ".cargo\config.toml" путь до libcef
>  "-L", "D:\\lib" - указывайте ваш путь до этого файла. Найти его можно в папке "lib" в корне проекта

# SAMP CEF</div>
This project embeds CEF into SA:MP expanding abilities to express yourself with beauty in-game interfaces using HTML / CSS / JavaScript.

Author: **ZOTTCE**
Editor: **Brownield**

Modifications: ``ext package``

## Building
### Build all other the project
```shell
rustup toolchain install nightly-2022-11-06-i686
cargo +nightly-2022-11-06-i686 build --release
```
### Build concrete package
```shell
rustup toolchain install nightly-2022-11-06-i686
cargo +nightly-2022-11-06-i686 build --release --package client --no-default-features
```

## CEF version

Current versions of CEF and Chromium:
`89.0.5+gc1f90d8+chromium-89.0.4389.40` `release branch 4389`

```
Date:             February 26, 2021

CEF Version:      89.0.5+gc1f90d8+chromium-89.0.4389.40
CEF URL:          https://bitbucket.org/chromiumembedded/cef.git
                  @c1f90d8c933dce163b74971707dbd79f00f18219

Chromium Version: 89.0.4389.40
Chromium URL:     https://chromium.googlesource.com/chromium/src.git
                  @2c3400a2b467aa3cf67b4942740db29e60feecb8

```
+ cargo update -p chrono --precise 0.4.30  Если выполнить, возможно пойдет билд
* $env:RUSTC_BOOTSTRAP=1; cargo +nightly-2022-11-06-i686 build --release --ignore-rust-version - powershell cmd
* $env:RUSTC_BOOTSTRAP=1; cargo +nightly-2022-11-06-i686 build --release --package client --no-default-features --ignore-rust-version - powershell cmd
