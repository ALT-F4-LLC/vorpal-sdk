# vorpal-sdk

Supported language SDKs for Vorpal.

## Overview

The `vorpal-sdk` is a toolkit designed to facilitate the creation of packages in various programming languages. Currently, the SDK supports Rust, with plans to extend support to Go, TypeScript and more in the near future.

The SDK provides a simple interface for defining a package, including its name, build script, and source path. This is done through the `Package::new` function, which accepts these three parameters. The package can then be built async using the `.package()` method.

In addition to the basic package definition, the SDK also allows for more complex configurations using included builder methods. For example, it's possible to specify additional build packages, the kind of source (e.g., HTTP), a source hash for verification purposes and much more.

## Packages

The following example illustrates how to create a "Hello, World!" package with Vorpal storing an `example.txt` as the output.

> [!NOTE]
> Currently, only `Rust` has an SDK but `Go` and `TypeScript` will be supported soon and examples will be updated to include them.

```rust
use anyhow::Result;
use std::env;
use vorpal_sdk::package::Package;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    Package::new(
        // name
        "example",

        // build script
        "echo \"Hello, World!\" >> $output/example.txt",

        // source path
        &env::current_dir()?.to_string_lossy().to_string(),
    )
    .package()
    .await?;

    Ok(())
}
```

The `Pacakge::new` function accepts three parameters:

- package name
- package build script
- package source uri

Package options can be updated using builder functions like shown for packaging `openssl`:

```rust
Package::new(
    "openssl",
    r#"
        cd openssl-3.3.1
        test -f configure || ./bootstrap
        ./configure \
            --openssldir=$output/ssl \
            --prefix=$output \
            --with-zlib-lib=$zlib/lib
        make -j$(nproc)
        make install
    "#,
    "https://github.com/openssl/openssl/releases/download/openssl-3.3.1/openssl-3.3.1.tar.gz",
)
.with_build_packages(vec![zlib])
.with_source_kind(PackageSourceKind::Http)
.with_source_hash("a53e2254e36124452582477935a680f07f9884fe1d6e9ec03c28ac71b750d84a")
.package()
.await?;
```
