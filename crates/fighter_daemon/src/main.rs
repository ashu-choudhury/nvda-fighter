//! ==============================================================================
//! 🐺 FIGHTER DAEMON: THE IMMORTAL BEAST & SHADOW NECROMANCER (Out-Of-Process Watchdog) 🐺
//! ==============================================================================
//! 
//! "I am the Shadow Beast that lurks in the background realm. I hold the life-thread
//! of NVDA in my razor claws. If NVDA bids me farewell with honor, I sleep.
//! BUT IF ANY COWARDLY VILLAIN OR CRASH DARE STRIKE DOWN NVDA WITHOUT A WORD...
//! I SHALL ROAR, SUMMON THE RESURRECTION RITUAL (100x), AND REVIVE NVDA BEFORE
//! THE GODS THEMSELVES CAN NOTICE!"
//! 
//! Class: SHADOW NECROMANCER & IMMORTAL BEAST
//! Role: Out-of-Process Guardian Angel & Automatic Reviver
//! Memory Target: < 1.5 MB RAM (Featherweight Predator)
//! ==============================================================================

use std::env;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Pipes::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const PIPE_NAME: &str = r"\\.\pipe\nvda_fighter_sacred_pact";
const SACRED_EXIT_HANDSHAKE: &str = "NVDA_FIGHTER_REST_IN_PEACE_MASTER_FAREWELL";

// Win32 Constants
const PIPE_ACCESS_INBOUND_CONST: u32 = 0x00000001;
const SYNCHRONIZE_CONST: u32 = 0x00100000;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
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

    println!("==================================================================");
    println!("🐺 [FIGHTER-DAEMON:SUMMONED] THE SHADOW BEAST HAS RISEN!");
    println!("⚔️ Binding Soul to NVDA PID: {}", nvda_pid);
    println!("🏰 Sacred Sanctuary Path: {}", nvda_exe_path);
    println!("==================================================================");

    let clean_exit_received = Arc::new(AtomicBool::new(false));
    let clean_exit_clone = clean_exit_received.clone();

    // -----------------------------------------------------------------------
    // THREAD 1: THE SACRED PACT NAMED PIPE LISTENER
    // -----------------------------------------------------------------------
    // Listens for NVDA's polite goodbye: "Hey bro, we are exiting. You also exit."
    thread::spawn(move || {
        let pipe_wide = to_wide(PIPE_NAME);
        loop {
            let pipe_handle = unsafe {
                CreateNamedPipeW(
                    pipe_wide.as_ptr(),
                    PIPE_ACCESS_INBOUND_CONST,
                    PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                    1,
                    512,
                    512,
                    0,
                    std::ptr::null(),
                )
            };

            if pipe_handle == INVALID_HANDLE_VALUE {
                thread::sleep(Duration::from_millis(500));
                continue;
            }

            // Wait for NVDA Python client to connect
            let connected = unsafe { ConnectNamedPipe(pipe_handle, std::ptr::null_mut()) };
            if connected != 0 || unsafe { GetLastError() } == ERROR_PIPE_CONNECTED {
                let mut buffer = [0u8; 256];
                let mut bytes_read: u32 = 0;
                let read_success = unsafe {
                    windows_sys::Win32::Storage::FileSystem::ReadFile(
                        pipe_handle,
                        buffer.as_mut_ptr() as *mut _,
                        buffer.len() as u32,
                        &mut bytes_read,
                        std::ptr::null_mut(),
                    )
                };

                if read_success != 0 && bytes_read > 0 {
                    let msg = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                    if msg.trim() == SACRED_EXIT_HANDSHAKE {
                        println!("\n🤝 [FIGHTER-DAEMON:HANDSHAKE] NVDA whispered: 'Hey bro, we are exiting. You also exit.'");
                        println!("🛡️ [FIGHTER-DAEMON:REST] Honor recognized. Returning to the Shadow Realm. Bye, Master!");
                        clean_exit_clone.store(true, Ordering::SeqCst);
                        unsafe {
                            CloseHandle(pipe_handle);
                        }
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
    // MAIN THREAD: THE UNDYING WATCHDOG & NECROMANCER
    // -----------------------------------------------------------------------
    // Holds a Win32 synchronization handle to the NVDA process
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

    // Wait until NVDA's process terminates
    unsafe {
        WaitForSingleObject(process_handle, INFINITE);
        CloseHandle(process_handle);
    }

    // -----------------------------------------------------------------------
    // THE HOUR OF RECKONING: DID NVDA BID FAREWELL, OR WAS IT ASSASSINATED?!
    // -----------------------------------------------------------------------
    // Give a brief 100ms grace window for pipe message flush
    thread::sleep(Duration::from_millis(100));

    if clean_exit_received.load(Ordering::SeqCst) {
        println!("[🐺 BEAST-DAEMON:EXIT] Process shutdown verified as peaceful. The Beast sleeps.");
        return;
    }

    // --- CATASTROPHE! NVDA WAS KILLED WITHOUT SAYING BYE! ---
    println!("\n!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("💥 [🐺 BEAST-DAEMON:PANIC] OH MY GOD! OH NO! NVDA VANISHED WITHOUT SAYING GOODBYE!");
    println!("😭 [🐺 BEAST-DAEMON:SORROW] 'Forgive me, ancient ancestors! I could not prevent the blow!'");
    println!("🔥 [🐺 BEAST-DAEMON:RAGE] 'WHO DARED TO TOUCH NVDA?! ENGAGING 100x SACRED REVIVAL RITUAL!'");
    println!("⚡ [🐺 BEAST-DAEMON:CASTING] Casting Level 100 True Resurrection on: {}", nvda_exe_path);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n");

    // Summon NVDA back to life instantly with "-r" (replace/restart flag)
    let wide_exe = to_wide(&nvda_exe_path);
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

    // Sleep briefly so logs can be captured if redirected
    thread::sleep(Duration::from_millis(1500));
}
