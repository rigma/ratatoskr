use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub enum Protocol {
    AMQP {
        address: Option<String>,
        link_name: Option<String>,
        link_properties: Option<HashMap<String, String>>,
        sender_settlement_mode: String,
    },
    ApacheKafka {
        acks: Option<String>,
        client_id: Option<String>,
        partition_key_extractor: String,
        topic_name: Option<String>,
    },
    HTTP {
        method: String,
        headers: Option<HashMap<String, String>>,
    },
    MQTT {
        expiry: u64,
        qos: u8,
        retain: bool,
        topic_name: String,
        user_properties: Option<HashMap<String, String>>,
    },
    NATS {
        subject: String,
    },
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AMQP {
                ref address,
                ref link_name,
                ref sender_settlement_mode,
                ..
            } => write!(
                f,
                "AMQP (address={}, link_name={}, sender_settlement_mode={})",
                if let Some(address) = address {
                    &address[..]
                } else {
                    "None"
                },
                if let Some(link_name) = link_name {
                    &link_name[..]
                } else {
                    "None"
                },
                *sender_settlement_mode
            ),
            Self::ApacheKafka {
                ref acks,
                ref client_id,
                ref partition_key_extractor,
                ref topic_name,
            } => write!(
                f,
                "Apache Kafka (acks={}, client_id={}, partition_key={}, topic_name={})",
                if let Some(acks) = acks {
                    &acks[..]
                } else {
                    "None"
                },
                if let Some(client_id) = client_id {
                    &client_id[..]
                } else {
                    "None"
                },
                *partition_key_extractor,
                if let Some(topic_name) = topic_name {
                    &topic_name[..]
                } else {
                    "None"
                }
            ),
            Self::HTTP { ref method, .. } => write!(f, "HTTP (method={})", *method),
            Self::MQTT {
                ref expiry,
                ref qos,
                ref retain,
                ref topic_name,
                ..
            } => write!(
                f,
                "MQTT (expiry={}, qos={}, retain={}, topic_name={})",
                *expiry, *qos, *retain, *topic_name
            ),
            Self::NATS { ref subject } => write!(f, "NATS (subject={})", *subject),
        }
    }
}
