use std::fs::read_dir;
mod post;
use post::{parse, Post};

fn main() {
    let posts = read_dir("posts").unwrap();
    for post in posts {
        let path = match post {
            Ok(post) => post.path(),
            Err(_) => continue,
        };

        let src = std::fs::read_to_string(path).unwrap();
        let mut post = parse(&src);
        let mut html = Vec::new();

        post.write(&mut html).unwrap();

        let html = String::from_utf8(html).unwrap();
        println!("{}", html);
    }
}
