pub use crate::std_structs::node::Node;
pub use crate::std_structs::relationship::Relationship;
pub use crate::std_structs::unbound_relationship::UnboundRelationship;
pub use crate::std_structs::path::Path;
pub use crate::std_structs::date::Date;
pub use crate::std_structs::time::Time;
pub use crate::std_structs::local_time::LocalTime;
pub use crate::std_structs::date_time::DateTime;
pub use crate::std_structs::date_time_zone_id::DateTimeZoneId;
pub use crate::std_structs::local_date_time::LocalDateTime;
pub use crate::std_structs::duration::Duration;
pub use crate::std_structs::point2d::Point2D;
pub use crate::std_structs::point3d::Point3D;
use crate::*;

pub mod node;
pub mod relationship;
pub mod unbound_relationship;
pub mod path;
pub mod date;
pub mod time;
pub mod local_time;
pub mod date_time;
pub mod date_time_zone_id;
pub mod local_date_time;
pub mod duration;
pub mod point2d;
pub mod point3d;

#[derive(Debug, Clone, PartialEq, PackableStructSum)]
pub enum StdStruct {
    #[tag = 0x4E]
    Node(Node),
    #[tag = 0x52]
    Relationship(Relationship),
    #[tag = 0x72]
    UnboundRelationship(UnboundRelationship),
    #[tag = 0x50]
    Path(Path),
    #[tag = 0x44]
    Date(Date),
    #[tag = 0x54]
    Time(Time),
    #[tag = 0x74]
    LocalTime(LocalTime),
    #[tag = 0x46]
    DateTime(DateTime),
    #[tag = 0x66]
    DateTimeZoneId(DateTimeZoneId),
    #[tag = 0x64]
    LocalDateTime(LocalDateTime),
    #[tag = 0x45]
    Duration(Duration),
    #[tag = 0x58]
    Point2D(Point2D),
    #[tag = 0x59]
    Point3D(Point3D),
}