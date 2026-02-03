# VVW - WAV Steganography Toolkit

A Rust CLI for embedding and extracting encrypted text and audio in WAV files.

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/vvw`.

## Quick Start

```bash
# Embed a message
vvw encode input.wav -o output.wav --message "Hello, world!"

# Extract the message
vvw decode output.wav
```

## Commands

### encode / decode

Embed and extract text from WAV files.

```bash
# Basic text embedding
vvw encode input.wav -o output.wav --message "secret message"
vvw decode output.wav

# From a file
vvw encode input.wav -o output.wav --message-file secret.txt
vvw decode output.wav

# Symmetric encryption (passphrase)
vvw encode input.wav -o output.wav --message "secret" --passphrase "puzzle"
vvw decode output.wav --passphrase "puzzle"

# Asymmetric encryption (public key)
vvw encode input.wav -o output.wav --message "secret" --encrypt-to alice.pub
vvw decode output.wav --key alice.priv

# Multi-recipient encryption
vvw encode input.wav -o output.wav --message "secret" \
    --encrypt-to alice.pub --encrypt-to bob.pub
vvw decode output.wav --key alice.priv   # Either recipient can decrypt
vvw decode output.wav --key bob.priv

# Signed message
vvw encode input.wav -o output.wav --message "verified" --sign --key my.priv
vvw decode output.wav --verify my.pub

# Metadata method (stores in RIFF chunk, not hidden but preserves audio)
vvw encode input.wav -o output.wav --message "data" --method metadata
vvw decode output.wav

# LSB options
vvw encode input.wav -o output.wav --message "data" --bits 2 --channels left
vvw decode output.wav
```

### play

Extract and play embedded audio.

```bash
# Play embedded audio
vvw play output.wav

# Extract to file instead
vvw play output.wav --extract-to recovered.wav

# With decryption
vvw play output.wav --passphrase "puzzle"
vvw play output.wav --key my.priv
```

### keygen

Generate a keypair for encryption and signing.

```bash
# Save to files
vvw keygen --output mykey
# Creates: mykey.pub and mykey.priv

# Output to stdout
vvw keygen
```

### inspect

Show embedded content metadata without decrypting.

```bash
vvw inspect output.wav

# Example output:
# VVW Embedded Data
# =================
#
# Method: LSB (Least Significant Bit)
# Content: text
# Payload size: 83 bytes (encrypted)
# Encryption: symmetric (passphrase)
# Signed: no
#
# Total embedded: 93 bytes
# Capacity used: 0.8%
# Available: 11018 bytes
```

## Steganography Methods

### LSB (Least Significant Bit)

Default method. Modifies the least significant bits of audio samples to embed data. With 1 bit per sample (default), the modification is inaudible (-96dB for 16-bit audio).

Options:
- `--bits 1-4` - Bits per sample (higher = more capacity, more audible)
- `--channels left|right|both` - Which channels to use

### Metadata

Stores data in a custom RIFF chunk (`vvwD`). Does not modify audio samples at all, but the chunk is visible to tools like `ffprobe`. Useful when audio fidelity is critical.

## Cryptography

- **Symmetric**: Argon2id key derivation + ChaCha20-Poly1305
- **Asymmetric**: X25519 key exchange + XChaCha20-Poly1305
- **Signatures**: Ed25519

Key files use a PEM-like format:
```
-----BEGIN VVW PUBLIC KEY-----
<base64 encoded key>
-----END VVW PUBLIC KEY-----
```

## Embedded Data Format

```
[4 bytes]  Magic: "VVW\x01"
[1 byte]   Flags (text, audio, signed, symmetric, asymmetric)
[1 byte]   Method (0=LSB, 1=metadata, 2=spread)
[4 bytes]  Payload length
[N bytes]  Payload (encrypted if applicable)
[64 bytes] Signature (if signed)
```

## Shell Completions

Generate shell completions for your shell:

```bash
# Bash
vvw completions bash > ~/.local/share/bash-completion/completions/vvw

# Zsh (add to fpath)
vvw completions zsh > ~/.zfunc/_vvw

# Fish
vvw completions fish > ~/.config/fish/completions/vvw.fish

# PowerShell
vvw completions powershell >> $PROFILE
```

For zsh, ensure `~/.zfunc` is in your fpath. Add to `~/.zshrc`:
```bash
fpath=(~/.zfunc $fpath)
autoload -Uz compinit && compinit
```

## License

MIT
