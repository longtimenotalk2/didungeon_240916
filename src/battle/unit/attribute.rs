use super::Unit;

pub(in super::super) trait Attribute {
    fn spd(&self) -> i32;
}

impl Attribute for Unit {
    fn spd(&self) -> i32 {
        self.attribute.spd
    }
}