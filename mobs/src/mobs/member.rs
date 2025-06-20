#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate
}
#[derive(Debug, PartialEq, Eq)]
pub struct Member {
    pub role: Role,
    pub age: u32
}