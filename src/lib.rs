use std::fmt;
use thiserror::Error;

// Custom errors for Bitcoin operations
#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Invalid transaction format")]
    InvalidTransaction,
    #[error("Invalid script format")]
    InvalidScript,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        // TODO: Implement constructor for Point
        Point { x, y }
    }
}

// Point methods for i32
impl Point<i32> {
    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Point methods for f64
impl Point<f64> {
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Custom serialization for Bitcoin transaction
pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        // TODO: Return a new builder for constructing a transaction
        LegacyTransactionBuilder::new()
    }
}

impl fmt::Display for LegacyTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "LegacyTransaction {{")?;
        writeln!(f, "  version: {}", self.version)?;
        writeln!(f, "  inputs: [")?;
        for input in &self.inputs {
            writeln!(f, "    {:?}", input)?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "  outputs: [")?;
        for output in &self.outputs {
            writeln!(f, "    {:?}", output)?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "  lock_time: {}", self.lock_time)?;
        write!(f, "}}")
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        // TODO: Implement default values
        LegacyTransactionBuilder {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        // TODO: Initialize new builder by calling default
        Self::default()
    }

    pub fn version(mut self, version: i32) -> Self {
        // TODO: Set the transaction version
        self.version = version;
        self
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        // TODO: Add input to the transaction
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        // TODO: Add output to the transaction
        self.outputs.push(output);
        self
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        // TODO: Set lock_time for transaction
        self.lock_time = lock_time;
        self
    }

    pub fn build(self) -> LegacyTransaction {
        // TODO: Build and return the final LegacyTransaction
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

impl TxInput {
    pub fn new(previous_output: OutPoint, script_sig: Vec<u8>, sequence: u32) -> Self {
        TxInput {
            previous_output,
            script_sig,
            sequence,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

impl TxOutput {
    pub fn new(value: u64, script_pubkey: Vec<u8>) -> Self {
        TxOutput {
            value,
            script_pubkey,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        OutPoint { txid, vout }
    }
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    // TODO: Match args to "send" or "balance" commands and parse required arguments
    if args.is_empty() {
        return Err(BitcoinError::ParseError("No command provided".to_string()));
    }

    match args[0].as_str() {
        "send" => {
            if args.len() < 3 {
                return Err(BitcoinError::ParseError(
                    "Send command requires amount and address".to_string(),
                ));
            }

            let amount = args[1]
                .parse::<u64>()
                .map_err(|_| BitcoinError::InvalidAmount)?;
            let address = args[2].clone();

            Ok(CliCommand::Send { amount, address })
        }
        "balance" => Ok(CliCommand::Balance),
        cmd => Err(BitcoinError::ParseError(format!(
            "Unknown command: {}",
            cmd
        ))),
    }
}

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

impl fmt::Display for CliCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliCommand::Send { amount, address } => {
                write!(f, "Send {{ amount: {}, address: {} }}", amount, address)
            }
            CliCommand::Balance => write!(f, "Balance"),
        }
    }
}

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
        if data.len() < 10 {
            return Err(BitcoinError::InvalidTransaction);
        }

        let mut offset = 0;

        // Parse version (4 bytes, little-endian)
        if data.len() < offset + 4 {
            return Err(BitcoinError::InvalidTransaction);
        }
        let version = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;
        // Parse input count (simplified - assume 1 byte for count)
        if data.len() < offset + 1 {
            return Err(BitcoinError::InvalidTransaction);
        }
        let input_count = data[offset] as usize;
        offset += 1;

        // For simplicity, create empty inputs (parsing full inputs would be complex)
        let inputs = vec![
            TxInput {
                previous_output: OutPoint::new([0; 32], 0),
                script_sig: vec![],
                sequence: 0xFFFFFFFF,
            };
            input_count
        ];

        // Parse output count (simplified - assume 1 byte for count)
        if data.len() < offset + 1 {
            return Err(BitcoinError::InvalidTransaction);
        }
        let output_count = data[offset] as usize;
        offset += 1;

        // For simplicity, create empty outputs
        let outputs = vec![
            TxOutput {
                value: 0,
                script_pubkey: vec![],
            };
            output_count
        ];

        // Parse lock_time (4 bytes, little-endian)
        if data.len() < offset + 4 {
            return Err(BitcoinError::InvalidTransaction);
        }
        let lock_time = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);

        Ok(LegacyTransaction {
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        // TODO: Serialize only version and lock_time (simplified)
        let mut bytes = Vec::new();

        // Serialize version (4 bytes, little-endian)
        bytes.extend_from_slice(&self.version.to_le_bytes());

        // Serialize lock_time (4 bytes, little-endian)
        bytes.extend_from_slice(&self.lock_time.to_le_bytes());

        bytes
    }
}
