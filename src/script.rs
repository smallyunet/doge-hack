use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::Builder as ScriptBuilder;
use bitcoin::hashes::{hash160, Hash};
use bitcoin::script::ScriptBuf;

#[derive(Debug)]
pub enum ScriptError {
    InvalidThreshold { m: u8, n: u8 },
    InvalidPubkeyLength(usize),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::InvalidThreshold { m, n } => write!(f, "invalid multisig threshold: m={m}, n={n}"),
            ScriptError::InvalidPubkeyLength(len) => write!(f, "invalid compressed pubkey length: {len}, expected 33"),
        }
    }
}

impl std::error::Error for ScriptError {}

fn op_n(n: u8) -> opcodes::Opcode {
    match n {
        0 => opcodes::all::OP_PUSHBYTES_0,
        1 => opcodes::all::OP_PUSHNUM_1,
        2 => opcodes::all::OP_PUSHNUM_2,
        3 => opcodes::all::OP_PUSHNUM_3,
        4 => opcodes::all::OP_PUSHNUM_4,
        5 => opcodes::all::OP_PUSHNUM_5,
        6 => opcodes::all::OP_PUSHNUM_6,
        7 => opcodes::all::OP_PUSHNUM_7,
        8 => opcodes::all::OP_PUSHNUM_8,
        9 => opcodes::all::OP_PUSHNUM_9,
        10 => opcodes::all::OP_PUSHNUM_10,
        11 => opcodes::all::OP_PUSHNUM_11,
        12 => opcodes::all::OP_PUSHNUM_12,
        13 => opcodes::all::OP_PUSHNUM_13,
        14 => opcodes::all::OP_PUSHNUM_14,
        15 => opcodes::all::OP_PUSHNUM_15,
        16 => opcodes::all::OP_PUSHNUM_16,
        _ => opcodes::all::OP_RETURN,
    }
}

/// Build a standard legacy multisig redeem script: m <pubkeys...> n OP_CHECKMULTISIG
///
/// Notes:
/// - Expects compressed pubkeys (33 bytes).
/// - Order of pubkeys affects address.
pub fn multisig_redeem_script(m: u8, pubkeys: &[Vec<u8>]) -> Result<ScriptBuf, ScriptError> {
    let n = pubkeys.len() as u8;
    if m == 0 || n == 0 || m > n || n > 16 {
        return Err(ScriptError::InvalidThreshold { m, n });
    }

    let mut b = ScriptBuilder::new().push_opcode(op_n(m));
    for pk in pubkeys {
        if pk.len() != 33 {
            return Err(ScriptError::InvalidPubkeyLength(pk.len()));
        }
        b = b.push_slice(<&bitcoin::script::PushBytes>::try_from(pk.as_slice()).expect("valid push bytes"));
    }

    Ok(b
        .push_opcode(op_n(n))
        .push_opcode(opcodes::all::OP_CHECKMULTISIG)
        .into_script())
}

/// P2SH scriptPubKey: OP_HASH160 <hash160(redeem_script)> OP_EQUAL
pub fn p2sh_script_pubkey(redeem_script: &ScriptBuf) -> ScriptBuf {
    let h = hash160::Hash::hash(redeem_script.as_bytes());
    ScriptBuilder::new()
        .push_opcode(opcodes::all::OP_HASH160)
        .push_slice(<&bitcoin::script::PushBytes>::try_from(h.as_byte_array().as_slice()).expect("valid push bytes"))
        .push_opcode(opcodes::all::OP_EQUAL)
        .into_script()
}

pub fn redeem_script_hash160(redeem_script: &ScriptBuf) -> [u8; 20] {
    let h = hash160::Hash::hash(redeem_script.as_bytes());
    *h.as_byte_array()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multisig_redeem_script_2of3() {
        let pubkeys = vec![vec![0x02u8; 33], vec![0x03u8; 33], vec![0x02u8; 33]];
        let script = multisig_redeem_script(2, &pubkeys).unwrap();
        assert!(script.as_bytes().len() > 0);

        let p2sh = p2sh_script_pubkey(&script);
        assert!(p2sh.as_bytes().len() > 0);

        let h = redeem_script_hash160(&script);
        assert_eq!(h.len(), 20);
    }
}
