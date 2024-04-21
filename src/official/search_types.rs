use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Builder, Default, Debug)]
#[builder(default, public, setter(into, strip_option))]
pub struct PackageSearchParam {
    pub query: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub repo: Option<Repo>,
    pub arch: Option<Arch>,
    pub maintainer: Option<String>,
    pub packager: Option<String>,
    pub flagged: Option<IsFlagged>,
}

#[test]
fn test_package_search_param_builder() {
    let b = PackageSearchParamBuilder::default()
        .query("query")
        .name("name")
        .repo(Repo::Core)
        .arch(Arch::Any)
        .flagged(IsFlagged::NotFlagged)
        .build();

    println!("PackageSearchParamBuilder: {:?}", b);

    b.unwrap();
}

#[derive(Clone, Debug)]
pub enum Repo {
    Core,
    CoreTesting,
    Extra,
    ExtraTesting,
    Multilib,
    MultilibTesting,
}

impl Repo {
    pub fn to_query_param_value(&self) -> &str {
        match self {
            Self::Core => "Core",
            Self::CoreTesting => "Core-Testing",
            Self::Extra => "Extra",
            Self::ExtraTesting => "Extra-Testing",
            Self::Multilib => "Multilib",
            Self::MultilibTesting => "Multilib-Testing",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Arch {
    Any,
    X86_64,
}

impl Arch {
    pub fn to_query_param_value(&self) -> &str {
        match self {
            Self::Any => "any",
            Self::X86_64 => "X86_64",
        }
    }
}

#[derive(Clone, Debug)]
pub enum IsFlagged {
    Flagged,
    NotFlagged,
}

impl IsFlagged {
    pub fn to_query_param_value(&self) -> &str {
        match self {
            Self::Flagged => "Flagged",
            Self::NotFlagged => "Not+Flagged",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageSearchResult {
    pub version: i64,
    pub limit: i64,
    pub valid: bool,
    pub results: Vec<Package>,
    pub num_pages: i64,
    pub page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "result")]
pub struct Package {
    pub pkgname: String,
    pub pkgbase: String,
    pub repo: String,
    pub arch: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub epoch: i64,
    pub pkgdesc: String,
    pub url: String,
    pub filename: String,
    pub compressed_size: i64,
    pub installed_size: i64,
    pub build_date: String,
    pub last_update: String,
    pub flag_date: Option<String>,
    pub maintainers: Vec<String>,
    pub packager: String,
    pub groups: Vec<String>,
    pub licenses: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<Value>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
}
