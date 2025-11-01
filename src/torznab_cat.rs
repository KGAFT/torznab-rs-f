#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorznabCategory {
    Reserved0000 = 0,
    Console1000 = 1000,
    ConsoleNds1010 = 1010,
    ConsolePsp1020 = 1020,
    ConsoleWii1030 = 1030,
    ConsoleXbox1040 = 1040,
    ConsoleXbox3601050 = 1050,
    ConsoleWiiware1060 = 1060,
    ConsoleXbox360Dlc1070 = 1070,

    Movies2000 = 2000,
    MoviesForeign2010 = 2010,
    MoviesOther2020 = 2020,
    MoviesSd2030 = 2030,
    MoviesHd2040 = 2040,
    MoviesUhd2045 = 2045,
    MoviesBluRay2050 = 2050,
    Movies3d2060 = 2060,

    Audio3000 = 3000,
    AudioMp33010 = 3010,
    AudioVideo3020 = 3020,
    AudioAudiobook3030 = 3030,
    AudioLossless3040 = 3040,

    Pc4000 = 4000,
    Pc0day4010 = 4010,
    PcIso4020 = 4020,
    PcMac4030 = 4030,
    PcMobileOther4040 = 4040,
    PcGames4050 = 4050,
    PcMobileIos4060 = 4060,
    PcMobileAndroid4070 = 4070,

    Tv5000 = 5000,
    TvForeign5020 = 5020,
    TvSd5030 = 5030,
    TvHd5040 = 5040,
    TvUhd5045 = 5045,
    TvOther5050 = 5050,
    TvSport5060 = 5060,
    TvAnime5070 = 5070,
    TvDocumentary5080 = 5080,

    Xxx6000 = 6000,
    XxxDvd6010 = 6010,
    XxxWmv6020 = 6020,
    XxxXvid6030 = 6030,
    XxxX2646040 = 6040,
    XxxPack6050 = 6050,
    XxxImgSet6060 = 6060,
    XxxOther6070 = 6070,

    Books7000 = 7000,
    BooksMags7010 = 7010,
    BooksEbook7020 = 7020,
    BooksComics7030 = 7030,

    Other8000 = 8000,
    OtherMisc8010 = 8010,
    // Note: Some sources show 7020=Ebook, 7030=Comics. :contentReference[oaicite:3]{index=3}

}

impl From<u32> for TorznabCategory {
    fn from(value: u32) -> Self {
        match value {
            0 => TorznabCategory::Reserved0000,
            1000 => TorznabCategory::Console1000,
            1010 => TorznabCategory::ConsoleNds1010,
            1020 => TorznabCategory::ConsolePsp1020,
            1030 => TorznabCategory::ConsoleWii1030,
            1040 => TorznabCategory::ConsoleXbox1040,
            1050 => TorznabCategory::ConsoleXbox3601050,
            1060 => TorznabCategory::ConsoleWiiware1060,
            1070 => TorznabCategory::ConsoleXbox360Dlc1070,

            2000 => TorznabCategory::Movies2000,
            2010 => TorznabCategory::MoviesForeign2010,
            2020 => TorznabCategory::MoviesOther2020,
            2030 => TorznabCategory::MoviesSd2030,
            2040 => TorznabCategory::MoviesHd2040,
            2045 => TorznabCategory::MoviesUhd2045,
            2050 => TorznabCategory::MoviesBluRay2050,
            2060 => TorznabCategory::Movies3d2060,

            3000 => TorznabCategory::Audio3000,
            3010 => TorznabCategory::AudioMp33010,
            3020 => TorznabCategory::AudioVideo3020,
            3030 => TorznabCategory::AudioAudiobook3030,
            3040 => TorznabCategory::AudioLossless3040,

            4000 => TorznabCategory::Pc4000,
            4010 => TorznabCategory::Pc0day4010,
            4020 => TorznabCategory::PcIso4020,
            4030 => TorznabCategory::PcMac4030,
            4040 => TorznabCategory::PcMobileOther4040,
            4050 => TorznabCategory::PcGames4050,
            4060 => TorznabCategory::PcMobileIos4060,
            4070 => TorznabCategory::PcMobileAndroid4070,

            5000 => TorznabCategory::Tv5000,
            5020 => TorznabCategory::TvForeign5020,
            5030 => TorznabCategory::TvSd5030,
            5040 => TorznabCategory::TvHd5040,
            5045 => TorznabCategory::TvUhd5045,
            5050 => TorznabCategory::TvOther5050,
            5060 => TorznabCategory::TvSport5060,
            5070 => TorznabCategory::TvAnime5070,
            5080 => TorznabCategory::TvDocumentary5080,

            6000 => TorznabCategory::Xxx6000,
            6010 => TorznabCategory::XxxDvd6010,
            6020 => TorznabCategory::XxxWmv6020,
            6030 => TorznabCategory::XxxXvid6030,
            6040 => TorznabCategory::XxxX2646040,
            6050 => TorznabCategory::XxxPack6050,
            6060 => TorznabCategory::XxxImgSet6060,
            6070 => TorznabCategory::XxxOther6070,

            7000 => TorznabCategory::Books7000,
            7010 => TorznabCategory::BooksMags7010,
            7020 => TorznabCategory::BooksEbook7020,
            7030 => TorznabCategory::BooksComics7030,

            8000 => TorznabCategory::Other8000,
            8010 => TorznabCategory::OtherMisc8010,

            _ => {TorznabCategory::Console1000}
        }
    }
}

impl TorznabCategory {
    /// Returns the numeric ID of the category.
    pub fn as_u32(self) -> u32 {
        match self {
            TorznabCategory::Reserved0000 => 0,
            TorznabCategory::Console1000 => 1000,
            TorznabCategory::ConsoleNds1010 => 1010,
            TorznabCategory::ConsolePsp1020 => 1020,
            TorznabCategory::ConsoleWii1030 => 1030,
            TorznabCategory::ConsoleXbox1040 => 1040,
            TorznabCategory::ConsoleXbox3601050 => 1050,
            TorznabCategory::ConsoleWiiware1060 => 1060,
            TorznabCategory::ConsoleXbox360Dlc1070 => 1070,

            TorznabCategory::Movies2000 => 2000,
            TorznabCategory::MoviesForeign2010 => 2010,
            TorznabCategory::MoviesOther2020 => 2020,
            TorznabCategory::MoviesSd2030 => 2030,
            TorznabCategory::MoviesHd2040 => 2040,
            TorznabCategory::MoviesUhd2045 => 2045,
            TorznabCategory::MoviesBluRay2050 => 2050,
            TorznabCategory::Movies3d2060 => 2060,

            TorznabCategory::Audio3000 => 3000,
            TorznabCategory::AudioMp33010 => 3010,
            TorznabCategory::AudioVideo3020 => 3020,
            TorznabCategory::AudioAudiobook3030 => 3030,
            TorznabCategory::AudioLossless3040 => 3040,

            TorznabCategory::Pc4000 => 4000,
            TorznabCategory::Pc0day4010 => 4010,
            TorznabCategory::PcIso4020 => 4020,
            TorznabCategory::PcMac4030 => 4030,
            TorznabCategory::PcMobileOther4040 => 4040,
            TorznabCategory::PcGames4050 => 4050,
            TorznabCategory::PcMobileIos4060 => 4060,
            TorznabCategory::PcMobileAndroid4070 => 4070,

            TorznabCategory::Tv5000 => 5000,
            TorznabCategory::TvForeign5020 => 5020,
            TorznabCategory::TvSd5030 => 5030,
            TorznabCategory::TvHd5040 => 5040,
            TorznabCategory::TvUhd5045 => 5045,
            TorznabCategory::TvOther5050 => 5050,
            TorznabCategory::TvSport5060 => 5060,
            TorznabCategory::TvAnime5070 => 5070,
            TorznabCategory::TvDocumentary5080 => 5080,

            TorznabCategory::Xxx6000 => 6000,
            TorznabCategory::XxxDvd6010 => 6010,
            TorznabCategory::XxxWmv6020 => 6020,
            TorznabCategory::XxxXvid6030 => 6030,
            TorznabCategory::XxxX2646040 => 6040,
            TorznabCategory::XxxPack6050 => 6050,
            TorznabCategory::XxxImgSet6060 => 6060,
            TorznabCategory::XxxOther6070 => 6070,

            TorznabCategory::Books7000 => 7000,
            TorznabCategory::BooksMags7010 => 7010,
            TorznabCategory::BooksEbook7020 => 7020,
            TorznabCategory::BooksComics7030 => 7030,

            TorznabCategory::Other8000 => 8000,
            TorznabCategory::OtherMisc8010 => 8010,
        }
    }
}
