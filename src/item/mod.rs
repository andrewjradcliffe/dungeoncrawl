pub mod consumable;
pub mod equipment;

use self::consumable::*;
use self::equipment::*;

pub enum Item {
    Consumable(Consumable),
    Gear(Gear),
}
