use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub snapshot_paths: SnapshotPaths,
}

#[derive(Deserialize, Debug)]
pub struct SnapshotPaths {
    pub overview: String,
}
