use clap::Parser;
use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "midium")]
#[command(about = "Sends a MIDI note on and off message")]
struct Cli {
    #[arg(short, long)]
    note: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let midi_out = MidiOutput::new("MIDI Output")?;
    let out_ports = midi_out.ports();

    if out_ports.is_empty() {
        eprintln!("No MIDI output ports available.");
        return Ok(());
    }

    let port = &out_ports[0];
    let mut conn_out = midi_out.connect(port, "midir-test")?;

    send_note_on(&mut conn_out, cli.note)?;
    sleep(Duration::from_secs(1));
    send_note_off(&mut conn_out, cli.note)?;

    Ok(())
}

fn send_note_on(conn_out: &mut MidiOutputConnection, note: u8) -> Result<(), Box<dyn Error>> {
    conn_out.send(&[0x90, note, 0x64])?; // Note on (channel 1, note, velocity)
    println!("Note on: {}", note);
    Ok(())
}

fn send_note_off(conn_out: &mut MidiOutputConnection, note: u8) -> Result<(), Box<dyn Error>> {
    conn_out.send(&[0x80, note, 0x64])?; // Note off (channel 1, note, velocity)
    println!("Note off: {}", note);
    Ok(())
}
