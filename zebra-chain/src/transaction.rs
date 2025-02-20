//! Transaction types.

use std::io;

use crate::serialization::{SerializationError, ZcashDeserialize, ZcashSerialize};
use crate::sha256d_writer::Sha256dWriter;

/// A hash of a `Transaction`
///
/// TODO: I'm pretty sure this is also a SHA256d hash but I haven't
/// confirmed it yet.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TransactionHash(pub [u8; 32]);

impl From<Transaction> for TransactionHash {
    fn from(transaction: Transaction) -> Self {
        let mut hash_writer = Sha256dWriter::default();
        transaction
            .zcash_serialize(&mut hash_writer)
            .expect("Transactions must serialize into the hash.");
        Self(hash_writer.finish())
    }
}

/// OutPoint
///
/// A particular transaction output reference.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OutPoint {
    /// References the transaction that contains the UTXO being spent.
    pub hash: TransactionHash,

    /// Identifies which UTXO from that transaction is referenced; the
    /// first output is 0, etc.
    pub index: u32,
}

/// Transaction Input
// `Copy` cannot be implemented for `Vec<u8>`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionInput {
    /// The previous output transaction reference.
    pub previous_output: OutPoint,

    /// Computational Script for confirming transaction authorization.
    // XXX pzec uses their own `Bytes` type that wraps a `Vec<u8>`
    // with some extra methods.
    pub signature_script: Vec<u8>,

    /// Transaction version as defined by the sender. Intended for
    /// "replacement" of transactions when information is updated
    /// before inclusion into a block.
    pub sequence: u32,
}

/// Transaction Output
///
/// The most fundamental building block of a transaction is a
/// transaction output -- the ZEC you own in your "wallet" is in
/// fact a subset of unspent transaction outputs (or "UTXO"s) of the
/// global UTXO set.
///
/// UTXOs are indivisible, discrete units of value which can only be
/// consumed in their entirety. Thus, if I want to send you 1 ZEC and
/// I only own one UTXO worth 2 ZEC, I would construct a transaction
/// that spends my UTXO and sends 1 ZEC to you and 1 ZEC back to me
/// (just like receiving change).
// `Copy` cannot be implemented for `Vec<u8>`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionOutput {
    /// Transaction value.
    // At https://en.bitcoin.it/wiki/Protocol_documentation#tx, this is an i64.
    pub value: u64,

    /// Usually contains the public key as a Bitcoin script setting up
    /// conditions to claim this output.
    pub pk_script: Vec<u8>,
}

/// Transaction
///
/// A transaction is an encoded data structure that facilitates the
/// transfer of value between two public key addresses on the Zcash
/// ecosystem. Everything is designed to ensure that transactions can
/// created, propagated on the network, validated, and finally added
/// to the global ledger of transactions (the blockchain).
// This is not up to date with the data included in the Zcash
// transaction format: https://zips.z.cash/protocol/protocol.pdf
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    /// Transaction data format version (note, this is signed).
    pub version: i32,

    /// A list of 1 or more transaction inputs or sources for coins.
    pub tx_in: Vec<TransactionInput>,

    /// A list of 1 or more transaction outputs or destinations for coins.
    pub tx_out: Vec<TransactionOutput>,

    /// The block number or timestamp at which this transaction is unlocked:
    ///
    /// |Value       |Description                                         |
    /// |------------|----------------------------------------------------|
    /// |0           |Not locked (default)                                |
    /// |< 500000000 |Block number at which this transaction is unlocked  |
    /// |>= 500000000|UNIX timestamp at which this transaction is unlocked|
    ///
    /// If all `TransactionInput`s have final (0xffffffff) sequence
    /// numbers, then lock_time is irrelevant. Otherwise, the
    /// transaction may not be added to a block until after `lock_time`.
    pub lock_time: u32,
}

impl ZcashSerialize for Transaction {
    fn zcash_serialize<W: io::Write>(&self, _writer: W) -> Result<(), SerializationError> {
        unimplemented!();
    }
}

impl ZcashDeserialize for Transaction {
    fn zcash_deserialize<R: io::Read>(_reader: R) -> Result<Self, SerializationError> {
        unimplemented!();
    }
}
