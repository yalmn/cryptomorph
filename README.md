<p align="center">
  <img src="cryptomorph/docs/logo.png" alt="Cryptomorph Logo" width="200" />
</p>

# Cryptomorph

**Cryptomorph** ist ein modulares, modernes CLI-Tool für starke, hybride Verschlüsselung, digitale Signaturen und künftige Erweiterungen (ECC, Post-Quantum, etc). Der Fokus liegt auf Übersichtlichkeit, Sicherheit und einfacher Erweiterbarkeit.

---

## Features

- RSA-Schlüsselpaar-Generierung (PEM, beliebige Bitlänge, z. B. 4096 Bit)
- Hybride Datei-Verschlüsselung (AES-256 + RSA)
- Datei-Verschlüsselung/Entschlüsselung mit AES-256
- Digitale Signaturen (SHA-256 + RSA) & Verifikation
- Schlüsselverwaltung im sicheren PEM-Format
- Moderne, klare CLI mit Hilfetexten
- Erweiterbar für weitere Verfahren (z. B. ECC, PQC)

---

## Schnellstart

### Installation (Beispiel für Windows)

```sh
cargo build --release
cd target/release
```

### Key-Generierung

```sh
./cryptomorph Rsa_Key_Gen 4096 out/
```

Legt `rsa_public.key` und `rsa_private.key` im PEM-Format im Verzeichnis `out/` ab.

### Hybride Datei-Verschlüsselung (AES+RSA)

**Verschlüsseln:**

```sh
./cryptomorph rsa_encrypt geheim.txt out/rsa_public.key geheim_encrypted.bin
```

**Entschlüsseln:**

```sh
./cryptomorph rsa_decrypt geheim_encrypted.bin out/rsa_private.key geheim_decrypted.txt
```

### Nur AES (ohne RSA)

**AES-256 Schlüssel generieren:**

```sh
./cryptomorph gen_aes_key
# Ausgabe: z.B. 34f5...abcd (64 Hex-Zeichen)
```

**Datei verschlüsseln:**

```sh
./cryptomorph aes_encrypt geheim.txt <aes-key-hex> geheim_aes.bin
```

**Datei entschlüsseln:**

```sh
./cryptomorph aes_decrypt geheim_aes.bin <aes-key-hex> geheim_decrypted.txt
```

### Digitale Signaturen

**Signieren:**

```sh
./cryptomorph rsa_sign geheim.txt out/rsa_private.key geheim.sig
```

**Verifizieren:**

```sh
./cryptomorph rsa_verify geheim.txt out/rsa_public.key geheim.sig
```

---

## Befehlsübersicht

| Befehl                  | Beschreibung                              |
| ----------------------- | ----------------------------------------- |
| Rsa_Key_Gen             | RSA-Schlüsselpaar generieren              |
| rsa_encrypt \<pub.key>  | Datei hybrid (AES+RSA) verschlüsseln      |
| rsa_decrypt \<priv.key> | Hybrid-verschlüsselte Datei entschlüsseln |
| aes_encrypt             | Datei direkt mit AES-256 verschlüsseln    |
| aes_decrypt             | AES-verschlüsselte Datei entschlüsseln    |
| rsa_sign \<priv.key>    | Datei signieren (RSA)                     |
| rsa_verify \<pub.key>   | Signatur verifizieren                     |
| gen_aes_key             | Zufälligen AES-256 Schlüssel erzeugen     |

Alle Schlüssel werden im sicheren PEM-Format erzeugt und verarbeitet.

---

## Beispiele & Szenarien

### 1. Datei sicher per E-Mail verschicken

- Schlüssel generieren: `Rsa_Key_Gen 4096 out/`
- Datei verschlüsseln: `rsa_encrypt nachricht.txt out/rsa_public.key nachricht_encrypted.bin`
- Datei entschlüsseln (Empfänger): `rsa_decrypt nachricht_encrypted.bin out/rsa_private.key nachricht.txt`

### 2. Dateien nur lokal verschlüsseln (ohne RSA)

- AES-Schlüssel generieren: `gen_aes_key`
- Datei verschlüsseln: `aes_encrypt daten.csv <aes-key-hex> daten_aes.bin`

### 3. Digitale Signaturen (Integritäts-Check, Absendernachweis)

- Datei signieren: `rsa_sign message.txt out/rsa_private.key message.sig`
- Signatur prüfen: `rsa_verify message.txt out/rsa_public.key message.sig`

---

## Funktionsweise der Queries / CLI

Jeder Befehl folgt dem Schema:

```
./cryptomorph <befehl> <parameter1> ... <parametern>
```

- Parameter (wie Dateinamen, Schlüsselpfade) sind **reihenfolgenabhängig**.
- **./cryptomorph --help** zeigt jederzeit eine Übersicht.

---

## Geplante Erweiterungen

- ECC-Verfahren (z. B. Curve25519)
- Post-Quantum Algorithmen
- Passwortbasierte Verschlüsselung
- Automatisierte Tests / Benchmarks
- CLI-Option für Batch-Processing
- ...

---

## Sicherheitshinweise

- Die Schlüsseldateien sollten niemals unverschlüsselt weitergegeben werden!
- Nur starke Passwörter/Schlüssel verwenden (vor allem bei AES!)
- `rsa_private.key` sicher aufbewahren!

---

## Lizenz

MIT oder Apache 2.0

---

## Mitwirken

PRs und neue Features sind sehr willkommen! Einfach Issue erstellen oder Fork + PR.
