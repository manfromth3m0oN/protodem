pub mod demo {
    include!(concat!(env!("OUT_DIR"), "/cstrike15_gcmessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/cstrike15_usermessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/engine_gcmessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/netmessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/steammessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/gcsdk_gcmessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/networkbasetypes.rs"));
    include!(concat!(env!("OUT_DIR"), "/network_connection.rs"));
    include!(concat!(env!("OUT_DIR"), "/demo.rs"));
    include!(concat!(env!("OUT_DIR"), "/gameevents.rs"));
    include!(concat!(env!("OUT_DIR"), "/usermessages.rs"));
    include!(concat!(env!("OUT_DIR"), "/cs_gameevents.rs"));
    include!(concat!(env!("OUT_DIR"), "/te.rs"));
}
