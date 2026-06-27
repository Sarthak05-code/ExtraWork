use std::io::{self, Write};

// ─── Encoder ────────────────────────────────────────────────────────────────

/// Encodes a full sentence word-by-word.
/// Each character in a word becomes its ASCII value.
/// Characters within a word are joined by '-'.
/// Words are separated by ' | '.
///
/// Example: "Hi Bob" → "72-105 | 66-111-98"
fn encode(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            word.bytes()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join("-")
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

// ─── Decoder ────────────────────────────────────────────────────────────────

/// Decodes a cryptic string back to human-readable text.
/// Splits on ' | ' to get words, then on '-' to get ASCII values.
fn decode(input: &str) -> Result<String, String> {
    input
        .split(" | ")
        .map(|word_code| {
            word_code
                .split('-')
                .map(|num| {
                    let ascii: u8 = num
                        .trim()
                        .parse()
                        .map_err(|_| format!("Invalid code: '{}'", num))?;
                    if ascii > 127 {
                        return Err(format!("Value {} is out of ASCII range", ascii));
                    }
                    Ok(ascii as char)
                })
                .collect::<Result<String, String>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|words| words.join(" "))
}

// ─── Display helpers ─────────────────────────────────────────────────────────

fn print_banner() {
    println!("╔══════════════════════════════════════════╗");
    println!("║       CRYPTIC TEXT  ·  ASCII Encoder     ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
}

fn print_menu() {
    println!("  [1]  Encode  →  Human text   to  Cryptic");
    println!("  [2]  Decode  →  Cryptic code to  Human");
    println!("  [3]  Exit");
    println!();
    print!("  Choice: ");
    io::stdout().flush().unwrap();
}

fn read_line(prompt: &str) -> String {
    print!("  {}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn show_encoded(original: &str, encoded: &str) {
    println!();
    println!("  ┌─ Original ────────────────────────────");
    println!("  │  {}", original);
    println!("  ├─ Cryptic ─────────────────────────────");
    println!("  │  {}", encoded);
    println!("  └───────────────────────────────────────");

    println!();
    println!("  ┌─ Word Breakdown ───────────────────────");
    for (i, (word, code)) in original
        .split_whitespace()
        .zip(encoded.split(" | "))
        .enumerate()
    {
        println!("  │  Word {:>2}: {:15}  →  {}", i + 1, word, code);
    }
    println!("  └───────────────────────────────────────");
}

fn show_decoded(cryptic: &str, decoded: &str) {
    println!();
    println!("  ┌─ Cryptic ──────────────────────────────");
    println!("  │  {}", cryptic);
    println!("  ├─ Decoded ──────────────────────────────");
    println!("  │  {}", decoded);
    println!("  └───────────────────────────────────────");
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() {
    print_banner();

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let input = read_line("Enter sentence: ");
                if input.is_empty() {
                    println!("  ⚠  Nothing to encode.\n");
                    continue;
                }
                let encoded = encode(&input);
                show_encoded(&input, &encoded);
                println!();
            }
            "2" => {
                let input = read_line("Enter cryptic code: ");
                if input.is_empty() {
                    println!("  ⚠  Nothing to decode.\n");
                    continue;
                }
                match decode(&input) {
                    Ok(decoded) => show_decoded(&input, &decoded),
                    Err(e) => println!("  ✗  Error: {}\n", e),
                }
                println!();
            }
            "3" | "q" | "quit" => {
                println!("  Goodbye!\n");
                break;
            }
            _ => println!("  ✗  Invalid choice. Pick 1, 2, or 3.\n"),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_single_word() {
        assert_eq!(encode("Hi"), "72-105");
    }

    #[test]
    fn test_encode_sentence() {
        assert_eq!(encode("Hi Bob"), "72-105 | 66-111-98");
    }

    #[test]
    fn test_decode_single_word() {
        assert_eq!(decode("72-105").unwrap(), "Hi");
    }

    #[test]
    fn test_decode_sentence() {
        assert_eq!(decode("72-105 | 66-111-98").unwrap(), "Hi Bob");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Hello World from Rust";
        assert_eq!(decode(&encode(original)).unwrap(), original);
    }

    #[test]
    fn test_roundtrip_punctuation() {
        let original = "Hey, what's up?";
        assert_eq!(decode(&encode(original)).unwrap(), original);
    }

    #[test]
    fn test_invalid_code() {
        assert!(decode("999-abc").is_err());
    }
}