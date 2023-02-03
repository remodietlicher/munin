use markdown::mdast::Node::ListItem;

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

        let lists = mdast.filter(&|node| -> bool { matches!(node, ListItem(_)) });
        let string_lists: Vec<String> = lists.iter().map(|n| n.to_string()).collect();
        println!("{:?}", string_lists);
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
