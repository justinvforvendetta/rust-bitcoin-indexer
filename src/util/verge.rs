use crate::{types::*, Hash160};
use verge::util::address;
use common_failures::prelude::*;
use failure::bail;

pub fn network_from_str(s: &str) -> Result<verge::Network> {
    Ok(match s {
        "main" => verge::Network::Verge,
        "test" => verge::Network::Testnet,
        "regtest" => verge::Network::Regtest,
        _ => bail!("Unknown or Incorrect Verge chain {}", s),
    })
}

fn bech_network(
    network: verge::network::constants::Network,
) -> verge_bech32::constants::Network {
    use verge::network::constants::Network;
    match network {
        Network::Verge => verge_bech32::constants::Network::Verge,
        Network::Testnet => verge_bech32::constants::Network::Testnet,
        Network::Regtest => verge_bech32::constants::Network::Regtest,
    }
}

/// Retrieve an address from the given script.
pub fn address_from_script(
    script: &verge::blockdata::script::Script,
    network: verge::network::constants::Network,
) -> Option<address::Address> {
    Some(address::Address {
        payload: if script.is_p2sh() {
            address::Payload::ScriptHash(
                Hash160::from_slice(&script.as_bytes()[2..22]).expect("correct data"),
            )
        } else if script.is_p2pkh() {
            address::Payload::PubkeyHash(
                Hash160::from_slice(&script.as_bytes()[3..23]).expect("correct data"),
            )
        } else if script.is_p2pk() {
            // no address format for p2kp
            return None;
        } else if script.is_v0_p2wsh() {
            address::Payload::WitnessProgram(
                verge_bech32::WitnessProgram::new(
                    verge_bech32::u5::try_from_u8(0).expect("0<32"),
                    script.as_bytes()[2..34].to_vec(),
                    bech_network(network),
                )
                .unwrap(),
            )
        } else if script.is_v0_p2wpkh() {
            address::Payload::WitnessProgram(
                verge_bech32::WitnessProgram::new(
                    verge_bech32::u5::try_from_u8(0).expect("0<32"),
                    script.as_bytes()[2..22].to_vec(),
                    bech_network(network),
                )
                .unwrap(),
            )
        } else {
            return None;
        },
        network,
    })
}
