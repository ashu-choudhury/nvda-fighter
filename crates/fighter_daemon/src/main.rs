//! ==============================================================================
//! 🐺 FIGHTER DAEMON: THE IMMORTAL BEAST & SHADOW NECROMANCER (Out-Of-Process Watchdog) 🐺
//! ==============================================================================
//! 
//! "I am the Shadow Beast that lurks in the background realm. I hold the life-thread
//! of NVDA in my razor claws. If NVDA bids me farewell with honor, I sleep.
//! BUT IF ANY COWARDLY VILLAIN CAUSES A COM COMA OR FREEZES NVDA FOR > 12 SECONDS,
//! OR KILLS NVDA WITHOUT A WORD...
//! I SHALL ROAR, SLAY THE FROZEN COMA PROCESS, SUMMON THE RESURRECTION RITUAL (100x),
//! AND REVIVE NVDA BEFORE THE GODS THEMSELVES CAN NOTICE!"
//! 
//! Class: ELITE SHADOW NECROMANCER (Level 100)
//! Features:
//! 1. Single Instance Protection: Win32 Named Mutex `Global\NVDA_FIGHTER_BEAST_MUTEX`.
//! 2. Persistent Battle Logging: Appends to `fighter_battle.log` so crash history is never wiped!
//! 3. Coma Watchdog: 12-second threshold with 15-second startup grace period.
//! 4. Process Death Watchdog: Immediate revival if NVDA terminates without goodbye.
//! ==============================================================================

use std::env;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Pipes::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Storage::FileSystem::*;

const PIPE_NAME: &str = r"\\.\pipe\nvda_fighter_sacred_pact";
const SACRED_EXIT_HANDSHAKE: &str = "NVDA_FIGHTER_REST_IN_PEACE_MASTER_FAREWELL";
const MUTEX_NAME: &str = r"Global\NVDA_FIGHTER_BEAST_MUTEX";

// Win32 Constants
const PIPE_ACCESS_DUPLEX_CONST: u32 = 0x00000003;
const SYNCHRONIZE_CONST: u32 = 0x00100000;
const PROCESS_TERMINATE_CONST: u32 = 0x00000001;

// Generous 12-second threshold so heavy tasks don't cause false restarts
const MAX_COMA_FREEZE_SECONDS: u64 = 12;

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

/// Persistent Battle Logger: writes to log file and console
fn battle_log(log_path: &str, message: &str) {
    let now = chrono_fallback_timestamp();
    let line = format!("[{}] {}\n", now, message);
    print!("{}", line);

    if !log_path.is_empty() {
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = f.write_all(line.as_bytes());
        }
    }
}

fn chrono_fallback_timestamp() -> String {
    let mut st: SYSTEMTIME = unsafe { std::mem::zeroed() };
    unsafe { windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut st); }
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond, st.wMilliseconds
    )
}

fn revive_nvda(log_path: &str, nvda_exe_path: &str) {
    battle_log(log_path, "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    battle_log(log_path, "🔥 [🐺 BEAST-DAEMON:RAGE] ENGAGING 100x SACRED REVIVAL RITUAL!");
    battle_log(log_path, &format!("⚡ [🐺 BEAST-DAEMON:CASTING] Casting Level 100 True Resurrection on: {}", nvda_exe_path));
    battle_log(log_path, "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

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
        battle_log(log_path, "✨ [🐺 BEAST-DAEMON:TRIUMPH] WOW! 100x REVIVAL SUCCESSFUL! NVDA HAS RISEN FROM THE GRAVE!");
    } else {
        battle_log(log_path, &format!("☠️ [🐺 BEAST-DAEMON:DESPAIR] Failed to cast resurrection! ShellExecute error code: {}", result as isize));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("[🐺 BEAST-DAEMON:ERROR] Usage: fighter_daemon.exe <NVDA_PID> <PATH_TO_NVDA_EXE> [LOG_FILE_PATH]");
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
    let log_file_path = if args.len() >= 4 { args[3].clone() } else { "fighter_battle.log".to_string() };

    // -----------------------------------------------------------------------
    // SINGLE INSTANCE PROTECTION (Win32 Named Mutex)
    // -----------------------------------------------------------------------
    let mutex_wide = to_wide(MUTEX_NAME);
    let mutex_handle = unsafe {
        CreateMutexW(std::ptr::null(), 1, mutex_wide.as_ptr())
    };

    if mutex_handle == std::ptr::null_mut() || unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        battle_log(&log_file_path, "[🐺 BEAST-DAEMON:WARN] Another Shadow Beast daemon is already vigilant! Exiting redundant beast.");
        return;
    }

    unsafe {
        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS);
    }

    battle_log(&log_file_path, "==================================================================");
    battle_log(&log_file_path, "🐺 [FIGHTER-DAEMON:SUMMONED] THE SHADOW BEAST HAS RISEN! (Level 100)");
    battle_log(&log_file_path, &format!("⚔️ Binding Soul to NVDA PID: {}", nvda_pid));
    battle_log(&log_file_path, &format!("🏰 Sacred Sanctuary Path: {}", nvda_exe_path));
    battle_log(&log_file_path, &format!("📜 Persistent Battle Log: {}", log_file_path));
    battle_log(&log_file_path, &format!("⏱️ Coma Timeout: {}s max freeze before emergency mercy-kill & revival", MAX_COMA_FREEZE_SECONDS));
    battle_log(&log_file_path, "==================================================================");

    let clean_exit_received = Arc::new(AtomicBool::new(false));
    let clean_exit_clone = clean_exit_received.clone();
    let heartbeat_counter = Arc::new(AtomicU64::new(0));
    let heartbeat_clone = heartbeat_counter.clone();

    let last_pulse_instant = Arc::new(parking_lot_instant::MonotonicPulse::new());
    let last_pulse_clone = last_pulse_instant.clone();

    let log_for_pipe = log_file_path.clone();

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
                            battle_log(&log_for_pipe, "🤝 [FIGHTER-DAEMON:HANDSHAKE] NVDA whispered: 'Hey bro, we are exiting. You also exit.'");
                            battle_log(&log_for_pipe, "🛡️ [FIGHTER-DAEMON:REST] Honor recognized. The Beast sleeps peacefully. Bye, Master!");
                            clean_exit_clone.store(true, Ordering::SeqCst);
                            unsafe { CloseHandle(pipe_handle); }
                            return;
                        } else if msg.starts_with("PULSE:") {
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
    let log_for_coma = log_file_path.clone();

    thread::spawn(move || {
        // 15-second initial grace period to allow NVDA startup
        thread::sleep(Duration::from_secs(15));

        loop {
            thread::sleep(Duration::from_millis(500));

            if clean_exit_for_coma.load(Ordering::SeqCst) {
                break;
            }

            let elapsed_secs = last_pulse_for_coma.elapsed_secs();
            if elapsed_secs >= MAX_COMA_FREEZE_SECONDS {
                battle_log(&log_for_coma, "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                battle_log(&log_for_coma, &format!("🚨 [BEAST-DAEMON:COMA-DETECTED] NVDA MAIN THREAD FROZEN FOR {} SECONDS!", elapsed_secs));
                battle_log(&log_for_coma, &format!("💀 [BEAST-DAEMON:MERCY-KILL] Terminating hung PID {} to break catastrophic COM freeze...", nvda_pid));
                battle_log(&log_for_coma, "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

                unsafe {
                    let kill_handle = OpenProcess(PROCESS_TERMINATE_CONST, 0, nvda_pid);
                    if kill_handle != std::ptr::null_mut() {
                        TerminateProcess(kill_handle, 1);
                        CloseHandle(kill_handle);
                    }
                }

                revive_nvda(&log_for_coma, &exe_path_for_coma);
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
        battle_log(&log_file_path, &format!("[🐺 BEAST-DAEMON:CRITICAL] Failed to open soul-link to NVDA PID {}. Is it already slain?", nvda_pid));
        return;
    }

    battle_log(&log_file_path, "[🐺 BEAST-DAEMON:PATROL] Soul-link forged! Standing eternal guard over NVDA...");

    unsafe {
        WaitForSingleObject(process_handle, INFINITE);
        CloseHandle(process_handle);
    }

    thread::sleep(Duration::from_millis(200));

    if clean_exit_received.load(Ordering::SeqCst) {
        battle_log(&log_file_path, "[🐺 BEAST-DAEMON:EXIT] Process shutdown verified as peaceful. The Beast sleeps.");
        unsafe { CloseHandle(mutex_handle); }
        return;
    }

    battle_log(&log_file_path, "💥 [🐺 BEAST-DAEMON:PANIC] OH MY GOD! OH NO! NVDA VANISHED WITHOUT SAYING GOODBYE!");
    battle_log(&log_file_path, "😭 [🐺 BEAST-DAEMON:SORROW] 'Forgive me, ancient ancestors! I could not prevent the blow!'");
    revive_nvda(&log_file_path, &nvda_exe_path);

    unsafe { CloseHandle(mutex_handle); }
    thread::sleep(Duration::from_millis(1500));
}

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
                return 0;
            }
            (now_ms.saturating_sub(last)) / 1000
        }
    }
}
