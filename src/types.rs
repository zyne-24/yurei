#[derive(Clone, Debug)]
pub struct VideoItem {
        pub title: String,
        pub channel: String,
        pub duration: String,
        pub id: String,
        pub thumbnail: String,
}

#[derive(Clone, Debug)]
pub struct VideoFormat {
        pub resolution: String,
        pub ext: String,
        pub fps: u64,
        pub id: String,
        pub vcodec: String,
}

pub enum Action {
        Stream,
        Download,
}
