// use std::io::Write;
use std::io::Cursor;

fn main() {
    let resp = reqwest::blocking::get("https://sg-sycdn.kuwo.cn/38b41fccd31a9becb6331b613f41d6ae/61974019/resource/n2/44/13/2485638934.mp3").unwrap();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let cursor = Cursor::new(resp.bytes().unwrap());
    let url_soruce = rodio::Decoder::new(cursor).unwrap();
    sink.append(url_soruce);
    sink.sleep_until_end();
}

fn play_audio() {
    // let file = std::fs::File::open("demo.mp3").unwrap();
    // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    // let sink = rodio::Sink::try_new(&handle).unwrap();
    // let file_source = rodio::Decoder::new(file).unwrap();
    // sink.append(file_source);
    // sink.sleep_until_end();
}

fn save_audio() {
    // let resp = reqwest::blocking::get("https://sg-sycdn.kuwo.cn/38b41fccd31a9becb6331b613f41d6ae/61974019/resource/n2/44/13/2485638934.mp3").unwrap();
    // let mut file = std::fs::File::create("test.mp3").unwrap();
    // file.write_all(&resp.bytes().expect("Unable to read the data")).expect("Unable to write data");
}
