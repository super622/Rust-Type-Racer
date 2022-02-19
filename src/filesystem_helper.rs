use ggez:: { filesystem, Context };

use std::io::{Read, Write};
use std::str;
use std::mem::swap;

pub fn read_file_by_lines(ctx: &Context, path: &str) -> Vec<String> {
    let file = filesystem::open(ctx, path);
        
    if file.is_err() {
        panic!("Error with opening {}!", path);
    }

    let mut buffer = Vec::new();
    let read_size = file.unwrap().read_to_end(&mut buffer);

    if read_size.is_err() || read_size.unwrap() == 0 {
        panic!("Empty file {}!", path);
    }

    let words = str::from_utf8(&buffer).unwrap().trim().split('\n').collect::<Vec<&str>>();
    words.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn save_score(ctx: &Context, username: String, score: f32, scoreboard_size: usize) -> Vec<String> {
    let mut file;
    if filesystem::exists(ctx, "/scoring.data") {
        let mut scores = read_file_by_lines(ctx, "/scoring.data");

        let mut new_line = format!("{} {:.2}", username, score);
        let mut insert = false;
        for line in scores.iter_mut() {
            let split = line.split(" ").collect::<Vec<&str>>();
            let saved_score = split[split.len() - 1].parse::<f32>().unwrap();

            if saved_score < score
            {
                insert = true;
            }

            if insert {
                swap(line, &mut new_line);
            }
        }

        if scores.len() < scoreboard_size
        {
            scores.push(new_line);
        }

        file = filesystem::create(ctx, "/scoring.data").unwrap();

        let _ = file.write(scores.join("\n").as_bytes());

        return scores;
    }
    else {
        file = filesystem::create(ctx, "/scoring.data").unwrap();
    }

    let new_score = format!("{} {:.2}", username, score);
    let _ = file.write(new_score.as_bytes());
    let mut result = Vec::new();
    result.push(new_score);

    result
}