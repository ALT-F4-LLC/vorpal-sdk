use log::{info, trace};
use std::collections::HashMap;
use std::env;
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use vorpal::api::config_service_client::ConfigServiceClient;
use vorpal::api::{
    ConfigPackageBuild, ConfigPackageOutput, ConfigPackageRequest, ConfigPackageSource,
    ConfigPackageSourceKind,
};

mod package_sandbox;

pub struct Agent {
    pub config_host: String,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            config_host: "http://[::1]:15323".to_string(),
        }
    }

    pub async fn with_config_host(mut self: Agent, host: &str) -> Agent {
        self.config_host = host.to_string();
        self
    }

    pub async fn connect(self) -> Result<ConfigServiceClient<Channel>, anyhow::Error> {
        Ok(ConfigServiceClient::connect(self.config_host.clone()).await?)
    }
}

pub struct PackageBuild {
    pub environment: HashMap<String, String>,
    pub packages: Vec<ConfigPackageOutput>,
    pub sandbox: Option<bool>,
    pub script: String,
}

pub struct PackageSource {
    pub kind: ConfigPackageSourceKind,
    pub hash: Option<String>,
    pub ignore_paths: Vec<String>,
    pub uri: String,
}

pub struct Package {
    pub agent: Agent,
    pub build: PackageBuild,
    pub name: String,
    pub source: PackageSource,
}

pub enum PackageSourceKind {
    Local,
    Http,
    Git,
}

impl Package {
    pub fn new(name: &str, build_script: &str, source_uri: &str) -> Package {
        let agent = Agent::new();

        Package {
            agent,
            build: PackageBuild {
                environment: HashMap::new(),
                sandbox: Some(true),
                packages: vec![],
                script: build_script.to_string(),
            },
            name: name.to_string(),
            source: PackageSource {
                hash: None,
                ignore_paths: vec![],
                kind: ConfigPackageSourceKind::Local,
                uri: source_uri.to_string(),
            },
        }
    }

    pub fn with_agent(mut self: Package, agent: Agent) -> Package {
        self.agent = agent;
        self
    }

    pub fn with_build_environment(mut self: Package, key: &str, value: &str) -> Package {
        self.build
            .environment
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_build_packages(mut self: Package, packages: Vec<ConfigPackageOutput>) -> Package {
        for p in packages {
            self.build.packages.push(p);
        }
        self
    }

    pub fn with_build_sandbox(mut self: Package, sandbox: bool) -> Package {
        self.build.sandbox = Some(sandbox);
        self
    }

    pub fn with_source_hash(mut self: Package, hash: &str) -> Package {
        self.source.hash = Some(hash.to_string());
        self
    }

    pub fn with_source_kind(mut self: Package, kind: PackageSourceKind) -> Package {
        self.source.kind = match kind {
            PackageSourceKind::Git => ConfigPackageSourceKind::Git,
            PackageSourceKind::Http => ConfigPackageSourceKind::Http,
            PackageSourceKind::Local => ConfigPackageSourceKind::Local,
        };
        self
    }

    pub fn with_source_ignore_paths(mut self: Package, paths: Vec<String>) -> Package {
        self.source.ignore_paths = paths;
        self
    }

    pub async fn package(self: Package) -> Result<ConfigPackageOutput, anyhow::Error> {
        trace!("package");

        let mut client = self.agent.connect().await?;

        let mut build_packages = vec![];

        match env::consts::OS {
            "linux" => {
                let gmp = package_stream(&mut client, package_sandbox::gmp()).await?;
                let sed = package_stream(&mut client, package_sandbox::sed()).await?;
                let isl = package_stream(&mut client, package_sandbox::isl(&gmp)).await?;
                let mpfr = package_stream(&mut client, package_sandbox::mpfr(&gmp)).await?;
                let mpc = package_stream(&mut client, package_sandbox::mpc(&gmp, &mpfr)).await?;
                let gcc =
                    package_stream(&mut client, package_sandbox::gcc(&gmp, &isl, &mpfr, &mpc))
                        .await?;
                let coreutils =
                    package_stream(&mut client, package_sandbox::coreutils(&sed)).await?;

                build_packages.push(gmp);
                build_packages.push(isl);
                build_packages.push(mpfr);
                build_packages.push(mpc);
                build_packages.push(gcc);
                build_packages.push(sed);
                build_packages.push(coreutils);
            }
            "macos" => {}
            _ => {}
        }

        for p in self.build.packages {
            build_packages.push(p);
        }

        info!("package build packages: {:?}", build_packages);

        let config = ConfigPackageRequest {
            build: Some(ConfigPackageBuild {
                environment: self.build.environment,
                packages: build_packages,
                sandbox: self.build.sandbox.unwrap(),
                script: self.build.script,
            }),
            name: self.name,
            source: Some(ConfigPackageSource {
                hash: self.source.hash,
                ignore_paths: self.source.ignore_paths,
                kind: self.source.kind.into(),
                uri: self.source.uri,
            }),
        };

        package_stream(&mut client, config).await
    }
}

async fn package_stream(
    client: &mut ConfigServiceClient<Channel>,
    config: ConfigPackageRequest,
) -> Result<ConfigPackageOutput, anyhow::Error> {
    let package_response = client.package(config).await?;
    let mut package_hash = "".to_string();
    let mut package_name = "".to_string();
    let mut package_stream = package_response.into_inner();

    while let Some(response) = package_stream.next().await {
        if let Err(e) = response {
            return Err(anyhow::anyhow!("gRPC error: {:?}", e));
        }

        let res = response.unwrap();

        if !res.log_output.is_empty() {
            let log_output = String::from_utf8_lossy(&res.log_output).into_owned();
            info!("{}", log_output);
        }

        if let Some(output) = res.package_output {
            package_hash = output.hash;
            package_name = output.name;
        }
    }

    if package_hash.is_empty() {
        return Err(anyhow::anyhow!("No package hash returned"));
    }

    if package_name.is_empty() {
        return Err(anyhow::anyhow!("No package name returned"));
    }

    Ok(ConfigPackageOutput {
        hash: package_hash,
        name: package_name,
    })
}
