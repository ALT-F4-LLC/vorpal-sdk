use rsa::pss::SigningKey;
use rsa::sha2::Sha256;
use rsa::signature::RandomizedSigner;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use vorpal::api::package_service_client::PackageServiceClient;
use vorpal::api::{BuildRequest, Package, PrepareRequest};
use vorpal::notary;
use vorpal::store;

pub struct PackageArgs {
    pub build_phase: String,
    pub ignore_paths: Vec<String>,
    pub install_phase: String,
    pub name: String,
    pub source: String,
}

pub fn new(args: PackageArgs) -> Package {
    Package {
        build_phase: args.build_phase,
        ignore_paths: args.ignore_paths,
        install_phase: args.install_phase,
        name: args.name,
        source: args.source,
    }
}

pub async fn run(package: Package) -> Result<(), anyhow::Error> {
    let package_dir = store::get_package_dir();
    if !package_dir.exists() {
        fs::create_dir_all(&package_dir)?;
    }

    println!("Preparing: {:?}", package.name);

    let source_id = prepare(package_dir, package.clone()).await?;

    println!("Building: {:?}-{:?}", package.name, source_id);

    let _ = build(source_id, package.clone()).await?;

    // TODO: build method

    // TODO status method

    // TODO: retrieve method

    Ok(())
}

async fn prepare(package_dir: PathBuf, package: Package) -> Result<String, anyhow::Error> {
    let source = Path::new(&package.source).canonicalize()?;
    let source_ignore_paths = package
        .ignore_paths
        .iter()
        .map(|i| Path::new(i).to_path_buf())
        .collect();
    let source_files = store::get_file_paths(&source, source_ignore_paths)?;
    let source_files_hashes = store::get_file_hashes(source_files.clone())?;
    let source_hash = store::get_source_hash(source_files_hashes.clone())?;
    let source_dir_name = store::get_package_dir_name(&package.name, &source_hash);
    let source_dir = package_dir.join(&source_dir_name).with_extension("source");
    let source_tar = source_dir.with_extension("source.tar.gz");
    if !source_tar.exists() {
        println!("Creating source tar: {:?}", source_tar);
        store::compress_files(source.clone(), source_tar.clone(), source_files.clone())?;
        fs::set_permissions(&source_tar, fs::Permissions::from_mode(0o444))?;
    }

    println!("Source file: {:?}", source_tar);

    let private_key = match notary::get_private_key() {
        Ok(key) => key,
        Err(e) => anyhow::bail!("Failed to get private key: {:?}", e),
    };
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut signing_rng = rand::thread_rng();
    let source_data = fs::read(&source_tar)?;
    let source_signature = signing_key.sign_with_rng(&mut signing_rng, &source_data);

    println!("Source signature: {:?}", source_signature);

    let request = tonic::Request::new(PrepareRequest {
        source_data,
        source_hash,
        source_name: package.name,
        source_signature: source_signature.to_string(),
    });
    let mut client = PackageServiceClient::connect("http://[::1]:15323").await?;
    let response = client.prepare(request).await?;
    let response_data = response.into_inner();

    Ok(response_data.source_id)
}

async fn build(package_id: String, package: Package) -> Result<Vec<u8>, anyhow::Error> {
    let request = tonic::Request::new(BuildRequest {
        build_phase: package.build_phase,
        install_phase: package.install_phase,
        source_id: package_id.clone(),
    });
    let mut client = PackageServiceClient::connect("http://[::1]:15323").await?;
    let response = client.build(request).await?;
    let response_data = response.into_inner();

    Ok(response_data.data)
}
