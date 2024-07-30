use jsmv_crypto::public_key_hash::PublicKeyHash;
use jsmv_proto::operation::{external::Deposit, ExternalOperation, SignedOperation};
use mavryk_crypto_rs::hash::ContractKt1Hash;
use mavryk_smart_rollup::{
    inbox::{InboxMessage, InternalInboxMessage, Transfer},
    michelson::{ticket::UnitTicket, MichelsonBytes, MichelsonPair},
    prelude::{debug_msg, Runtime},
    types::Contract,
};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

pub type ExternalMessage = SignedOperation;
pub type InternalMessage = ExternalOperation;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    External(ExternalMessage),
    Internal(InternalMessage),
}

// reciever, ticket
type RollupType = MichelsonPair<MichelsonBytes, UnitTicket>;

fn read_transfer(
    rt: &mut impl Runtime,
    transfer: Transfer<RollupType>,
    ticketer: Option<&ContractKt1Hash>,
) -> Option<Message> {
    debug_msg!(rt, "Internal message: transfer\n");

    let ticketer = match ticketer {
        Some(ticketer) => ticketer,
        None => {
            debug_msg!(
                rt,
                "Deposit ignored because of different smart rollup address"
            );
            return None;
        }
    };

    if transfer.destination.hash().0 != &rt.reveal_metadata().raw_rollup_address {
        debug_msg!(
            rt,
            "Deposit ignored because of different smart rollup address"
        );
        return None;
    };

    let ticket = transfer.payload.1;

    match &ticket.creator().0 {
        Contract::Originated(kt1) if kt1 == ticketer => (),
        _ => {
            debug_msg!(rt, "Deposit ignored because of different ticketer");
            return None;
        }
    }

    let amount = ticket.amount().to_u64()?;

    let pkh_bytes = transfer.payload.0 .0;
    let reciever = PublicKeyHash::from_slice(&pkh_bytes).ok()?;

    let content = Deposit { amount, reciever };

    debug_msg!(rt, "Deposit: {content:?}\n");

    Some(Message::Internal(InternalMessage::Deposit(content)))
}

fn read_external_message(rt: &mut impl Runtime, bytes: &[u8]) -> Option<ExternalMessage> {
    let msg: ExternalMessage = bincode::deserialize(bytes).ok()?;
    debug_msg!(rt, "External message: {msg:?}\n");
    Some(msg)
}

pub fn read_message(
    rt: &mut impl Runtime,
    ticketer: Option<&ContractKt1Hash>,
) -> Option<Message> {
    let input = rt.read_input().ok()??;
    let _ = rt.mark_for_reboot();

    let (_, message) = InboxMessage::<RollupType>::parse(input.as_ref()).ok()?;
    debug_msg!(rt, "Message: {message:?}\n");

    match message {
        InboxMessage::Internal(InternalInboxMessage::StartOfLevel) => {
            // Start of level message pushed by the Layer 1 at the
            // beginning of eavh level.
            debug_msg!(rt, "Internal message: start of level\n");
            None
        }
        InboxMessage::Internal(InternalInboxMessage::InfoPerLevel(info)) => {
            // The "Info per level" messages follows the "Start of level"
            // message and contains information on the previous Layer 1 block.
            debug_msg!(
                rt,
                "Internal message: level info \
                        (block predecessor: {}, predecessor_timestamp: {}\n",
                info.predecessor,
                info.predecessor_timestamp
            );
            None
        }
        InboxMessage::Internal(InternalInboxMessage::EndOfLevel) => {
            // The "End of level" message is pushed by the Layer 1
            // at the end of each level.
            debug_msg!(rt, "Internal message: end of level\n");
            None
        }
        InboxMessage::Internal(InternalInboxMessage::Transfer(transfer)) => {
            read_transfer(rt, transfer, ticketer)
        }
        InboxMessage::External(bytes) => match read_external_message(rt, bytes) {
            Some(msg) => Some(Message::External(msg)),
            None => {
                debug_msg!(rt, "Failed to parse external message\n");
                None
            }
        },
    }
}
