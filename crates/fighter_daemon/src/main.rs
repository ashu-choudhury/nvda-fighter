//! ==============================================================================
//! 🐺 FIGHTER DAEMON: THE IMMORTAL BEAST & SHADOW NECROMANCER (Out-Of-Process Watchdog) 🐺
//! ==============================================================================
//! 
//! "I am the Shadow Beast that lurks in the background realm. I hold the life-thread
//! of NVDA in my razor claws. If NVDA bids me farewell with honor, I sleep.
//! BUT IF ANY COWARDLY VILLAIN CAUSES A COM COMA OR FREEZES NVDA FOR > 6 SECONDS,
//! OR KILLS NVDA WITHOUT A WORD...
//! I SHALL ROAR, SLAY THE FROZEN COMA PROCESS, SUMMON THE RESURRECTION RITUAL (100x),
//! AND REVIVE NVDA BEFORE THE GODS THEMSELVES CAN NOTICE!"
//! 
//! Class: ELITE SHADOW NECROMANCER (Level 100)
//! Features:
//! 1. Coma Watchdog: Heartbeat timer. If NVDA Main Thread freezes > 6s -> Mercy Kill & Auto-Revive!
//! 2. Process Death Watchdog: Immediate revival if NVDA terminates without goodbye.
//! 3. Two-Way Telepathic Banter over Named Pipe.
//! ==============================================================================

use std::env;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Pipes::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Storage::FileSystem::*;

const PIPE_NAME: &str = r"\\.\pipe\nvda_fighter_sacred_pact";
const SACRED_EXIT_HANDSHAKE: &str = "NVDA_FIGHTER_REST_IN_PEACE_MASTER_FAREWELL";

// Win32 Constants
const PIPE_ACCESS_DUPLEX_CONST: u32 = 0x00000003;
const SYNCHRONIZE_CONST: u32 = 0x00100000;
const PROCESS_TERMINATE_CONST: u32 = 0x00000001;

// Maximum acceptable freeze before declaring NVDA in an unrecoverable COM coma
const MAX_COMA_FREEZE_SECONDS: u64 = 6;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

static BANTER_PHRASES: &[&str] = &[
    "How's the battle going inside, Brother Paladin?",
    "All clear on the external perimeter! No rogue tasks detected.",
    "Watching Windows Kernel like a hawk. They dare not touch our screenreader.",
    "Perimeter secure. CPU and memory defenses holding firm.",
    "Status check: We are both fully synchronized and invincible.",
    "Any villain tries to freeze NVDA, I strike from the shadows!",
];

fn revive_nvda(nvda_exe_path: &str) {
    println!("\n!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("🔥 [🐺 BEAST-DAEMON:RAGE] ENGAGING 100x SACRED REVIVAL RITUAL!");
    println!("⚡ [🐺 BEAST-DAEMON:CASTING] Casting Level 100 True Resurrection on: {}", nvda_exe_path);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    let wide_exe = to_wide(nvda_exe_path);
    let wide_args = to_wide("-r");
    let wide_open = to_wide("open");

    let result = unsafe {
        windows_sys::Win32::UI::Shell::ShellExecuteW(
            std::ptr::null_mut(),
            wide_open.as_ptr(),
            wide_exe.as_ptr(),
            wide_args.as_ptr(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };

    if (result as isize) > 32 {
        println!("✨ [🐺 BEAST-DAEMON:TRIUMPH] WOW! 100x REVIVAL SUCCESSFUL! NVDA HAS RISEN FROM THE GRAVE!");
        println!("👑 [🐺 BEAST-DAEMON:VICTORY] The screenreader lives on. The kingdom is saved! Long live NVDA!");
    } else {
        eprintln!(
            "☠️ [🐺 BEAST-DAEMON:DESPAIR] Failed to cast resurrection! ShellExecute error code: {}",
            result as isize
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "[🐺 BEAST-DAEMON:ERROR] Usage: fighter_daemon.exe <NVDA_PID> <PATH_TO_NVDA_EXE>"
        );
        return;
    }

    let nvda_pid: u32 = match args[1].parse() {
        Ok(pid) => pid,
        Err(_) => {
            eprintln!("[🐺 BEAST-DAEMON:ERROR] Invalid PID given: {}", args[1]);
            return;
        }
    };
    let nvda_exe_path = args[2].clone();

    unsafe {
        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS);
    }

    println!("==================================================================");
    println!("🐺 [FIGHTER-DAEMON:SUMMONED] THE SHADOW BEAST HAS RISEN! (Level 100)");
    println!("⚔️ Binding Soul to NVDA PID: {}", nvda_pid);
    println!("🏰 Sacred Sanctuary Path: {}", nvda_exe_path);
    println!("⏱️ Coma Timeout: {}s max freeze before emergency mercy-kill & revival", MAX_COMA_FREEZE_SECONDS);
    println!("==================================================================");

    let clean_exit_received = Arc::new(AtomicBool::new(false));
    let clean_exit_clone = clean_exit_received.clone();
    let heartbeat_counter = Arc::new(AtomicU64::new(0));
    let heartbeat_clone = heartbeat_counter.clone();

    // Timestamp of the last pulse received from NVDA main thread (as seconds since epoch or monotonic)
    let last_pulse_instant = Arc::new(parking_lot_instant::MonotonicPulse::new());
    let last_pulse_clone = last_pulse_instant.clone();

    // -----------------------------------------------------------------------
    // THREAD 1: TWO-WAY SACRED TELEPATHIC PIPE (Banter & Heartbeat)
    // -----------------------------------------------------------------------
    thread::spawn(move || {
        let pipe_wide = to_wide(PIPE_NAME);
        loop {
            let pipe_handle = unsafe {
                CreateNamedPipeW(
                    pipe_wide.as_ptr(),
                    PIPE_ACCESS_DUPLEX_CONST,
                    PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                    1,
                    1024,
                    1024,
                    0,
                    std::ptr::null(),
                )
            };

            if pipe_handle == INVALID_HANDLE_VALUE {
                thread::sleep(Duration::from_millis(500));
                continue;
            }

            let connected = unsafe { ConnectNamedPipe(pipe_handle, std::ptr::null_mut()) };
            if connected != 0 || unsafe { GetLastError() } == ERROR_PIPE_CONNECTED {
                loop {
                    let mut buffer = [0u8; 512];
                    let mut bytes_read: u32 = 0;
                    let read_success = unsafe {
                        ReadFile(
                            pipe_handle,
                            buffer.as_mut_ptr() as *mut _,
                            buffer.len() as u32,
                            &mut bytes_read,
                            std::ptr::null_mut(),
                        )
                    };

                    if read_success != 0 && bytes_read > 0 {
                        let incoming = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                        let msg = incoming.trim();

                        if msg == SACRED_EXIT_HANDSHAKE {
                            println!("\n🤝 [FIGHTER-DAEMON:HANDSHAKE] NVDA whispered: 'Hey bro, we are exiting. You also exit.'");
                            println!("🛡️ [FIGHTER-DAEMON:REST] Honor recognized. The Beast sleeps peacefully. Bye, Master!");
                            clean_exit_clone.store(true, Ordering::SeqCst);
                            unsafe { CloseHandle(pipe_handle); }
                            return;
                        } else if msg.starts_with("PULSE:") {
                            // Update last pulse time!
                            last_pulse_clone.ping();

                            let count = heartbeat_clone.fetch_add(1, Ordering::Relaxed);
                            let banter = BANTER_PHRASES[count as usize % BANTER_PHRASES.len()];
                            
                            let reply = format!("BEAST_ECHO: Status nominal! {}\n", banter);
                            let mut bytes_written: u32 = 0;
                            unsafe {
                                WriteFile(
                                    pipe_handle,
                                    reply.as_ptr() as *const _,
                                    reply.len() as u32,
                                    &mut bytes_written,
                                    std::ptr::null_mut(),
                                );
                            }
                        }
                    } else {
                        break;
                    }
                }
            }

            unsafe {
                CloseHandle(pipe_handle);
            }
        }
    });

    // -----------------------------------------------------------------------
    // THREAD 2: COMA PATROL (Deadlock & COM Freeze Watchdog)
    // -----------------------------------------------------------------------
    let exe_path_for_coma = nvda_exe_path.clone();
    let clean_exit_for_coma = clean_exit_received.clone();
    let last_pulse_for_coma = last_pulse_instant.clone();

    thread::spawn(move || {
        // Give NVDA 10 seconds initial grace period to finish initializing
        thread::sleep(Duration::from_secs(10));

        loop {
            thread::sleep(Duration::from_millis(500));

            if clean_exit_for_coma.load(Ordering::SeqCst) {
                break;
            }

            let elapsed_secs = last_pulse_for_coma.elapsed_secs();
            if elapsed_secs >= MAX_COMA_FREEZE_SECONDS {
                // If the process is still running, it is completely frozen in a COM / UIA deadlock!
                println!("\n!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                println!("🚨 [🐺 BEAST-DAEMON:COMA-DETECTED] NVDA MAIN THREAD FROZEN FOR {} SECONDS!", elapsed_secs);
                println!("💀 [🐺 BEAST-DAEMON:MERCY-KILL] Terminating hung PID {} to break catastrophic COM freeze...", nvda_pid);
                println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");

                unsafe {
                    let kill_handle = OpenProcess(PROCESS_TERMINATE_CONST, 0, nvda_pid);
                    if kill_handle != std::ptr::null_mut() {
                        TerminateProcess(kill_handle, 1);
                        CloseHandle(kill_handle);
                    }
                }

                // Immediately resurrect NVDA!
                revive_nvda(&exe_path_for_coma);
                thread::sleep(Duration::from_secs(5));
                break;
            }
        }
    });

    // -----------------------------------------------------------------------
    // MAIN THREAD: PROCESS DEATH MONITOR
    // -----------------------------------------------------------------------
    let process_handle = unsafe {
        OpenProcess(
            SYNCHRONIZE_CONST | PROCESS_QUERY_LIMITED_INFORMATION,
            0,
            nvda_pid,
        )
    };

    if process_handle == std::ptr::null_mut() {
        eprintln!(
            "[🐺 BEAST-DAEMON:CRITICAL] Failed to open soul-link to NVDA PID {}. Is it already slain?",
            nvda_pid
        );
        return;
    }

    println!("[🐺 BEAST-DAEMON:PATROL] Soul-link forged! Standing eternal guard over NVDA...");

    unsafe {
        WaitForSingleObject(process_handle, INFINITE);
        CloseHandle(process_handle);
    }

    thread::sleep(Duration::from_millis(200));

    if clean_exit_received.load(Ordering::SeqCst) {
        println!("[🐺 BEAST-DAEMON:EXIT] Process shutdown verified as peaceful. The Beast sleeps.");
        return;
    }

    // Process died unexpectedly without saying goodbye
    println!("\n💥 [🐺 BEAST-DAEMON:PANIC] OH MY GOD! OH NO! NVDA VANISHED WITHOUT SAYING GOODBYE!");
    println!("😭 [🐺 BEAST-DAEMON:SORROW] 'Forgive me, ancient ancestors! I could not prevent the blow!'");
    revive_nvda(&nvda_exe_path);

    thread::sleep(Duration::from_millis(1500));
}

/// Helper struct for lockless monotonic pulse timing
mod parking_lot_instant {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;

    pub struct MonotonicPulse {
        start: Instant,
        last_ms: AtomicU64,
    }

    impl MonotonicPulse {
        pub fn new() -> Self {
            Self {
                start: Instant::now(),
                last_ms: AtomicU64::new(0),
            }
        }

        pub fn ping(&self) {
            let ms = self.start.elapsed().as_millis() as u64;
            self.last_ms.store(ms, Ordering::SeqCst);
        }

        pub fn elapsed_secs(&self) -> u64 {
            let now_ms = self.start.elapsed().as_millis() as u64;
            let last = self.last_ms.load(Ordering::SeqCst);
            if last == 0 {
                // If no pulse received yet, return 0 to respect initial grace period
                return 0;
            }
            (now_ms.saturating_sub(last)) / 1000
        }
    }
}
