use effect::Effect;

pub(super) mod mapping;
pub(super) mod effect;
pub(super) mod mix;
pub(super) mod product;
pub(super) mod substance;

trait Mixable {
    fn get_cost(&self) -> f32;
    fn get_replacement_effect(&self, effect: &Effect) -> Option<Effect>;
    fn get_default_effect(&self) -> Effect;
}
