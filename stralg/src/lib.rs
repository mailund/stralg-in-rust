pub mod patterns;
pub mod search;
pub mod utils;

pub use patterns::border_array;
pub use patterns::strict_border_array;
pub use search::kmp;
pub use search::naive;
pub use utils::AlphabetImpl;
