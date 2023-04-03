#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
pub enum SideEffect {
    #[default]
    None,
    // Can stick to the walls and ceiling.
    // If you stick to something - you move slower.
    Sticky,
    //
    Slippery,
    //
    Shield,
    //
    Thorns,
    //
    Flashlight,
    //
    Laser,
}

impl SideEffect {
    pub fn to_index(&self) -> u32 {
        match *self {
            SideEffect::None => 0,
            SideEffect::Sticky => 1,
            SideEffect::Slippery => 2,
            SideEffect::Shield => 3,
            SideEffect::Thorns => 4,
            SideEffect::Flashlight => 5,
            SideEffect::Laser => 6,
        }
    }
}
