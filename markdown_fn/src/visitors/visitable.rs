use super::visitor::MarkdownASTVisitor;
use markdown::mdast;

pub trait MarkdownASTVisitable {
    fn accept<V: MarkdownASTVisitor>(&self, visitor: &V) -> V::Result;
}

impl MarkdownASTVisitable for mdast::Node {
    fn accept<V: MarkdownASTVisitor>(&self, visitor: &V) -> V::Result {
        visitor.visit_node(self)
    }
}
