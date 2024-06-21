use std::collections::HashMap;
use vorpal::api::{
    ConfigPackageBuild, ConfigPackageOutput, ConfigPackageRequest, ConfigPackageSource,
    ConfigPackageSourceKind,
};

pub fn gmp() -> ConfigPackageRequest {
    let hash = "191226cef6e9ce60e291e178db47682aadea28cb3e92f35f006ba317f3e10195";

    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![],
            sandbox: false,
            script: r#"
                cd gmp-6.3.0
                ./configure --prefix=$output
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "gmp".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(hash.to_string()),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://gmplib.org/download/gmp/gmp-6.3.0.tar.xz".to_string(),
        }),
    }
}

pub fn mpfr(gmp: &ConfigPackageOutput) -> ConfigPackageRequest {
    let hash = "8e3814a6595d335c49b39f5777c6783ba1cd2e57fb3a1696f009b4e5f45f97d4";

    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![gmp.clone()],
            sandbox: false,
            script: r#"
                cd mpfr-4.2.1
                ./configure --prefix=$output --with-gmp=$gmp
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "mpfr".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(hash.to_string()),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://www.mpfr.org/mpfr-current/mpfr-4.2.1.tar.xz".to_string(),
        }),
    }
}

pub fn mpc(gmp: &ConfigPackageOutput, mpfr: &ConfigPackageOutput) -> ConfigPackageRequest {
    let hash = "c179fbcd6e48931a16c0af37d0c4a5e1688dd07d71e2b4a532c68cd5edbb5b72";

    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![gmp.clone(), mpfr.clone()],
            sandbox: false,
            script: r#"
                cd mpc-1.3.1
                ./configure \
                    --prefix=$output \
                    --with-gmp=$gmp \
                    --with-mpfr=$mpfr
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "mpc".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(hash.to_string()),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://ftp.gnu.org/gnu/mpc/mpc-1.3.1.tar.gz".to_string(),
        }),
    }
}

pub fn isl(gmp: &ConfigPackageOutput) -> ConfigPackageRequest {
    let hash = "f5902a68b7bf7aa7c76ba8825ef11835a5ed371e8e871579cbd08219eceba018";

    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![gmp.clone()],
            sandbox: false,
            script: r#"
                cd isl-0.26
                ./configure \
                    --prefix=$output \
                    --with-gmp-prefix=$gmp
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "isl".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(hash.to_string()),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://libisl.sourceforge.io/isl-0.26.tar.xz".to_string(),
        }),
    }
}

pub fn gcc(
    gmp: &ConfigPackageOutput,
    isl: &ConfigPackageOutput,
    mpfr: &ConfigPackageOutput,
    mpc: &ConfigPackageOutput,
) -> ConfigPackageRequest {
    let hash = "0dfc855272326c3d6e300738ab5fd022473aaa2a498efb07d9c45227731b02ef";

    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![gmp.clone(), isl.clone(), mpfr.clone(), mpc.clone()],
            sandbox: false,
            script: r#"
                cd gcc-14.1.0
                test -f configure || ./bootstrap
                ./configure \
                    --prefix=$output \
                    --with-gmp=$gmp \
                    --with-isl=$isl \
                    --with-mpfr=$mpfr \
                    --with-mpc=$mpc
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "gcc".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(hash.to_string()),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://ftp.gnu.org/gnu/gcc/gcc-14.1.0/gcc-14.1.0.tar.xz".to_string(),
        }),
    }
}

pub fn sed() -> ConfigPackageRequest {
    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![],
            sandbox: false,
            script: r#"
                cd sed-4.9
                test -f configure || ./bootstrap
                ./configure --prefix=$output
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "sed".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(
                "434ff552af89340088e0d8cb206c251761297909bbee401176bc8f655e8e7cf2".to_string(),
            ),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://ftp.gnu.org/gnu/sed/sed-4.9.tar.xz".to_string(),
        }),
    }
}

pub fn coreutils(sed: &ConfigPackageOutput) -> ConfigPackageRequest {
    ConfigPackageRequest {
        build: Some(ConfigPackageBuild {
            environment: HashMap::new(),
            packages: vec![sed.clone()],
            sandbox: false,
            script: r#"
                cd coreutils-9.5
                test -f configure || ./bootstrap
                ./configure --prefix=$output
                make -j$(nproc)
                make install
            "#
            .to_string(),
        }),
        name: "coreutils".to_string(),
        source: Some(ConfigPackageSource {
            hash: Some(
                "af6d643afd6241ec35c7781b7f999b97a66c84bea4710ad2bb15e75a5caf11b4".to_string(),
            ),
            ignore_paths: vec![],
            kind: ConfigPackageSourceKind::Http.into(),
            uri: "https://ftp.gnu.org/gnu/coreutils/coreutils-9.5.tar.xz".to_string(),
        }),
    }
}
