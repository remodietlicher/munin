use super::visitor::Visitor;
use markdown::mdast;

pub trait Visitable {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result;
}

impl Visitable for mdast::Node {
    fn accept<V: Visitor>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}
