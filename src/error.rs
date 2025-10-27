#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("missing expected field `{0}`")]
	MissingField(String),
	#[error("HTTP error")]
	Reqwest(#[from] reqwest::Error),
	#[error("failed to execute task")]
	JoinError(#[from] tokio::task::JoinError),
	#[error("RSS error")]
	RSS(#[from] rss::Error),
	#[error("missing title")]
	MissingTitle,
	#[error("missing size")]
	MissingSize,
	#[error("missing link")]
	MissingLink,
	#[error("empty extension `{0}`")]
	EmptyExtension(String),
	#[error("failed to parse integer")]
	ParseInt(#[from] std::num::ParseIntError),
	#[error("failed to parse float")]
	ParseFloat(#[from] std::num::ParseFloatError),
	#[cfg(feature = "require-parse-names")]
	#[error("failed to parse torrent name")]
	ParseTorrentName(#[from] torrent_name_parser::error::ErrorMatch)
}

