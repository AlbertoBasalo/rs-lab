fn main() {
    // ATM machine

    // amount_to_withdraw
    let amount_to_withdraw: u16 = 250;

    // Early return if amount_to_withdraw is zero
    if amount_to_withdraw == 0 {
        println!("Nothing to withdraw.");
        return;
    }
    // maximum allowed amount to withdraw
    const MAX_AMOUNT_TO_WITHDRAW: u16 = 1000;
    // Early return if amount_to_withdraw is greater than MAX_AMOUNT_TO_WITHDRAW
    if amount_to_withdraw > MAX_AMOUNT_TO_WITHDRAW as u16 {
        println!("Amount to withdraw is greater than maximum allowed.");
        return;
    }

    // Array of available notes values
    const COUNT_AVAILABLE_NOTE_VALUES: usize = 5;
    let available_note_values: [u8; COUNT_AVAILABLE_NOTE_VALUES] = [100, 50, 20, 10, 5];

    // Early return if amount_to_withdraw is not multiple of the minimum note value
    let min_note_value: u8 = available_note_values[COUNT_AVAILABLE_NOTE_VALUES - 1];
    if amount_to_withdraw % (min_note_value as u16) != 0 {
        println!("Amount to withdraw is not multiple of the minimum note value.");
        return;
    }

    // Vector of notes to withdraw
    #[derive(Debug)]
    struct NotesToWithdraw {
        note_value: u8,
        notes_of_value: u8,
    }
    let mut notes_to_withdraw: Vec<NotesToWithdraw> = Vec::new();
    // pending_amount_to_withdraw
    let mut pending_amount_to_withdraw: u16 = amount_to_withdraw;
    // Iterate over available_note_values
    for note_value in available_note_values.iter() {
        // Calculate the number of notes to withdraw
        let notes_of_value: u8 = (pending_amount_to_withdraw / *note_value as u16) as u8;
        // early return if notes_to_withdraw is zero
        if notes_of_value == 0 {
            continue;
        }
        // Add notes_of_value to notes_to_withdraw
        let notes_to_withdraw_item: NotesToWithdraw = NotesToWithdraw {
            note_value: *note_value,
            notes_of_value: notes_of_value,
        };
        notes_to_withdraw.push(notes_to_withdraw_item);
        // Update pending_amount_to_withdraw
        let notes_value: u16 = notes_of_value as u16 * *note_value as u16;
        pending_amount_to_withdraw -= notes_value;
    }
    if pending_amount_to_withdraw != 0 {
        println!("Error: pending_amount_to_withdraw is not zero.");
        return;
    }
    // traverse notes_to_withdraw and print each note details
    for note_to_withdraw in notes_to_withdraw.iter() {
        println!(
            "{} notes of {}.",
            note_to_withdraw.notes_of_value, note_to_withdraw.note_value
        );
    }
}
