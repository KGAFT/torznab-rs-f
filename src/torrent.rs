extern crate rss;

use core::str::FromStr;
use std::collections::BTreeMap;
use std::time::Duration;

use rss::extension::Extension;
use rss::Item;

use crate::Error;
use crate::Torrent;

fn get_extension_value<'a>(ext: &'a BTreeMap<String, Vec<Extension>>, key: &str) -> Result<Option<&'a str>, Error> {
	let ext = match ext.get("attr") {
		Some(v) => v,
		None => return Ok(None)
	};
	if(ext.is_empty()) {
		return Err(Error::EmptyExtension(key.to_string()))
	}
	for extension in ext.iter() {
		if let Some(name) = extension.attrs().get("name") {
			if let Some(value) = extension.attrs().get("value") {
				if(name == key) {
					return Ok(Some(value));
				}
			}
		}
	}
	Ok(None)
}

fn get_parsed_extension_value<T>(ext: &BTreeMap<String, Vec<Extension>>, key: &str) -> Result<Option<T>, Error>
where
	T: FromStr,
	Error: From<T::Err>
{
	match get_extension_value(ext, key)? {
		Some(v) => Ok(Some(v.parse::<T>()?)),
		None => Ok(None)
	}
}

pub fn from_item(item: Item) -> Result<Torrent, Error> {
	let name = item.title().ok_or(Error::MissingTitle)?.to_string();
	let size = item.enclosure().ok_or(Error::MissingSize)?.length().parse()?;
	let categories = item.categories().iter().map(|category| category.name().parse::<u32>()).collect::<Result<Vec<_>, _>>()?;
	let link = item.link().ok_or(Error::MissingLink)?.to_string();
	let mut seeders = None;
	let mut leechers = None;
	let mut minimum_ratio = None;
	let mut minimum_seedtime = None;
	if let Some(torznab) = item.extensions().get("torznab") {
		seeders = match get_parsed_extension_value(torznab, "seeders")? {
			Some(seeders) => {
				leechers = get_parsed_extension_value::<u16>(torznab, "peers")?.map(|peers| peers - seeders);
				Some(seeders)
			},
			None => None
		};
		minimum_ratio = get_parsed_extension_value(torznab, "minimumrato")?;
		minimum_seedtime = get_parsed_extension_value(torznab, "minimumseedtime")?.map(Duration::from_secs);
	}

	#[cfg(feature = "require-parse-names")]
	return Ok(Torrent::new(name, size, categories, link, seeders, leechers, minimum_ratio, minimum_seedtime)?);
	#[cfg(not(feature = "require-parse-names"))]
	Ok(Torrent::new(name, size, categories, link, seeders, leechers, minimum_ratio, minimum_seedtime))
}

