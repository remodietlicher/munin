use markdown::mdast;

pub trait MutVisitor {
    type Result;
    fn visit(&mut self, node: &mut mdast::Node) -> Self::Result;
}
