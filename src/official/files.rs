use crate::official::types::*;

use derive_builder::Builder;

#[derive(Builder, Debug, PartialEq)]
#[builder(public, setter(into, strip_option))]
pub struct PackageInternalFilesParam {
    pub repo: Repo,
    pub arch: Arch,
    pub pkgname: String,
}

pub async fn get_package_internal_files(
    param: PackageInternalFilesParam,
) -> Result<PackageInternalFiles, reqwest::Error> {
    let url = build_url(param);

    let client = reqwest::Client::new();
    let response = client.get(url.to_string()).send().await?;

    let details: PackageInternalFiles = response.json().await?;

    Ok(details)
}

#[tokio::test]
async fn test_get_package_details() {
    let p = PackageInternalFilesParamBuilder::create_empty()
        .repo(Repo::Core)
        .arch(Arch::X86_64)
        .pkgname("pacman")
        .build()
        .unwrap();

    assert_eq!(
        p,
        PackageInternalFilesParam {
            repo: Repo::Core,
            arch: Arch::X86_64,
            pkgname: "pacman".to_string(),
        }
    );

    let result = get_package_internal_files(p).await;
    println!("{:?}", result);
}

fn build_url(param: PackageInternalFilesParam) -> Box<url::Url> {
    let mut u = url::Url::parse("https://archlinux.org/packages").unwrap();

    u.path_segments_mut()
        .unwrap()
        .push(param.repo.to_query_param_value())
        .push(param.arch.to_query_param_value())
        .push(param.pkgname.as_str())
        .push("files")
        .push("json");

    Box::new(u)
}

#[test]
fn test_build_url() {
    let p = PackageInternalFilesParamBuilder::create_empty()
        .repo(Repo::Core)
        .arch(Arch::X86_64)
        .pkgname("coreutils")
        .build()
        .unwrap();

    let url = build_url(p);
    assert_eq!(
        url.to_string(),
        "https://archlinux.org/packages/Core/x86_64/coreutils/files/json",
    )
}
