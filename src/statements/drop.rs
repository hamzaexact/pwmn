use crate::interpreter::ast::DropTree;

pub struct Drop;

impl Drop {
    pub fn execute(obj_drp: DropTree) -> Result<(), Box<dyn std::error::Error>> {
        match obj_drp {
            DropTree::Reg(_) => {}
            DropTree::Ent(_) => {}
        }

        Ok(())
    }
}
