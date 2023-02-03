use markdown::mdast;

pub trait Visitor {
    type Result;
    fn visit(&mut self, node: &mdast::Node) -> Self::Result;
}
