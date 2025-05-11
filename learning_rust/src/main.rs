use std::io;

struct Note {
    title: String,
    content: String,
    date: String,
}
fn main() {
    let mut continuing = true;
    let mut input = String::new();
    let mut agenda = Vec::new();
    while continuing {
        input.clear();
        println!("Welcome to your agenda program!");
        println!("Please enter a command:");
        println!("1. Add a new note");
        println!("2. View all notes");
        println!("3. View a specific note");
        println!("4. Delete a note");
        println!("5. Exit");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" => {
                add_note(&mut agenda);
            }
            "2" => {
                view_all_notes(&agenda);
            }
            "3" => {
                view_specific_note(&agenda);
            }
            "4" => {
                delete_note(&mut agenda);
            }
            "5" => {
                continuing = false;
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
}

fn add_note(agenda: &mut Vec<Note>) {
    let mut note = Note {
        title: String::new(),
        content: String::new(),
        date: String::new(),
    };
    println!("Enter your new note's title:");
    io::stdin().read_line(&mut note.title).expect("Failed to read line");
    println!("Enter your new note's content:");
    io::stdin().read_line(&mut note.content).expect("Failed to read line");
    println!("When did you write this note? / type the date:");
    io::stdin().read_line(&mut note.date).expect("Failed to read line");
    agenda.push(note);
    println!("Note was added successfully!");
}

fn view_all_notes(agenda: &Vec<Note>) {
    if agenda.is_empty() {
        println!("No notes found.");
    } else {
        for (i, note) in agenda.iter().enumerate() {
            println!("Note {}: {}", i + 1, note.title);
            println!("Content: {}", note.content);
            println!("Date: {}", note.date);
            println!("-------------------------");
        }
    }
}

fn view_specific_note(agenda: &Vec<Note>) {
    println!("Enter the note number you want to view:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let note_number: usize = input.trim().parse().expect("Please enter a number");
    if note_number > 0 && note_number <= agenda.len() {
        let note = &agenda[note_number - 1];
        println!("Note {}: {}", note_number, note.title);
        println!("Content: {}", note.content);
        println!("Date: {}", note.date);
    } else {
        println!("Invalid note number.");
    }
}

fn delete_note(agenda: &mut Vec<Note>) {
    println!("Enter the note number you want to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let note_number: usize = input.trim().parse().expect("Please enter a number");
    if note_number > 0 && note_number <= agenda.len() {
        agenda.remove(note_number - 1);
        println!("Note {} was deleted successfully!", note_number);
    } else {
        println!("Invalid note number.");
    }
}