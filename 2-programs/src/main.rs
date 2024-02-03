use std::env;
// ATM machine
fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("🚧 Please provide the amount to withdraw.");
        return;
    }

    // use match to handle the parse
    let typed_amount: &String = &args[1];
    let amount_to_withdraw: u16 = match typed_amount.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("🚧 Please provide a valid amount to withdraw.");
            return;
        }
    };

    // Early return if amount_to_withdraw is zero
    if amount_to_withdraw == 0 {
        println!("🕳️ Nothing to withdraw.");
        return;
    }
    // maximum allowed amount to withdraw
    const MAX_AMOUNT_TO_WITHDRAW: u16 = 1000;
    // Early return if amount_to_withdraw is greater than MAX_AMOUNT_TO_WITHDRAW
    if amount_to_withdraw > MAX_AMOUNT_TO_WITHDRAW as u16 {
        println!("🚧 Amount to withdraw is greater than maximum allowed.");
        return;
    }

    // Array of available notes values
    const COUNT_AVAILABLE_NOTE_VALUES: usize = 6;
    let available_note_values: [u8; COUNT_AVAILABLE_NOTE_VALUES] = [200, 100, 50, 20, 10, 5];

    // Early return if amount_to_withdraw is not multiple of the minimum note value
    let min_note_value: u8 = available_note_values[COUNT_AVAILABLE_NOTE_VALUES - 1];
    if amount_to_withdraw % (min_note_value as u16) != 0 {
        println!("🚧 Amount to withdraw is not multiple of the minimum note value.");
        return;
    }

    // Struct to store the notes of a given value and quantity
    struct WadOfNotes {
        value: u8,
        quantity: u8,
    }
    // Vector of notes of a given value to withdraw in wallet
    let mut wallet: Vec<WadOfNotes> = Vec::new();

    let mut pending_amount: u16 = amount_to_withdraw;
    // Iterate over available_note_values
    for &note_value in available_note_values.iter() {
        // Calculate the number of notes to withdraw
        let quantity: u8 = (pending_amount / note_value as u16) as u8;
        // early return if notes_to_withdraw is zero
        if quantity == 0 {
            continue;
        }
        // Add a wad of notes to wallet
        let wad: WadOfNotes = WadOfNotes {
            value: note_value,
            quantity,
        };
        wallet.push(wad);
        // Update pending_amount
        let wad_value: u16 = quantity as u16 * note_value as u16;
        pending_amount -= wad_value;
    }
    if pending_amount != 0 {
        println!("🔥 Error: pending_amount is not zero.");
        return;
    }

    // traverse wallet and print each wad with note details
    let mut index: usize = 0;
    while index < wallet.len() {
        let wad: &WadOfNotes = &wallet[index];
        println!("💸 {} notes of {}.", wad.quantity, wad.value);
        index += 1;
    }
}
