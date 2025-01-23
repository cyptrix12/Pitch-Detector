pub fn closest_note_with_octave(frequency: f32) -> (String, i32) {
    
    let notes = [
        //Hz dla nut w najniższej oktawie
        // Ze strony MixButton
        ("C", 16.35),
        ("C#", 17.32),
        ("D", 18.35),
        ("D#", 19.45),
        ("E", 20.60),
        ("F", 21.83),
        ("F#", 23.12),
        ("G", 24.50),
        ("G#", 25.96),
        ("A", 27.50),
        ("A#", 29.14),
        ("B", 30.87),
    ];

    let mut closest_note = ("Unknown", 0.0);
    let mut closest_octave = 0;
    let mut min_difference = f32::MAX;

    for octave in 0..=8 { // Zakres oktaw, wyżej i niżej jest poza ludzkim uchem
        for &(note, base_frequency) in &notes {
            let adjusted_frequency = base_frequency * 2f32.powi(octave); // Częstotliwość nuty w danej oktawie
            let difference = (frequency - adjusted_frequency).abs();
            if difference < min_difference {
                min_difference = difference;
                closest_note = (note, adjusted_frequency);
                closest_octave = octave;
            }
        }
    }

    (closest_note.0.to_string(), closest_octave)
}
