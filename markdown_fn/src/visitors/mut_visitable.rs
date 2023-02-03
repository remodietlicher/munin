use super::mut_visitor::MutVisitor;

use markdown::mdast;

pub trait MutVisitable {
    fn accept<V: MutVisitor>(&mut self, visitor: &mut V) -> V::Result;
}

impl MutVisitable for mdast::Node {
    fn accept<V: MutVisitor>(&mut self, visitor: &mut V) -> V::Result {
        visitor.visit(self)
    }
}
