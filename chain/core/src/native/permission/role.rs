use super::action::ElementAction;
use super::operate::{Operate, check, union, diff};

#[derive(Clone, Debug)]
#[allow(dead_code, unused_variables)]
pub struct Role {
    // the role name
    name: String,
    // the permissions
    permissions: Vec<String>,
}

#[allow(dead_code, unused_variables)]
impl Operate for Role {
    fn modify_element(&mut self, element: &Vec<String>, action: ElementAction){
        // check the permission
        check(&self.name, &"update_group".to_string());
        match action {
            ElementAction::Add => self.add_permission(element),
            ElementAction::Delete => self.delete_permission(element),
        }
    }
}

#[allow(dead_code, unused_variables)]
impl Role {
    pub fn add_permission(&mut self, element: &Vec<String>) {
        self.permissions = union(&self.permissions, element);
    }

    pub fn delete_permission(&mut self, element: &Vec<String>) {
        self.permissions = diff(&self.permissions, element);
    }

}