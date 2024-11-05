use core::fmt;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use rodio::Decoder;

pub struct Song {

    pub title: String,
    pub artist: String,
    pub path: String,
    pub source: Option<Decoder<BufReader<File>>>

}

impl Song {

    pub fn new(title: String, path: String, artist: Option<String>) -> Song {

        Song {
            title,
            artist: match artist {
                Some(val) => String::from(val),
                None => String::from("Unknown")
            },
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
