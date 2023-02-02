use markdown::mdast;
use std::collections::BTreeMap;

use super::{visitable::Visitable, visitor::Visitor};

pub struct DebugVisitor;

impl DebugVisitor {
    fn visit_root(&self, root: &mdast::Root) -> () {
        println!("Visited Root");
        root.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_list(&self, list: &mdast::List) -> () {
        println!("Visited List");
        list.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_text(&self, text: &mdast::Text) -> () {
        println!("Visited Text with value {:?}", text.value);
    }
    fn visit_yaml(&self, yaml: &mdast::Yaml) -> () {
        match serde_yaml::from_str(&yaml.value) {
            Ok(yaml) => {
                let deserialized: BTreeMap<String, serde_yaml::Value> = yaml;
                println!("Visited Yaml with contents: {:?}", deserialized)
            }
            Err(e) => panic!("failed to parse yaml: {:?}", e),
        }
    }
    fn visit_emphasis(&self, emphasis: &mdast::Emphasis) -> () {
        println!("Visited Emphasis");
        emphasis.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_link(&self, _link: &mdast::Link) -> () {
        println!("Visited Link")
    }
    fn visit_strong(&self, strong: &mdast::Strong) -> () {
        println!("Visited Strong");
        strong.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_code(&self, code: &mdast::Code) -> () {
        println!("Visited Code with lang {:?}", code.lang)
    }
    fn visit_heading(&self, heading: &mdast::Heading) -> () {
        println!("Visited Heading");
        heading.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_list_item(&self, list_item: &mdast::ListItem) -> () {
        let tmp = list_item.clone();
        let node = mdast::Node::ListItem(tmp);
        println!(
            "Visited ListItem, it is checked? {:?}, content: {:?}",
            list_item.checked,
            node.to_string()
        );
        list_item.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_paragraph(&self, paragraph: &mdast::Paragraph) -> () {
        println!("Visited Paragraph");
        paragraph.children.iter().for_each(|c| {
            c.accept(self);
        });
    }
    fn visit_default(&self) -> () {
        println!("visited unimplemented node!")
    }
}

impl Visitor for DebugVisitor {
    type Result = ();
    fn visit(&self, node: &mdast::Node) -> Self::Result {
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
}
