use anyhow::Result;
use std::env;
use vorpal_sdk::package::{Package, PackageSourceKind};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let rust_std = Package::new(
        "rust-std",
        "cp -pr rust-std-1.78.0-aarch64-apple-darwin/* $output/.",
        "https://static.rust-lang.org/dist/2024-05-02/rust-std-1.78.0-aarch64-apple-darwin.tar.gz",
    )
    .with_source_kind(PackageSourceKind::Http)
    .with_source_hash("0689a9b2dec87c272954db9212a8f3d5243f55f777f90d84d2b3aeb2aa938ba5")
    .package()
    .await?;

    let rustc = Package::new(
        "rustc",
        r#"
            cp -pr rustc-1.78.0-aarch64-apple-darwin/rustc/* $output/.
            cat $rust_std/rust-std-aarch64-apple-darwin/manifest.in >> $output/manifest.in
            cp -pr $rust_std/rust-std-aarch64-apple-darwin/lib $output
        "#,
        "https://static.rust-lang.org/dist/2024-05-02/rustc-1.78.0-aarch64-apple-darwin.tar.gz",
    )
    .with_build_packages(vec![rust_std])
    .with_source_kind(PackageSourceKind::Http)
    .with_source_hash("1512db881f5bdd7f4bbcfede7f5217bd51ca03dc6741c3577b4d071863690211")
    .package()
    .await?;

    let zlib = Package::new(
        "zlib",
        r#"
            cd zlib-1.3.1
            test -f configure || ./bootstrap
            ./configure --prefix=$output
            make -j$(nproc)
            make install
        "#,
        "https://zlib.net/zlib-1.3.1.tar.gz",
    )
    .with_source_kind(PackageSourceKind::Http)
    .with_source_hash("3f7995d5f103719283f509c23624287ce95c349439e881ed935a3c2c807bb683")
    .package()
    .await?;

    let openssl = Package::new(
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

    let cargo = Package::new(
        "cargo",
        "cp -pr cargo-1.78.0-aarch64-apple-darwin/cargo/* $output/.",
        "https://static.rust-lang.org/dist/2024-05-02/cargo-1.78.0-aarch64-apple-darwin.tar.gz",
    )
    .with_source_kind(PackageSourceKind::Http)
    .with_source_hash("d8ed8e9f5ceefcfe3bca7acd0797ade24eadb17ddccaa319cd00ea290f598d00")
    .package()
    .await?;

    let protoc = Package::new(
        "protoc",
        r#"
            cp -pr ./bin ./include $output/.
            chmod +x $output/bin/protoc
        "#,
        "https://github.com/protocolbuffers/protobuf/releases/download/v27.1/protoc-27.1-osx-aarch_64.zip",
    )
    .with_source_kind(PackageSourceKind::Http)
    .with_source_hash("451209a93ab8c1a3e73210f7ceb7ccf04abec0538782b6e670a497834202846c")
    .package()
    .await?;

    Package::new(
        "vorpal-sdk",
        r#"
            export OPENSSL_DIR=$openssl
            export PROTOC=$protoc/bin/protoc

            cargo check
            cargo build --release

            mkdir -p $output/bin
            cp -pr target/release/vorpal $output/bin/.
        "#,
        &env::current_dir()?.to_string_lossy().to_string(),
    )
    .with_build_packages(vec![cargo, openssl, protoc, rustc])
    .with_source_ignore_paths(vec![
        ".git".to_string(),
        ".gitignore".to_string(),
        "target".to_string(),
    ])
    .package()
    .await?;

    Ok(())
}

pub async fn build_rust_package() {}
