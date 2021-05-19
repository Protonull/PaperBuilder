use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct PaperAllVersions {
    pub versions: Vec<String>
}

#[derive(Deserialize, Debug)]
pub(crate) struct PaperAllVersionBuilds {
    pub builds: Vec<u32>
}

#[derive(Deserialize, Debug)]
pub(crate) struct PaperVersionBuild {
    pub changes: Vec<PaperVersionBuildChange>
}

#[derive(Deserialize, Debug)]
pub(crate) struct PaperVersionBuildChange {
    pub commit: String
}
