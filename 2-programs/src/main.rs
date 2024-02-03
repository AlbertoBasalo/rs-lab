fn main() {
    // ATM machine

    // amount_to_withdraw
    let amount_to_withdraw: u16 = 123;

    // Array of available notes values
    const count_available_note_values: usize = 5;
    let available_note_values: [u8; count_available_note_values] = [100, 50, 20, 10, 5];

    // Vector of notes to withdraw
    let mut notes_to_withdraw: Vec<u8> = Vec::new();

    // Early return if amount_to_withdraw is zero
    if amount_to_withdraw == 0 {
        println!("Nothing to withdraw.");
        return;
    }
    // Early return if amount_to_withdraw is not multiple of the minimum note value
    let min_note_value: u8 = available_note_values[count_available_note_values - 1];
    if amount_to_withdraw % (min_note_value as u16) != 0 {
        println!("Amount to withdraw is not multiple of the minimum note value.");
        return;
    }
}
