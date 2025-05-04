use super::effect::Effect;



#[derive(Debug, PartialEq)]
pub(crate) enum Product {
    OgKush,
    SourDiesel,
    GreenCrack,
    GrandaddyPurple,
    Meth,
    Cocaine,
}

impl Product {
    pub(crate) fn try_from_str(input: &str) -> Result<Self, String> {
        match input {
            "OG Kush" => Ok(Self::OgKush),
            "Sour Diesel" => Ok(Self::SourDiesel),
            "Green Crack" => Ok(Self::GreenCrack),
            "Grandaddy Purple" => Ok(Self::GrandaddyPurple),
            "Meth" => Ok(Self::Meth),
            "Cocaine" => Ok(Self::Cocaine),
            input => Err(format!("Unknown product: {}", input))
        }
    }

    pub(crate) fn get_base_price(&self) -> f32 {
        match self {
            Self::OgKush => 35.0,
            Self::SourDiesel => 40.0,
            Self::GreenCrack => 46.0,
            Self::GrandaddyPurple => 46.0,
            Self::Meth => 70.0,
            Self::Cocaine => 150.0,
        }
    }

    pub(crate) fn get_base_effect(&self) -> Option<Effect> {
        match self {
            Self::OgKush => Some(Effect::Calming),
            Self::SourDiesel => Some(Effect::Refreshing),
            Self::GreenCrack => Some(Effect::Energizing),
            Self::GrandaddyPurple => Some(Effect::Sedating),
            Self::Meth => None,
            Self::Cocaine => None,
        }
    }
}
