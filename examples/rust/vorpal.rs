use anyhow::Result;
use vorpal_sdk::package;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let example = package::new(package::PackageArgs {
        build_phase: r#"
            echo "hello, world!" >> example.txt
            cat example.txt
        "#,
        ignore_paths: vec![".direnv", ".git", "target"],
        install_phase: r#"
            mkdir -p $OUTPUT
            cp example.txt $OUTPUT/example.txt
        "#,
        name: "example",
        source: ".",
    });

    package::run(&example).await
}
