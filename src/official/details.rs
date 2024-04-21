use crate::official::types::*;
use derive_builder::Builder;
use url;

#[derive(Builder, Debug, PartialEq)]
#[builder(public, setter(into, strip_option))]
pub struct PackageDetailsParam {
    pub repo: Repo,
    pub arch: Arch,
    pub pkgname: String,
}

pub async fn get_package_details(
    param: PackageDetailsParam,
) -> Result<PackageDetails, reqwest::Error> {
    let url = build_url(param);

    let client = reqwest::Client::new();
    let response = client.get(url.to_string()).send().await?;

    let details: PackageDetails = response.json().await?;

    Ok(details)
}

#[tokio::test]
async fn test_get_package_details() {
    let p = PackageDetailsParamBuilder::create_empty()
        .repo(Repo::Core)
        .arch(Arch::X86_64)
        .pkgname("pacman")
        .build()
        .unwrap();

   assert_eq!(
       p,
       PackageDetailsParam {
           repo: Repo::Core,
           arch: Arch::X86_64,
           pkgname: "pacman".to_string(),
       }
   );

    let result = get_package_details(p).await;
    println!("{:?}", result);
}

fn build_url(param: PackageDetailsParam) -> Box<url::Url> {
    let mut u = url::Url::parse("https://archlinux.org/packages").unwrap();

    u.path_segments_mut()
        .unwrap()
        .push(param.repo.to_query_param_value())
        .push(param.arch.to_query_param_value())
        .push(param.pkgname.as_str())
        .push("json");

    Box::new(u)
}

#[test]
fn test_build_url() {
    let p = PackageDetailsParamBuilder::create_empty()
        .repo(Repo::Core)
        .arch(Arch::X86_64)
        .pkgname("coreutils")
        .build()
        .unwrap();

    let url = build_url(p);
    assert_eq!(
        url.to_string(),
        "https://archlinux.org/packages/Core/x86_64/coreutils/json",
    )
}
