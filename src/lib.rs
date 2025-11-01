#![allow(unused_parens)]
extern crate bytes;
extern crate rss;
extern crate thiserror;
extern crate urlencoding;

use bytes::Buf;
use bytes::Bytes;
use itertools::Itertools;
use rss::Channel;
use rss::Item;
use smartcow::SmartCow;
use smartstring::alias::String;
use tracing::instrument;

mod error;
pub use error::Error;
mod torrent;
mod torznab_cat;

#[cfg(any(feature = "parse-names", feature = "require-parse-names"))]
/// Re-exported from [`torrent-name-parser`](torrent_name_parser::Metadata)
pub use torrent_name_parser::Metadata;

/// Re-exported from [`torrent-common`](torrent_common::Torrent)
pub use torrent_common::Torrent;
use crate::torznab_cat::TorznabCategory;

#[derive(Clone)]
pub struct Client {
	http: reqwest::Client,
	base_url: std::string::String,
	apikey: std::string::String
}

impl Client {
	#[instrument(err, level = "info", skip(base_url, apikey), fields(base_url = %base_url.to_string(), apikey = %apikey.to_string()))]
	pub fn new(base_url: impl ToString, apikey: impl ToString) -> Result<Self, reqwest::Error> {
		let this = Self{
			http: reqwest::Client::builder()
				.gzip(true)
				.build()?,
			base_url: base_url.to_string(),
			apikey: apikey.to_string()
		};
		// TODO:  Check caps
		Ok(this)
	}

	#[instrument(err, level = "debug", skip(self))]
	pub async fn get(&self, t: TorznabCategory, qparams: Vec<(&str, SmartCow<'_>)>) -> Result<Bytes, reqwest::Error> {
		let url = format!("{}?category={}&apikey={}&{}", self.base_url, t.as_u32(), self.apikey, qparams.into_iter()
			.map(|(k, v)| {
				let mut s = String::from(urlencoding::encode(k));
				s.push('=');
				s.push_str(&urlencoding::encode(&v));
				s
			})
			.join("&")
		);
		println!("{}", url);
		self.http.get(&url).send().await?.error_for_status()?.bytes().await
	}

	#[instrument(err, level = "debug", skip(self))]
	pub async fn get_items(&self, t: TorznabCategory, qparams: Vec<(&str, SmartCow<'_>)>) -> Result<Vec<Item>, Error> {
		let bytes = self.get(t, qparams).await?.reader();
		let channel = Channel::read_from(bytes)?;
		Ok(channel.into_items())
	}
	//Sorry, but in new torznab there is no such parameters
	#[instrument(err, level = "info", skip(self))]
	pub async fn tvsearch(&self, q: Option<&str>) -> Result<Vec<Result<Torrent, Error>>, Error> {
		let mut qparams = Vec::new();
		if let Some(v) = q {
			qparams.push(("q", SmartCow::Borrowed(v)));
		}
		let items = self.get_items(TorznabCategory::Tv5000, qparams).await?;
		Ok(items.into_iter().map(torrent::from_item).collect::<Vec<_>>())
	}
	//Sorry, but in new torznab there is no such parameters
	#[instrument(err, level = "info", skip(self))]
	pub async fn moviesearch(&self, q: Option<&str>) -> Result<Vec<Result<Torrent, Error>>, Error> {
		let mut qparams = Vec::new();
		if let Some(v) = q {
			qparams.push(("q", SmartCow::Borrowed(v)))
		}
		let items = self.get_items(TorznabCategory::Movies2000, qparams).await?;
		Ok(items.into_iter().map(torrent::from_item).collect::<Vec<_>>())
	}

	#[instrument(err, level = "info", skip(self))]
	pub async fn audiosearch(&self, q: Option<&str>) -> Result<Vec<Result<Torrent, Error>>, Error>{
		let mut qparams = Vec::new();
		if let Some(v) = q {
			qparams.push(("q", SmartCow::Borrowed(v)));
		}
		let items = self.get_items(TorznabCategory::Audio3000, qparams).await?;
		Ok(items.into_iter().map(torrent::from_item).collect::<Vec<_>>())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn issue001_broken_jackett_feed() {
		let s = /* {{{ */ r#"
			<?xml version="1.0" encoding="UTF-8"?>
			<rss version="1.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:torznab="http://torznab.com/schemas/2015/feed">
			  <channel>
				<atom:link href="https://jackett.libretinker.com/" rel="self" type="application/rss+xml" />
				<title>RARBG</title>
				<description>RARBG is a Public torrent site for MOVIES / TV / GENERAL</description>
				<link>https://rarbg.to/</link>
				<language>en-us</language>
				<category>search</category>
				<item>
				  <title>Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.8bit.SDR.DDP5.1-CM</title>
				  <guid>https://rarbg.to/infohash/cea91d952f2c94777944059284ed7422f6d3c7c8</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_7_0_9_2_0__cea91d952f&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Sat, 03 Apr 2021 05:29:49 -0400</pubDate>
				  <size>6827927817</size>
				  <description />
				  <link>magnet:?xt=urn:btih:cea91d952f2c94777944059284ed7422f6d3c7c8&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.8bit.SDR.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2960&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2960</link>
				  <category>2045</category>
				  <category>100051</category>
				  <enclosure url="magnet:?xt=urn:btih:cea91d952f2c94777944059284ed7422f6d3c7c8&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.8bit.SDR.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2960&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2960" length="6827927817" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2045" />
				  <torznab:attr name="category" value="100051" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="109" />
				  <torznab:attr name="peers" value="133" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:cea91d952f2c94777944059284ed7422f6d3c7c8&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.8bit.SDR.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2960&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2960" />
				  <torznab:attr name="infohash" value="cea91d952f2c94777944059284ed7422f6d3c7c8" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.1080p.WEBRip.x265-RARBG</title>
				  <guid>https://rarbg.to/infohash/e4b7c9952ccf8ff9d9c702260f466d29eeaf7111</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_7_0_0_5_5__e4b7c9952c&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Fri, 02 Apr 2021 05:51:01 -0400</pubDate>
				  <size>1820639152</size>
				  <description />
				  <link>magnet:?xt=urn:btih:e4b7c9952ccf8ff9d9c702260f466d29eeaf7111&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x265-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2970&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2970</link>
				  <category>2040</category>
				  <category>100054</category>
				  <enclosure url="magnet:?xt=urn:btih:e4b7c9952ccf8ff9d9c702260f466d29eeaf7111&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x265-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2970&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2970" length="1820639152" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100054" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="264" />
				  <torznab:attr name="peers" value="287" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:e4b7c9952ccf8ff9d9c702260f466d29eeaf7111&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x265-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2970&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2970" />
				  <torznab:attr name="infohash" value="e4b7c9952ccf8ff9d9c702260f466d29eeaf7111" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.PROPER.1080p.WEBRip.x264-RARBG</title>
				  <guid>https://rarbg.to/infohash/c7910b6216251addf60a3d8c4ffc2e5209044a9f</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_9_8_3_9__c7910b6216&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Fri, 02 Apr 2021 02:48:29 -0400</pubDate>
				  <size>2227542446</size>
				  <description />
				  <link>magnet:?xt=urn:btih:c7910b6216251addf60a3d8c4ffc2e5209044a9f&amp;dn=Chaos.Walking.2021.PROPER.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870</link>
				  <category>2040</category>
				  <category>100044</category>
				  <enclosure url="magnet:?xt=urn:btih:c7910b6216251addf60a3d8c4ffc2e5209044a9f&amp;dn=Chaos.Walking.2021.PROPER.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870" length="2227542446" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100044" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="279" />
				  <torznab:attr name="peers" value="295" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:c7910b6216251addf60a3d8c4ffc2e5209044a9f&amp;dn=Chaos.Walking.2021.PROPER.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870" />
				  <torznab:attr name="infohash" value="c7910b6216251addf60a3d8c4ffc2e5209044a9f" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.1080p.AMZN.WEBRip.DDP5.1.x264-CM</title>
				  <guid>https://rarbg.to/infohash/a16d25826cde3046bf0df4aee556957d20706af7</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_9_7_0_3__a16d25826c&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Fri, 02 Apr 2021 00:51:40 -0400</pubDate>
				  <size>6190367211</size>
				  <description />
				  <link>magnet:?xt=urn:btih:a16d25826cde3046bf0df4aee556957d20706af7&amp;dn=Chaos.Walking.2021.1080p.AMZN.WEBRip.DDP5.1.x264-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2980&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2980</link>
				  <category>2040</category>
				  <category>100044</category>
				  <enclosure url="magnet:?xt=urn:btih:a16d25826cde3046bf0df4aee556957d20706af7&amp;dn=Chaos.Walking.2021.1080p.AMZN.WEBRip.DDP5.1.x264-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2980&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2980" length="6190367211" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100044" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="108" />
				  <torznab:attr name="peers" value="116" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:a16d25826cde3046bf0df4aee556957d20706af7&amp;dn=Chaos.Walking.2021.1080p.AMZN.WEBRip.DDP5.1.x264-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2980&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2980" />
				  <torznab:attr name="infohash" value="a16d25826cde3046bf0df4aee556957d20706af7" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-CM</title>
				  <guid>https://rarbg.to/infohash/6e4631a271a37692e90a8c08f565648506ed4ef4</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_9_7_0_2__6e4631a271&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Fri, 02 Apr 2021 00:51:15 -0400</pubDate>
				  <size>12460018899</size>
				  <description />
				  <link>magnet:?xt=urn:btih:6e4631a271a37692e90a8c08f565648506ed4ef4&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2990&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2990</link>
				  <category>2045</category>
				  <category>100052</category>
				  <enclosure url="magnet:?xt=urn:btih:6e4631a271a37692e90a8c08f565648506ed4ef4&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2990&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2990" length="12460018899" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2045" />
				  <torznab:attr name="category" value="100052" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="216" />
				  <torznab:attr name="peers" value="274" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:6e4631a271a37692e90a8c08f565648506ed4ef4&amp;dn=Chaos.Walking.2021.2160p.AMZN.WEB-DL.x265.10bit.HDR10Plus.DDP5.1-CM&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2990&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2990" />
				  <torznab:attr name="infohash" value="6e4631a271a37692e90a8c08f565648506ed4ef4" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.1080p.WEBRip.x264-RARBG</title>
				  <guid>https://rarbg.to/infohash/bef69e8fddadd4cbf32dc6523b0aeb8af4afe88c</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_9_0_6__bef69e8fdd&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 03:56:19 -0400</pubDate>
				  <size>2228167421</size>
				  <description />
				  <link>magnet:?xt=urn:btih:bef69e8fddadd4cbf32dc6523b0aeb8af4afe88c&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2770&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2770</link>
				  <category>2040</category>
				  <category>100044</category>
				  <enclosure url="magnet:?xt=urn:btih:bef69e8fddadd4cbf32dc6523b0aeb8af4afe88c&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2770&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2770" length="2228167421" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100044" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="1029" />
				  <torznab:attr name="peers" value="1284" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:bef69e8fddadd4cbf32dc6523b0aeb8af4afe88c&amp;dn=Chaos.Walking.2021.1080p.WEBRip.x264-RARBG&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2770&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2770" />
				  <torznab:attr name="infohash" value="bef69e8fddadd4cbf32dc6523b0aeb8af4afe88c" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.WEB-DL.x264-FGT</title>
				  <guid>https://rarbg.to/infohash/07e1f00968128424e7a44b728c3e1c3f49fa1371</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_8_6_3__07e1f00968&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 03:13:28 -0400</pubDate>
				  <size>1111925396</size>
				  <description />
				  <link>magnet:?xt=urn:btih:07e1f00968128424e7a44b728c3e1c3f49fa1371&amp;dn=Chaos.Walking.2021.WEB-DL.x264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870</link>
				  <category>2030</category>
				  <category>100017</category>
				  <enclosure url="magnet:?xt=urn:btih:07e1f00968128424e7a44b728c3e1c3f49fa1371&amp;dn=Chaos.Walking.2021.WEB-DL.x264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870" length="1111925396" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2030" />
				  <torznab:attr name="category" value="100017" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="290" />
				  <torznab:attr name="peers" value="320" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:07e1f00968128424e7a44b728c3e1c3f49fa1371&amp;dn=Chaos.Walking.2021.WEB-DL.x264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2870&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2870" />
				  <torznab:attr name="infohash" value="07e1f00968128424e7a44b728c3e1c3f49fa1371" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.WEB-DL.XviD.MP3-FGT</title>
				  <guid>https://rarbg.to/infohash/6aecf23c528540d4f4f175e347b9874cd6386c72</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_8_5_0__6aecf23c52&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 03:00:25 -0400</pubDate>
				  <size>1635570463</size>
				  <description />
				  <link>magnet:?xt=urn:btih:6aecf23c528540d4f4f175e347b9874cd6386c72&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.MP3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2830&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2830</link>
				  <category>2030</category>
				  <category>100014</category>
				  <enclosure url="magnet:?xt=urn:btih:6aecf23c528540d4f4f175e347b9874cd6386c72&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.MP3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2830&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2830" length="1635570463" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2030" />
				  <torznab:attr name="category" value="100014" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="87" />
				  <torznab:attr name="peers" value="98" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:6aecf23c528540d4f4f175e347b9874cd6386c72&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.MP3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2830&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2830" />
				  <torznab:attr name="infohash" value="6aecf23c528540d4f4f175e347b9874cd6386c72" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.WEB-DL.XviD.AC3-FGT</title>
				  <guid>https://rarbg.to/infohash/d01bcd81602fd79fa6d8a5cbeef40c513dfa2095</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_8_4_9__d01bcd8160&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 03:00:19 -0400</pubDate>
				  <size>1789601479</size>
				  <description />
				  <link>magnet:?xt=urn:btih:d01bcd81602fd79fa6d8a5cbeef40c513dfa2095&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2860&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2860</link>
				  <category>2030</category>
				  <category>100014</category>
				  <enclosure url="magnet:?xt=urn:btih:d01bcd81602fd79fa6d8a5cbeef40c513dfa2095&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2860&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2860" length="1789601479" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2030" />
				  <torznab:attr name="category" value="100014" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="19" />
				  <torznab:attr name="peers" value="21" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:d01bcd81602fd79fa6d8a5cbeef40c513dfa2095&amp;dn=Chaos.Walking.2021.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2860&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2860" />
				  <torznab:attr name="infohash" value="d01bcd81602fd79fa6d8a5cbeef40c513dfa2095" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.720p.WEB-DL.XviD.AC3-FGT</title>
				  <guid>https://rarbg.to/infohash/25258acafd004e9854769ce4beeb8b057337cc86</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_8_4_8__25258acafd&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 03:00:09 -0400</pubDate>
				  <size>3584040605</size>
				  <description />
				  <link>magnet:?xt=urn:btih:25258acafd004e9854769ce4beeb8b057337cc86&amp;dn=Chaos.Walking.2021.720p.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2890&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2890</link>
				  <category>2040</category>
				  <category>100048</category>
				  <enclosure url="magnet:?xt=urn:btih:25258acafd004e9854769ce4beeb8b057337cc86&amp;dn=Chaos.Walking.2021.720p.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2890&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2890" length="3584040605" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100048" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="30" />
				  <torznab:attr name="peers" value="52" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:25258acafd004e9854769ce4beeb8b057337cc86&amp;dn=Chaos.Walking.2021.720p.WEB-DL.XviD.AC3-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2890&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2890" />
				  <torznab:attr name="infohash" value="25258acafd004e9854769ce4beeb8b057337cc86" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
				<item>
				  <title>Chaos.Walking.2021.1080p.WEB-DL.DD5.1.H264-FGT</title>
				  <guid>https://rarbg.to/infohash/1ec7c58d2d039035db2c42dab81e1ad841aa43ca</guid>
				  <jackettindexer id="rarbg">RARBG</jackettindexer>
				  <comments>https://torrentapi.org/redirect_to_info.php?token=8s2m4h7nwj&amp;p=2_4_6_8_8_4_4__1ec7c58d2d&amp;app_id=jackett_v0.17.908</comments>
				  <pubDate>Thu, 01 Apr 2021 02:38:20 -0400</pubDate>
				  <size>3994659600</size>
				  <description />
				  <link>magnet:?xt=urn:btih:1ec7c58d2d039035db2c42dab81e1ad841aa43ca&amp;dn=Chaos.Walking.2021.1080p.WEB-DL.DD5.1.H264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2850&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2850</link>
				  <category>2040</category>
				  <category>100044</category>
				  <enclosure url="magnet:?xt=urn:btih:1ec7c58d2d039035db2c42dab81e1ad841aa43ca&amp;dn=Chaos.Walking.2021.1080p.WEB-DL.DD5.1.H264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2850&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2850" length="3994659600" type="application/x-bittorrent" />
				  <torznab:attr name="category" value="2040" />
				  <torznab:attr name="category" value="100044" />
				  <torznab:attr name="imdb" value="2076822" />
				  <torznab:attr name="imdbid" value="tt2076822" />
				  <torznab:attr name="tmdbid" value="412656" />
				  <torznab:attr name="seeders" value="753" />
				  <torznab:attr name="peers" value="813" />
				  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:1ec7c58d2d039035db2c42dab81e1ad841aa43ca&amp;dn=Chaos.Walking.2021.1080p.WEB-DL.DD5.1.H264-FGT&amp;tr=http%3A%2F%2Ftracker.trackerfix.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2F9.rarbg.me%3A2850&amp;tr=udp%3A%2F%2F9.rarbg.to%3A2850" />
				  <torznab:attr name="infohash" value="1ec7c58d2d039035db2c42dab81e1ad841aa43ca" />
				  <torznab:attr name="downloadvolumefactor" value="0" />
				  <torznab:attr name="uploadvolumefactor" value="1" />
				</item>
			  </channel>
			</rss>
		"#; // }}}
		let b = bytes::Bytes::from(s).reader();
		let channel = Channel::read_from(b).unwrap();
		let items = channel.into_items();
		assert!(items.len() == 11);
		for item in items.into_iter() {
			torrent::from_item(item).unwrap();
		}
	}
}

