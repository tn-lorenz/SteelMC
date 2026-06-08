use glam::DVec3;
use steel_macros::{ReadFrom, ServerPacket};
use steel_utils::types::InteractionHand;

/// Serverbound packet sent when a player right-clicks an entity.
#[derive(ReadFrom, ServerPacket, Clone, Debug)]
pub struct SInteract {
    #[read(as = VarInt)]
    pub entity_id: i32,

    pub hand: InteractionHand,

    pub location: DVec3,

    pub using_secondary_action: bool,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use steel_utils::serial::ReadFrom as _;

    use super::*;

    #[test]
    fn interact_packet_reads_vanilla_field_order() {
        let mut bytes = Vec::new();
        bytes.push(42);
        bytes.push(1);
        bytes.extend_from_slice(&0.25_f64.to_be_bytes());
        bytes.extend_from_slice(&0.5_f64.to_be_bytes());
        bytes.extend_from_slice(&0.75_f64.to_be_bytes());
        bytes.push(1);

        let packet = SInteract::read(&mut Cursor::new(&bytes))
            .unwrap_or_else(|error| panic!("interact packet should parse: {error}"));

        assert_eq!(packet.entity_id, 42);
        assert_eq!(packet.hand, InteractionHand::OffHand);
        assert_eq!(packet.location, DVec3::new(0.25, 0.5, 0.75));
        assert!(packet.using_secondary_action);
    }
}
