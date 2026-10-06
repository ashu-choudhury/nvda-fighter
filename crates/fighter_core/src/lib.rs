//! ==============================================================================
//! ⚔️ NVDA FIGHTER CORE: THE SACRED AURA OF IMMORTALITY (In-Process Veteran Shield) ⚔️
//! ==============================================================================
//! 
//! "By the decree of the Grand Master, NVDA and the Windows Kernel share equal
//! sovereignty. No mortal terminal dump, no wicked memory leak, and no villainous
//! buffer overflow shall pierce this sacred citadel."
//! 
//! Class: ELITE ARCH-PALADIN (Level 100)
//! Guild: The Sacred Order of the Unkillable Screenreader
//! HP: 9999 / 9999
//! MP: 9999 / 9999
//! ==============================================================================

use std::ffi::{c_char, CStr, CString};
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Mutex;
use windows_sys::Win32::System::ProcessStatus::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;

/// Sacred RPG Stats of the Veteran Paladin
static FIGHTER_HP: AtomicI32 = AtomicI32::new(9999);
static FIGHTER_MAX_HP: i32 = 9999;
static TOTAL_VILLAINS_VANQUISHED: AtomicU64 = AtomicU64::new(0);
static TOTAL_BYTES_SHIELDED: AtomicU64 = AtomicU64::new(0);

/// Dragon-Breath threshold: > 10,000 characters dumped in console or speech
const DRAGON_BREATH_THRESHOLD: usize = 10_000;
const SACRED_TAIL_REMNANT: usize = 500;

/// Callback function pointer to log directly into NVDA Python console
type NvdaLogCallback = extern "C" fn(level: i32, msg: *const c_char);
static NVDA_LOGGER: Mutex<Option<NvdaLogCallback>> = Mutex::new(None);

/// Register the Python NVDA logging portal
#[no_mangle]
pub extern "C" fn fighter_set_nvda_logger(callback: NvdaLogCallback) {
    let mut logger = NVDA_LOGGER.lock().unwrap();
    *logger = Some(callback);
    drop(logger);
    fighter_log(
        1,
        "⚔️ [PALADIN-CORE] Telepathic link to NVDA Console established! Combat logs synchronised.",
    );
}

/// Helper to transmit battle messages through the NVDA logging portal
pub fn fighter_log(level: i32, message: &str) {
    if let Ok(guard) = NVDA_LOGGER.lock() {
        if let Some(cb) = *guard {
            if let Ok(c_msg) = CString::new(message) {
                cb(level, c_msg.as_ptr());
                return;
            }
        }
    }
    // Fallback if portal not hooked yet
    println!("[FIGHTER-CORE] {}", message);
}

/// ---------------------------------------------------------------------------
/// SPELL 1: EQUIP WIN32 DIVINE TITAN ARMOR
/// ---------------------------------------------------------------------------
/// Fortifies NVDA with Real-Time / High Priority, locks Working Set against
/// paging to disk swap, configures 1ms multimedia timer resolution,
/// and silences Windows modal crash dialogs.
#[no_mangle]
pub extern "C" fn fighter_equip_divine_armor() -> bool {
    unsafe {
        let current_process = GetCurrentProcess();

        // 1. Elevate Process Priority to REALTIME or HIGH
        // Try REALTIME_PRIORITY_CLASS first; if unprivileged, fallback to HIGH_PRIORITY_CLASS
        let mut prio_result = SetPriorityClass(current_process, REALTIME_PRIORITY_CLASS);
        let mut prio_str = "REALTIME_PRIORITY_CLASS (God Mode)";
        if prio_result == 0 {
            prio_result = SetPriorityClass(current_process, HIGH_PRIORITY_CLASS);
            prio_str = "HIGH_PRIORITY_CLASS (Equal with OS Kernel)";
        }

        // 2. Lock Priority against Windows dynamic de-boost
        let _ = SetProcessPriorityBoost(current_process, 0);

        // 3. Elevate Main Thread Priority to THREAD_PRIORITY_HIGHEST
        let current_thread = GetCurrentThread();
        SetThreadPriority(current_thread, THREAD_PRIORITY_HIGHEST);

        // 4. Request 1ms Multimedia Timer Resolution for silky smooth speech latency
        windows_sys::Win32::Media::timeBeginPeriod(1);

        // 5. Expand Working Set Minimum to 128 MB and Max to 512 MB to prevent disk swapping
        let min_ws: usize = 128 * 1024 * 1024;
        let max_ws: usize = 512 * 1024 * 1024;
        let _ = SetProcessWorkingSetSize(current_process, min_ws, max_ws);

        // 6. Banishes Windows Crash Traps: No modal error dialogs
        SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);

        fighter_log(
            1,
            &format!(
                "🛡️ [PALADIN-ARMOR] Titan Armor Equipped! Priority: {} | Thread: HIGHEST | Timer: 1ms | Crash Modals: BANISHED",
                prio_str
            ),
        );

        prio_result != 0
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 2: THE SACRED TEXT INTERCEPTOR (Dragon-Breath Terminal & Speech Shield)
/// ---------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn fighter_intercept_text_dump(input_ptr: *const c_char) -> *mut c_char {
    if input_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(input_ptr) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let len = text.chars().count();

    // If within peaceful limits, pass through
    if len <= DRAGON_BREATH_THRESHOLD {
        return std::ptr::null_mut();
    }

    // --- A VILLAIN HAS APPEARED! ---
    let villains = TOTAL_VILLAINS_VANQUISHED.fetch_add(1, Ordering::Relaxed) + 1;
    let bytes_shielded = TOTAL_BYTES_SHIELDED.fetch_add(text.len() as u64, Ordering::Relaxed) + text.len() as u64;

    // Take slight chip damage, regenerate if low
    let current_hp = FIGHTER_HP.fetch_sub(10, Ordering::Relaxed) - 10;
    let hp_display = if current_hp < 200 {
        FIGHTER_HP.store(FIGHTER_MAX_HP, Ordering::Relaxed);
        FIGHTER_MAX_HP
    } else {
        current_hp
    };

    // Extract tail remnant (last 500 characters)
    let tail_chars: String = text.chars().rev().take(SACRED_TAIL_REMNANT).collect();
    let tail: String = tail_chars.chars().rev().collect();

    let shielded_text = format!(
        "[⚔️ NVDA FIGHTER | Level 100 Arch-Paladin | HP: {}/{} | Villains Vanquished: {} | Total Shielded: {} bytes]\n\
         🛡️ DRAGON-BREATH BLAST NEUTRALISED: {} characters dumped! Speech buffer saved from catastrophic choke.\n\
         📜 Remnant Output (Last {} chars):\n{}",
        hp_display,
        FIGHTER_MAX_HP,
        villains,
        bytes_shielded,
        len,
        SACRED_TAIL_REMNANT,
        tail
    );

    fighter_log(
        2,
        &format!(
            "⚔️ [PALADIN-COMBAT] Neutralized dragon-breath dump of {} characters! Villains crushed so far: {}",
            len, villains
        ),
    );

    match CString::new(shielded_text) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees allocated text string from fighter_intercept_text_dump
#[no_mangle]
pub extern "C" fn fighter_free_text(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 3: ALCHEMICAL PURGE & LAG COMBAT
/// ---------------------------------------------------------------------------
/// Trims working set memory and empties stagnant pages back to Windows.
#[no_mangle]
pub extern "C" fn fighter_purge_memory_leak() -> u64 {
    unsafe {
        let current_process = GetCurrentProcess();
        let mut counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        if GetProcessMemoryInfo(
            current_process,
            &mut counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ) != 0
        {
            let before_working_set = counters.WorkingSetSize;
            EmptyWorkingSet(current_process);

            let mut after_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            after_counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
            GetProcessMemoryInfo(
                current_process,
                &mut after_counters,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );

            let freed_bytes = before_working_set.saturating_sub(after_counters.WorkingSetSize);
            let freed_mb = freed_bytes / (1024 * 1024);
            if freed_mb > 0 {
                fighter_log(
                    1,
                    &format!(
                        "🧪 [ALCHEMIST-PURGE] Banished {} MB of phantom memory back to the Windows ether!",
                        freed_mb
                    ),
                );
            }
            return freed_bytes as u64;
        }
        0
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 4: INSPECT RPG STATS
/// ---------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn fighter_get_stats(out_hp: *mut i32, out_kills: *mut u64, out_bytes: *mut u64) {
    unsafe {
        if !out_hp.is_null() {
            *out_hp = FIGHTER_HP.load(Ordering::Relaxed);
        }
        if !out_kills.is_null() {
            *out_kills = TOTAL_VILLAINS_VANQUISHED.load(Ordering::Relaxed);
        }
        if !out_bytes.is_null() {
            *out_bytes = TOTAL_BYTES_SHIELDED.load(Ordering::Relaxed);
        }
    }
}
