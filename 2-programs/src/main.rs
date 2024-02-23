use std::env;
// ATM machine
fn main() {
    // Get the command line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Early return if no arguments are provided
    if args.len() < 2 {
        println!("🚧 Please provide the amount to withdraw.");
        return;
    }

    // Get the amount to withdraw by index position from the vector of arguments
    let typed_amount: &String = &args[1];

    // Returns an enum to represent the result of parsing a string to a number, either Ok or Err
    let parse_result: Result<u16, std::num::ParseIntError> = typed_amount.parse();

    // Match the parse result enum to either Ok or Err
    let amount_to_withdraw: u16 = match parse_result {
        Ok(n) => n, // Return the number if it's valid
        Err(_) => {
            println!("🚧 Please provide a valid amount number to withdraw.");
            return;
        }
    };

    // Early return if amount_to_withdraw is zero or is greater than MAX_AMOUNT_TO_WITHDRAW
    if amount_to_withdraw == 0 {
        println!("🕳️ Nothing to withdraw.");
        return;
    }
    const MAX_AMOUNT_TO_WITHDRAW: u16 = 1000;
    if amount_to_withdraw > MAX_AMOUNT_TO_WITHDRAW as u16 {
        println!("🚧 Amount to withdraw is greater than maximum allowed.");
        return;
    }

    // Array of available notes values
    const NUM_DISTINCT_NOTE_VALUES: usize = 6;
    let available_note_values: [u8; NUM_DISTINCT_NOTE_VALUES] = [200, 100, 50, 20, 10, 5];

    // Early return if amount_to_withdraw is not multiple of the minimum note value
    let min_note_value: u8 = available_note_values[NUM_DISTINCT_NOTE_VALUES - 1];
    if amount_to_withdraw % (min_note_value as u16) != 0 {
        println!("🚧 Amount to withdraw is not multiple of the minimum note value.");
        return;
    }

    // Struct to store a wad of notes of a given value
    struct WadOfNotes {
        value: u8,
        quantity: u8,
    }
    // Vector of wads of notes to keep in your wallet
    let mut wallet: Vec<WadOfNotes> = Vec::new();

    let mut pending_amount: u16 = amount_to_withdraw;

    // Iterate over available_note_values
    for &note_value in available_note_values.iter() {
        // Calculate the number of notes to withdraw of the current note value
        let quantity: u8 = (pending_amount / note_value as u16) as u8;
        // early return if nothing to withdraw
        if quantity == 0 {
            continue;
        }
        // Create a wad of notes of the current note value and quantity
        let wad: WadOfNotes = WadOfNotes {
            value: note_value,
            quantity,
        };
        // Push the wad to the wallet
        wallet.push(wad);
        // Update pending_amount
        let wad_value: u16 = (quantity as u16) * (note_value as u16);
        pending_amount -= wad_value;
    }
    if pending_amount != 0 {
        println!("🔥 Error: pending_amount is not zero.");
        return;
    }

    // traverse wallet and print each wad with note details
    let mut index: usize = 0;
    println!("💼 Save {} in to your wallet", amount_to_withdraw);
    while index < wallet.len() {
        let wad: &WadOfNotes = &wallet[index];
        println!("💸 A wad of {} notes of {}.", wad.quantity, wad.value);
        index += 1;
    }
}
