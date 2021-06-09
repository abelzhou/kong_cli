mod replace;
mod entity;
mod change;
mod search;

pub use replace::replace;
pub use entity::TargetList;
pub use entity::Target;
pub use entity::UpstreamList;
pub use entity::Upstream;
pub use change::change_upstream_weight;
pub use search::get_all_upstream;
pub use search::search_all_by_ip;
pub use search::search_target_from_upstream;