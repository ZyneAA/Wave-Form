use std::error::Error;

mod youtube;
mod music;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {


    ui::test::test_render()

    /* let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let api_key = String::from("");:

    match youtube::video::find(input.trim(), &api_key, 1) {

        Ok(result) => {

           let video_id = result.items[0].id.video_id.clone();
           println!("{} {} {}", &result.items[0].snippet.title, &result.items[0].snippet.channel, &result.items[0].snippet.publish_time);

           let response = music::info::get_music_url(&video_id).unwrap();

           let download_thread_0 = music::downloader::threaded_download_audio(&response, "ok0.mp3");
           let download_thread_1 = music::downloader::threaded_download_audio(&response, "ok1.mp3");
           let download_thread_2 =  music::downloader::threaded_download_audio(&response, "ok2.mp3");

           let detail_thread = thread::spawn(move || {

                println!("in detail thread");

               let id = video_id.clone();
               let key = api_key.clone();

               let detail = youtube::video::get_video_details(&id, &key).unwrap();
               println!("{}", detail.items[0].content_details.duration);

           });

           let _ = download_thread_0.unwrap().join();
           let _ = download_thread_1.unwrap().join();
           let _ = download_thread_2.unwrap().join();
           detail_thread.join().unwrap();

        },
        Err(e) => println!("{}", e),
    }; */

}

