use core::fmt;
use std::time::Duration;
use std::{collections::VecDeque, u128};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use id3::{Tag, TagLike, Version};
use rodio::Decoder;

pub struct MetaData {

    pub artist: Option<String>,
    pub album: Option<String>,
    pub genere: Option<String>,
    pub release_date: Option<String>,

}

impl MetaData {

    pub fn new(artist: Option<String>, album: Option<String>, genere: Option<String>, duration: Option<Duration>, release_date: Option<String>) -> MetaData {

        MetaData {
            artist,
            album,
            genere,
            release_date,
        }

    }

}

pub struct Song {

    pub title: String,
    pub path: String,
    pub source: Option<Decoder<BufReader<File>>>

}

impl Song {

    pub fn new(title: String, path: String) -> Song {

        Song {
            title,
            path,
            source: None
        }

    }

    pub fn add_source(&mut self) {

        self.source = Some(get_source(self.path.as_str()).unwrap());

    }

    pub fn get_source(&self) -> Decoder<BufReader<File>> {

        get_source(&self.path).unwrap()

    }

}

pub struct Queue{

    queue: VecDeque<Song>

}

impl PartialEq for Song {

    fn eq(&self, other: &Song) -> bool{

        self.title == other.title

    }

}

impl fmt::Debug for Song {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.title)
    }

}

impl fmt::Debug for Queue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.queue)
    }

}

impl Queue {


    pub fn new() -> Queue{

        Queue {
            queue: VecDeque::new()
        }

    }

    pub fn is_empty(&self) -> bool {

        self.queue.is_empty()

    }

    pub fn size(&self) -> usize {

        self.queue.len()

    }

    pub fn clear(&mut self) {

        self.queue.clear();

    }

    pub fn add_front(&mut self, song: Song) {

        self.queue.push_front(song);

    }

    pub fn push(&mut self, song: Song) {

        self.queue.push_back(song);

    }

    pub fn get_first(&mut self) -> Option<Song> {

        self.queue.pop_front()

    }

    pub fn remove_at(&mut self, index: usize) -> Option<Song> {

        self.queue.remove(index)

    }

    pub fn contains(&self, song: &Song) -> bool {

        self.queue.contains(song)

    }

}

fn get_source(audio_url: &str) -> Result<Decoder<BufReader<File>>, Box<dyn Error>> {

    let file = File::open(audio_url)?;
    let source = Decoder::new(BufReader::new(file))?;

    Ok(source)

}

pub fn add_meta_data_to_mp3(path: &str, meta_data: MetaData) {

    let mut tag = Tag::read_from_path(path).unwrap_or_else(|_| Tag::new());

    tag.set_artist(meta_data.artist.unwrap_or("Unknown".to_string()));
    tag.set_album(meta_data.album.unwrap_or("Unknown".to_string()));
    tag.set_genre(meta_data.genere.unwrap_or("Unknown".to_string()));

    tag.write_to_path(path, Version::Id3v24).unwrap();

}

pub fn get_meta_data(path: &str) -> Result<MetaData, Box<dyn std::error::Error>> {

    let tag = Tag::read_from_path(path)?;

    let artist = String::from(tag.artist().unwrap_or("Unknown Artist"));
    let album = String::from(tag.album().unwrap_or("Unknown Album"));
    let genre = String::from(tag.genre().unwrap_or("Unknown Genre"));

    Ok(MetaData::new(Some(artist), Some(album), Some(genre), None, None))

}
