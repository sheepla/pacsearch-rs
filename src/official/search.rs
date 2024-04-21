mod types;

use crate::types::*;
use reqwest;
use std::collections::HashMap;

pub async fn search_packages(
    param: PackageSearchParam,
) -> Result<PackageSearchResult, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://archlinux.org/packages/search/json/")
        .query(&build_query_param(param))
        .send()
        .await?;

    let search_result: PackageSearchResult = response.json().await?;
    Ok(search_result)
}

fn build_query_param(param: PackageSearchParam) -> HashMap<String, String> {
    let mut q = HashMap::new();

    if let Some(query) = param.query {
        q.insert("q".to_string(), query);
    }

    if let Some(name) = param.name {
        q.insert("name".to_string(), name);
    }

    if let Some(description) = param.description {
        q.insert("description".to_string(), description);
    }

    if let Some(repo) = param.repo {
        q.insert("repo".to_string(), repo.to_query_param_value().to_string());
    }

    if let Some(arch) = param.arch {
        q.insert("arch".to_string(), arch.to_query_param_value().to_string());
    }

    if let Some(maintainer) = param.maintainer {
        q.insert("maintainer".to_string(), maintainer);
    }

    if let Some(packager) = param.packager {
        q.insert("packager".to_string(), packager);
    }

    if let Some(flagged) = param.flagged {
        q.insert("flagged".to_string(), flagged.to_query_param_value().to_string());
    }

    q
}

#[tokio::test]
async fn test_search_packages() {
    let param = PackageSearchParamBuilder::default()
        .query("pacman")
        .repo(Repo::Core)
        .build()
        .unwrap();

    println!("param: {:?}", param);

    let packages = search_packages(param).await.unwrap();

    println!("------------");
    for pkg in &packages.results {
        println!("{}/{} v{} - {}", pkg.repo, pkg.pkgname, pkg.pkgver, pkg.pkgdesc);
    }
    println!("------------");
}
