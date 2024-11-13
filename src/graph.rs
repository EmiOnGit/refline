use std::path::PathBuf;

struct Graph {
    // parent:
}
enum Node {
    Directory(Box<Node>),
    Img(PathBuf),
}
