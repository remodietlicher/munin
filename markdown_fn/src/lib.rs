use markdown::mdast::Node;

use crate::traits::filter::Filter;
use crate::visitors::visitable::Visitable;
pub mod traits;
mod visitors;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn debug() -> Result<(), String> {
    let md_file = std::fs::read_to_string("/home/remo/project/munin/munin/dummy.md");

    let mut options = markdown::ParseOptions::gfm();
    options.constructs.frontmatter = true;

    if let Ok(md_file) = md_file {
        let mdast = markdown::to_mdast(&md_file[..], &options)?;
        println!("{:?}", mdast);

        let mut visitor = visitors::debug::DebugVisitor {};
        mdast.accept(&mut visitor);

        let lists = mdast.filter(&|node| -> bool { matches!(node, Node::ListItem(_)) });
        let string_lists: Vec<String> = lists.iter().map(|n| n.to_string()).collect();
        println!("USING FILTER: {:?}", string_lists);

        let elements: Vec<&markdown::mdast::Node> = mdast
            .into_iter()
            .filter(|node| matches!(node, Node::ListItem(li) if li.checked.is_some()))
            .collect();
        let string_lists: Vec<String> = elements.iter().map(|n| n.to_string()).collect();
        println!("list: {:?}", string_lists);
        println!("full list tree: {:?}", elements[0]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
