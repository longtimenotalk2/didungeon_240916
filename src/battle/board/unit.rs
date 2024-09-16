use super::UnitData;

impl UnitData {
    pub(super) fn name(&self) -> String {
        self.name.clone()
    }
}