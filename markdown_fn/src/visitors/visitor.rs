use markdown::mdast;

pub trait Visitor {
    type Result;
    fn visit(&self, node: &mdast::Node) -> Self::Result;
}
