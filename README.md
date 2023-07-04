# blogworm 
"blogworm" is a tool used to fetch the latest security blog posts.

## How it works

It saves the timestamp of the user's first run in '~/.blogworm/timestamp'. Afterwards, it fetches the latest blog posts from the blog sources. If a blog post has a creation time later than the last run timestamp, it will output the latest blog posts.

## How to use

To show all the blog source
```bash
blogworm -w
```

To fetch the latest blog post from a blog source, you can use the following command
```bash
blogworm -g $blogname
```

To check the latest blog posts 
```bash
blogworm
```
![run](run.gif)
## Install
```bash
cargo build --release
```

## Summary:
This project was written by me to learn the Rust programming language. The code is poorly written, and I apologize for that! I will update with new blog sources and work on optimizing the code in the future! ^^
