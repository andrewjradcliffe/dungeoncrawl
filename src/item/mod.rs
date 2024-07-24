pub mod consumable;
pub mod equipment;
pub mod equipment_bag;

use self::consumable::*;
use self::equipment::*;

pub enum Item {
    Consumable(Consumable),
    Gear(Gear),
}
