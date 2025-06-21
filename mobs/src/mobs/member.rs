#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Member {
    pub role: Role,
    pub age: u32
}