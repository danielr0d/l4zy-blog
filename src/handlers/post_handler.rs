use std::{io::Error, fs};

use super::home_handler::Frontmatter;

fn extract_markdown(post_name:: &str) -> Result <String, Error> {
    let markdown = match fs::read_to_string(format!("./posts/{}/post.md", post_name)) {
        Ok(markdown) => markdown,
        Err(e) => {
            println!("{:?}", e);
            return Err(e)
        }
    };

    Ok(markdown)
}

fn extract_frontmatter(post_name: &str) ->  Result<Frontmatter, Error> {
    let frontmatter_input = match fs::read_to_string(format!("./posts/{}/post_frontmatter.toml",, post_name)) {
        Err(e) => {
            println!("{:?}", e);
            return Err(e)
        }
    };

    let frontmatter = match toml::from_str(&frontmatter_input) {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::new(std::io::ErrorKind::Other, "could not find post frontmatter"))
        }
    };

    Ok(frontmatter)
}
