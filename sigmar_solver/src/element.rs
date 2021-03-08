
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Element {
    VITAE,
    MORT,

    AIR,
    FIRE,
    WATER,
    PLANT,
    SALT,

    QUICKSILVER,
    LEAD,
    TIN,
    IRON,
    COPPER,
    SILVER,
    GOLD,

    EMPTY,
}

impl Element {
    fn metal_id(&self) -> u32 {
        match self {
            Element::LEAD => 0,
            Element::TIN => 1,
            Element::IRON => 2,
            Element::COPPER => 3,
            Element::SILVER => 4,
            Element::GOLD => 5,
            _ => panic!(),
        }
    }
    pub fn cmp_metal(&self, other: Self) -> std::cmp::Ordering {
        if !(self.is_metal() && other.is_metal()) {
            return std::cmp::Ordering::Equal;
        }
        return self.metal_id().cmp(&other.metal_id());
    }
    pub fn next_metal(&self) -> Self {
        match self {
            Element::LEAD => Element::TIN,
            Element::TIN => Element::IRON,
            Element::IRON => Element::COPPER,
            Element::COPPER => Element::SILVER,
            Element::SILVER => Element::GOLD,
            Element::GOLD => Element::EMPTY,
            _ => {
                panic!("Not a metal")
            }
        }
    }
    pub fn is_element(&self) -> bool {
        match self {
            Element::AIR | Element::FIRE | Element::WATER | Element::PLANT => true,
            _ => false,
        }
    }
    pub fn is_metal(&self) -> bool {
        match self {
            Element::LEAD
            | Element::TIN
            | Element::IRON
            | Element::COPPER
            | Element::SILVER
            | Element::GOLD => true,
            _ => false,
        }
    }
    pub fn can_match(&self, other: Element) -> bool {
        if *self == Element::SALT && *self == other {
            return true;
        }
        if (*self == Self::QUICKSILVER || *self == Self::SALT)
            && !(other == Self::QUICKSILVER || other == Self::SALT)
        {
            return other.can_match(*self);
        }
        if self.is_metal() && other == Self::QUICKSILVER && *self != Self::GOLD {
            return true;
        }
        if (self.is_element() && other == Self::SALT) || *self == other {
            return true;
        }
        if (*self == Self::VITAE && other == Self::MORT)
            || (*self == Self::MORT && other == Self::VITAE)
        {
            return true;
        }
        return false;
    }
}

