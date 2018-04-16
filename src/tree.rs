struct Node {
    val: String,
    children: Option<Vec<Node>>,
}

impl Node {
    fn new_from_dir(files: Vec<String>) -> Node {

