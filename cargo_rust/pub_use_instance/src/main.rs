mod art {
    pub mod kinds {
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }
    }

    pub mod utils {
        use crate::art::kinds::PrimaryColor;

        pub fn mix(c1: PrimaryColor, c2: PrimaryColor) {
            // mix implementation
        }
    }
}

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
