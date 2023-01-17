use derive_more::{Display, Error};

#[derive(Debug, Error, Display)]
pub enum RepoError {
	#[display(fmt = "Repo Post Category Error: Generic")]
	Generic,
}
