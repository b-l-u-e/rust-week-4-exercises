use rust_week_4_exercises::*;

fn main() {
    println!("🚀 Rust Week 4 Exercises - Bitcoin Library Demo\n");

    // Test Point generic struct
    println!("=== Point Generic Struct Tests ===");
    test_point_operations();

    // Test Legacy Transaction Builder Pattern
    println!("\n=== Legacy Transaction Builder Tests ===");
    test_transaction_builder();

    // Test CLI Argument Parsing
    println!("\n=== CLI Argument Parsing Tests ===");
    test_cli_parsing();

    // Test Serialization and Deserialization
    println!("\n=== Serialization/Deserialization Tests ===");
    test_serialization();

    // Test Error Handling
    println!("\n=== Error Handling Tests ===");
    test_error_handling();

    // Realistic Bitcoin Transaction Demo
    println!("\n=== Realistic Bitcoin Transaction Demo ===");
    demo_realistic_transaction();

    println!("\n✨ All demos completed successfully!");
}

fn test_point_operations() {
    // Test Point with different types
    let point_i32 = Point::new(10, 20);
    let point_f64 = Point::new(3.14, 2.71);
    let point_str = Point::new("x".to_string(), "y".to_string());

    println!("Integer Point: {:?}", point_i32);
    println!("Float Point: {:?}", point_f64);
    println!("String Point: {:?}", point_str);

    // Test Point methods
    let point1 = Point::new(0, 0);
    let point2 = Point::new(3, 4);
    let manhattan_dist = point1.manhattan_distance(&point2);
    println!(
        "Manhattan distance between {:?} and {:?}: {}",
        point1, point2, manhattan_dist
    );

    let point1_f = Point::new(0.0, 0.0);
    let point2_f = Point::new(3.0, 4.0);
    let euclidean_dist = point1_f.distance(&point2_f);
    println!(
        "Euclidean distance between {:?} and {:?}: {:.2}",
        point1_f, point2_f, euclidean_dist
    );

    // Test Point equality
    let point_a = Point::new(1, 2);
    let point_b = Point::new(1, 2);
    let point_c = Point::new(2, 1);
    println!(
        "Point equality: {:?} == {:?} -> {}",
        point_a,
        point_b,
        point_a == point_b
    );
    println!(
        "Point equality: {:?} == {:?} -> {}",
        point_a,
        point_c,
        point_a == point_c
    );
}

fn test_transaction_builder() {
    println!("Testing Legacy Transaction Builder Pattern...");

    // Create transaction components
    let outpoint = OutPoint::new([0xaa; 32], 0);
    let input = TxInput::new(
        outpoint,
        vec![0x76, 0xa9, 0x14], // Simple script
        0xFFFFFFFF,
    );

    let output = TxOutput::new(
        50000000,                           // 0.5 BTC in satoshis
        vec![0x76, 0xa9, 0x14, 0x88, 0xac], // P2PKH script
    );

    // Build transaction using builder pattern
    let transaction = LegacyTransaction::builder()
        .version(2)
        .add_input(input.clone())
        .add_output(output.clone())
        .lock_time(500000)
        .build();

    println!("Built transaction using builder pattern:");
    println!("{}", transaction);

    // Test multiple inputs/outputs
    let input2 = TxInput::new(
        OutPoint::new([0xbb; 32], 1),
        vec![0x47, 0x30, 0x44], // Different script
        0xFFFFFFFE,
    );

    let output2 = TxOutput::new(
        25000000,         // 0.25 BTC
        vec![0x00, 0x14], // P2WPKH script
    );

    let multi_tx = LegacyTransaction::builder()
        .version(1)
        .add_input(input)
        .add_input(input2)
        .add_output(output)
        .add_output(output2)
        .lock_time(0)
        .build();

    println!("\nMulti-input/output transaction:");
    println!("{}", multi_tx);

    // Test default builder
    let default_tx = LegacyTransactionBuilder::default().build();
    println!("Default transaction:");
    println!("{}", default_tx);
}

fn test_cli_parsing() {
    println!("Testing CLI argument parsing...");

    // Test valid commands
    let test_cases = vec![
        (
            vec![
                "send".to_string(),
                "100000".to_string(),
                "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            ],
            "Send command",
        ),
        (vec!["balance".to_string()], "Balance command"),
    ];

    for (args, description) in test_cases {
        match parse_cli_args(&args) {
            Ok(command) => {
                println!("✓ {} parsed successfully: {}", description, command);
            }
            Err(e) => {
                println!("✗ {} failed: {}", description, e);
            }
        }
    }

    // Test error cases
    let error_cases = vec![
        (vec![], "No arguments"),
        (vec!["unknown".to_string()], "Unknown command"),
        (vec!["send".to_string()], "Send without arguments"),
        (
            vec![
                "send".to_string(),
                "invalid".to_string(),
                "address".to_string(),
            ],
            "Send with invalid amount",
        ),
    ];

    for (args, description) in error_cases {
        match parse_cli_args(&args) {
            Ok(command) => {
                println!("✗ {} should have failed but got: {}", description, command);
            }
            Err(e) => {
                println!("✓ {} properly failed: {}", description, e);
            }
        }
    }
}

fn test_serialization() {
    println!("Testing serialization and deserialization...");

    // Create a transaction
    let transaction = LegacyTransaction::builder()
        .version(1)
        .add_input(TxInput::new(
            OutPoint::new([0xcc; 32], 0),
            vec![0x76, 0xa9],
            0xFFFFFFFF,
        ))
        .add_output(TxOutput::new(
            100000000, // 1 BTC
            vec![0x76, 0xa9, 0x14, 0x88, 0xac],
        ))
        .lock_time(0)
        .build();

    println!("Original transaction:");
    println!("{}", transaction);

    // Test serialization
    let serialized = transaction.serialize();
    println!(
        "Serialized to {} bytes: {:02x?}",
        serialized.len(),
        serialized
    );

    // Test deserialization with simple binary data
    let simple_binary = vec![
        0x01, 0x00, 0x00, 0x00, // version = 1
        0x01, // 1 input
        0x01, // 1 output
        0x00, 0x00, 0x00, 0x00, // lock_time = 0
    ];

    match LegacyTransaction::try_from(simple_binary.as_slice()) {
        Ok(parsed_tx) => {
            println!("✓ Successfully parsed transaction from binary:");
            println!("{}", parsed_tx);
        }
        Err(e) => {
            println!("✗ Failed to parse transaction: {}", e);
        }
    }

    // Test with insufficient data
    let insufficient_data = vec![0x01, 0x00]; // Only 2 bytes
    match LegacyTransaction::try_from(insufficient_data.as_slice()) {
        Ok(_) => {
            println!("✗ Should have failed with insufficient data");
        }
        Err(e) => {
            println!("✓ Correctly failed with insufficient data: {}", e);
        }
    }
}

fn test_error_handling() {
    println!("Testing error handling...");

    // Test different error types
    let errors = vec![
        BitcoinError::InvalidTransaction,
        BitcoinError::InvalidScript,
        BitcoinError::InvalidAmount,
        BitcoinError::ParseError("Custom parse error".to_string()),
    ];

    for error in errors {
        println!("Error: {}", error);
        println!("  Debug: {:?}", error);
    }

    // Test error propagation
    fn function_that_returns_error() -> Result<(), BitcoinError> {
        Err(BitcoinError::InvalidTransaction)
    }

    match function_that_returns_error() {
        Ok(_) => println!("Function succeeded"),
        Err(e) => println!("Function failed as expected: {}", e),
    }
}

fn demo_realistic_transaction() {
    println!("Creating a realistic Bitcoin transaction...");

    // Simulate spending from a previous transaction
    let prev_txid = [
        0x6f, 0x73, 0x08, 0xbb, 0xe9, 0x5c, 0x0f, 0x6e, 0x13, 0x01, 0xdd, 0x73, 0xa8, 0xda, 0x77,
        0xd2, 0x15, 0x5b, 0x07, 0x73, 0xbc, 0x29, 0x7a, 0xc4, 0x7f, 0x9c, 0xd7, 0x38, 0x00, 0x10,
        0x00, 0x00,
    ];

    // Create realistic P2PKH unlocking script (signature + pubkey)
    let mut script_sig = Vec::new();
    script_sig.push(0x47); // PUSH 71 bytes (signature)
    script_sig.extend_from_slice(&[0x30, 0x44, 0x02, 0x20]); // DER signature start
    script_sig.extend_from_slice(&[0x12; 32]); // r value (32 bytes)
    script_sig.extend_from_slice(&[0x02, 0x20]); // s value start
    script_sig.extend_from_slice(&[0x34; 32]); // s value (32 bytes)
    script_sig.push(0x01); // SIGHASH_ALL
    script_sig.push(0x21); // PUSH 33 bytes (compressed pubkey)
    script_sig.push(0x03); // Compressed pubkey prefix
    script_sig.extend_from_slice(&[0x56; 32]); // Pubkey (32 bytes)

    let input = TxInput::new(OutPoint::new(prev_txid, 0), script_sig, 0xFFFFFFFF);

    // Create P2PKH output script
    let mut p2pkh_script = Vec::new();
    p2pkh_script.push(0x76); // OP_DUP
    p2pkh_script.push(0xa9); // OP_HASH160
    p2pkh_script.push(0x14); // PUSH 20 bytes
    p2pkh_script.extend_from_slice(&[0x89; 20]); // Hash160 of pubkey
    p2pkh_script.push(0x88); // OP_EQUALVERIFY
    p2pkh_script.push(0xac); // OP_CHECKSIG

    let output1 = TxOutput::new(
        75000000, // 0.75 BTC to recipient
        p2pkh_script,
    );

    // Create change output (P2WPKH)
    let mut p2wpkh_script = Vec::new();
    p2wpkh_script.push(0x00); // OP_0
    p2wpkh_script.push(0x14); // PUSH 20 bytes
    p2wpkh_script.extend_from_slice(&[0xab; 20]); // Hash160 for change

    let output2 = TxOutput::new(
        24990000, // 0.2499 BTC change (0.0001 BTC fee)
        p2wpkh_script,
    );

    // Build the realistic transaction
    let realistic_tx = LegacyTransaction::builder()
        .version(2) // BIP 68
        .add_input(input)
        .add_output(output1)
        .add_output(output2)
        .lock_time(0)
        .build();

    println!("Realistic Bitcoin Transaction:");
    println!("{}", realistic_tx);

    // Calculate transaction details
    let total_input_value = 100000000u64; // Assume 1 BTC input
    let total_output_value: u64 = realistic_tx.outputs.iter().map(|o| o.value).sum();
    let fee = total_input_value - total_output_value;

    println!("\nTransaction Analysis:");
    println!(
        "  Total Input Value: {} satoshis ({:.8} BTC)",
        total_input_value,
        total_input_value as f64 / 100_000_000.0
    );
    println!(
        "  Total Output Value: {} satoshis ({:.8} BTC)",
        total_output_value,
        total_output_value as f64 / 100_000_000.0
    );
    println!(
        "  Transaction Fee: {} satoshis ({:.8} BTC)",
        fee,
        fee as f64 / 100_000_000.0
    );

    // Serialize the transaction
    let serialized = realistic_tx.serialize();
    println!("  Serialized Size: {} bytes", serialized.len());
    println!("  Serialized Data: {:02x?}", serialized);

    // Test CLI simulation
    println!("\nSimulating CLI commands:");
    let cli_args = vec![
        "send".to_string(),
        "75000000".to_string(),
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    ];

    match parse_cli_args(&cli_args) {
        Ok(cmd) => println!("CLI Command: {}", cmd),
        Err(e) => println!("CLI Error: {}", e),
    }
}
