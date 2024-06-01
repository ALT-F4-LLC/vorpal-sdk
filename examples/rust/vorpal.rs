use anyhow::Result;
use vorpal_sdk::package;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let example = package::new(package::PackageArgs {
        build_phase: r#"
            echo "hello, world!" >> example.txt
        "#
        .to_string(),
        ignore_paths: vec![
            ".direnv".to_string(),
            ".git".to_string(),
            "target".to_string(),
        ],
        install_phase: r#"
            cp example.txt $output
        "#
        .to_string(),
        name: "example".to_string(),
        source: ".".to_string(),
    });

    package::run(example).await
}
