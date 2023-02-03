use markdown::mdast::Node;

pub trait Filter {
    fn filter(&self, callback_fn: &dyn Fn(&Node) -> bool) -> Vec<&Node>;
}

impl Filter for Node {
    fn filter(&self, callback_fn: &dyn Fn(&Node) -> bool) -> Vec<&Node> {
        let mut res: Vec<&Node> = Vec::new();
        if callback_fn(self) {
            res.push(self);
        }
        if let Some(children) = self.children() {
            for child in children {
                res.extend(child.filter(callback_fn));
            }
        }
        res
    }
}
