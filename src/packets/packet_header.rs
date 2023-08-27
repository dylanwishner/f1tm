use std::mem::size_of;

pub struct PacketHeader {
    // 2023
    pub m_packet_format: u16,

    // Game year - last two digits e.g. 23
    pub m_game_year: u8,

    // Game major version - "X.00"
    pub m_game_major_version: u8,

    // Game minor version - "1.XX"
    pub m_game_minor_version: u8,

    // Version of this packet type, all start from 1
    pub m_packet_version: u8,

    // Identifier for the packet type, see below
    pub m_packet_id: u8,

    // Unique identifier for the session
    pub m_session_uid: u64,

    // Session timestamp
    pub m_session_time: f32,

    // Identifier for the frame the data was retrieved on
    pub m_frame_identifier: u32,

    // Overall identifier for the frame the data was retrieved on, doesn't go back after flashbacks
    pub m_overall_frame_identifier: u32,

    // Index of player's car in the array
    pub m_player_car_index: u8,

    // Index of secondary player's car in the array (splitscreen) 255 if no second player
    pub m_secondary_player_car_index: u8,
}

impl From<&[u8]> for PacketHeader {
    fn from(packet: &[u8]) -> Self {
        PacketHeader {
            m_packet_format: u16::from_le_bytes([packet[0], packet[1]]),
            m_game_year: u8::from_le_bytes([packet[2]]),
            m_game_major_version: u8::from_le_bytes([packet[3]]),
            m_game_minor_version: u8::from_le_bytes([packet[4]]),
            m_packet_version: u8::from_le_bytes([packet[5]]),
            m_packet_id: u8::from_le_bytes([packet[6]]),
            m_session_uid: u64::from_le_bytes([
                packet[7], packet[8], packet[9], packet[10], packet[11], packet[12], packet[13],
                packet[14],
            ]),
            m_session_time: f32::from_le_bytes([packet[15], packet[16], packet[17], packet[18]]),
            m_frame_identifier: u32::from_le_bytes([
                packet[19], packet[20], packet[21], packet[22],
            ]),
            m_overall_frame_identifier: u32::from_le_bytes([
                packet[23], packet[24], packet[25], packet[26],
            ]),
            m_player_car_index: u8::from_le_bytes([packet[27]]),
            m_secondary_player_car_index: u8::from_le_bytes([packet[28]]),
        }
    }
}

pub const PACKET_HEADER_SIZE: usize = size_of::<PacketHeader>();
