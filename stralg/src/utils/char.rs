pub trait CharacterTrait: Eq + std::hash::Hash + TryFrom<usize> + Copy + Into<usize> {
    type Error: std::fmt::Debug;
    const MAX: usize;
}
impl CharacterTrait for u8 {
    type Error = std::num::TryFromIntError;
    const MAX: usize = u8::MAX as usize - 1; // -1 to leave room for the sentinel
}
impl CharacterTrait for u16 {
    type Error = std::num::TryFromIntError;
    const MAX: usize = u16::MAX as usize - 1; // -1 to leave room for the sentinel
}
