use const_format::concatcp;

pub const YOUTUBE_BASE_URL: &'static str = "https://www.youtube.com/youtubei/v1";
pub const YOUTUBE_SEARCH: &'static str = concatcp!(YOUTUBE_BASE_URL, "/search");
pub const YOUTUBE_LIVE_CHAT: &'static str = concatcp!(YOUTUBE_BASE_URL, "/live_chat/get_live_chat");
pub const YOUTUBE_KEY: &'static str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";