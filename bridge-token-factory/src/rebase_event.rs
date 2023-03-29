use bridge_common::prover::{EthAddress, EthEventParams, EthRebaseEvent};
use ethabi::{ParamType, Token};
use near_sdk::Balance;

/// Data that was emitted by the Ethereum Locked event.
#[derive(Debug, Eq, PartialEq)]
pub struct EthRebasedEvent {
    pub rebaser_address: EthAddress,
    pub epoch: Balance,
    pub exchange_rate: Balance,
    pub cpi: Balance,
    pub requested_supply_adjustment: Balance,
    pub timestamp_sec: Balance,
}

impl EthRebasedEvent {
    fn event_params() -> EthEventParams {
        vec![
            ("epoch".to_string(), ParamType::Uint(256), true),
            ("exchange_rate".to_string(), ParamType::Uint(256), false),
            ("cpi".to_string(), ParamType::Uint(256), false),
            (
                "requested_supply_adjustment".to_string(),
                ParamType::Int(256),
                false,
            ),
            ("timestamp_sec".to_string(), ParamType::Uint(256), false),
        ]
    }

    /// Parse raw log entry data.
    pub fn from_log_entry_data(data: &[u8]) -> Self {
        let event =
            EthRebaseEvent::from_log_entry_data("Rebased", EthRebasedEvent::event_params(), data);
        let epoch = event.log.params[0]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let exchange_rate = event.log.params[1]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let cpi = event.log.params[2]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let requested_supply_adjustment = event.log.params[3]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let timestamp_sec = event.log.params[4]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();

        Self {
            rebaser_address: event.rebaser_address,
            epoch,
            exchange_rate,
            cpi,
            requested_supply_adjustment,
            timestamp_sec,
        }
    }

    pub fn to_log_entry_data(&self) -> Vec<u8> {
        EthRebaseEvent::to_log_entry_data(
            "Rebased",
            EthRebasedEvent::event_params(),
            self.rebaser_address,
            vec![vec![u8::try_from(self.epoch).unwrap()]],
            vec![
                Token::Uint(self.requested_supply_adjustment.into()),
                Token::Uint(self.timestamp_sec.into()),
            ],
        )
    }
}

impl std::fmt::Display for EthRebasedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "epoch: {}; exchange_rate: {}; cpi: {}; requested_supply_adjustment: {}; timestamp_sec: {}" ,
            self.epoch, self.exchange_rate, self.cpi, self.requested_supply_adjustment, self.timestamp_sec,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_data() {
        let event_data = EthRebasedEvent {
            rebaser_address: [0u8; 20],
            epoch: 100,
            exchange_rate: 1_250_000_000_000_000_000,
            cpi: 1_100_000_000_000_000_000,
            requested_supply_adjustment: 1_000_000,
            timestamp_sec: 123_456_789,
        };
        let data = event_data.to_log_entry_data();
        let result = EthRebasedEvent::from_log_entry_data(&data);
        assert_eq!(result, event_data);
    }
}
