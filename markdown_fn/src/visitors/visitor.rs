use markdown::mdast;

pub trait MarkdownASTVisitor {
    type Result;
    fn visit_node(&self, node: &mdast::Node) -> Self::Result {
        match node {
            mdast::Node::Root(root) => self.visit_root(root),
            mdast::Node::List(list) => self.visit_list(list),
            mdast::Node::Text(text) => self.visit_text(text),
            mdast::Node::Heading(heading) => self.visit_heading(heading),
            mdast::Node::Yaml(yaml) => self.visit_yaml(yaml),
            mdast::Node::Emphasis(emphasis) => self.visit_emphasis(emphasis),
            mdast::Node::Link(link) => self.visit_link(link),
            mdast::Node::Strong(strong) => self.visit_strong(strong),
            mdast::Node::Code(code) => self.visit_code(code),
            mdast::Node::ListItem(list_item) => self.visit_list_item(list_item),
            mdast::Node::Paragraph(paragraph) => self.visit_paragraph(paragraph),
            _ => self.visit_default(),
        }
    }

    fn visit_root(&self, node: &mdast::Root) -> Self::Result;
    fn visit_list(&self, list: &mdast::List) -> Self::Result;
    fn visit_text(&self, text: &mdast::Text) -> Self::Result;
    fn visit_yaml(&self, yaml: &mdast::Yaml) -> Self::Result;
    fn visit_emphasis(&self, emphasis: &mdast::Emphasis) -> Self::Result;
    fn visit_link(&self, link: &mdast::Link) -> Self::Result;
    fn visit_strong(&self, strong: &mdast::Strong) -> Self::Result;
    fn visit_code(&self, code: &mdast::Code) -> Self::Result;
    fn visit_heading(&self, heading: &mdast::Heading) -> Self::Result;
    fn visit_list_item(&self, list_item: &mdast::ListItem) -> Self::Result;
    fn visit_paragraph(&self, paragraph: &mdast::Paragraph) -> Self::Result;
    fn visit_default(&self) -> Self::Result;
}
