use crate::CommandMap;

pub(crate) mod list;
mod help;
mod make;

pub fn register(map: &mut CommandMap) {
    map.insert("help", help::execute);
    map.insert("list", list::execute);
    map.insert("make", make::execute);
}