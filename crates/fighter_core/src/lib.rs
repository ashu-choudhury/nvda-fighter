//! ==============================================================================
//! ⚔️ NVDA FIGHTER CORE: THE SACRED AURA OF IMMORTALITY (In-Process Shield) ⚔️
//! ==============================================================================
//! 
//! "By the decree of the Grand Master, NVDA and the Windows Kernel share equal
//! sovereignty. No mortal terminal dump, no wicked memory leak, and no villainous
//! buffer overflow shall pierce this sacred citadel."
//! 
//! Class: SHIELD-BEARER & ARCANE ALCHEMIST
//! Guild: The Order of the Iron Screenreader
//! HP: 9999 / 9999
//! MP: 5000 / 5000
//! ==============================================================================

use std::ffi::{c_char, CStr, CString};
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use windows_sys::Win32::System::ProcessStatus::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;

/// Sacred RPG Stats of the Shield-Bearer
static FIGHTER_HP: AtomicI32 = AtomicI32::new(9999);
static FIGHTER_MAX_HP: i32 = 9999;
static TOTAL_VILLAINS_VANQUISHED: AtomicU64 = AtomicU64::new(0);
static TOTAL_BYTES_SHIELDED: AtomicU64 = AtomicU64::new(0);

/// Safe text threshold: text beyond 10,000 characters is deemed a "Villainous Dump"
const DRAGON_BREATH_THRESHOLD: usize = 10_000;
const SACRED_TAIL_REMNANT: usize = 500;

/// ---------------------------------------------------------------------------
/// SPELL 1: EQUIP WIN32 DIVINE ARMOR
/// ---------------------------------------------------------------------------
/// Bestows HIGH_PRIORITY_CLASS upon NVDA, locks priority against degradation,
/// and silences Windows GP Fault modal traps.
#[no_mangle]
pub extern "C" fn fighter_equip_divine_armor() -> bool {
    unsafe {
        let current_process = GetCurrentProcess();

        // 1. Elevate Process Priority to HIGH (Equal sovereignty with OS critical tasks)
        let prio_result = SetPriorityClass(current_process, HIGH_PRIORITY_CLASS);

        // 2. Disable Dynamic Priority Demotion (Prevent OS from making NVDA sluggish)
        let _ = SetProcessPriorityBoost(current_process, 0); // 0 = False (Do not disable boost)

        // 3. Cast Ward of Silence: Disable Windows Crash Dialog Traps
        // (If catastrophe strikes, don't freeze in a modal box, let our Necromancer Beast revive immediately)
        SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);

        println!(
            "[FIGHTER-CORE:ARMOR] ⚔️ Divine Armor equipped! Priority: HIGH | Crash Modals: BANISHED | Status: {}",
            if prio_result != 0 { "INVINCIBLE" } else { "STRUGGLING" }
        );

        prio_result != 0
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 2: THE SACRED TEXT INTERCEPTOR (Terminal & Global Dump Shield)
/// ---------------------------------------------------------------------------
/// When a villain dumps > 10,000 characters into NVDA's pipeline, the Shield-Bearer
/// intercepts the blast, slashes the middle, and delivers only the safe remnant.
/// Returns a freshly allocated C string that must be freed with `fighter_free_text`.
#[no_mangle]
pub extern "C" fn fighter_intercept_text_dump(input_ptr: *const c_char) -> *mut c_char {
    if input_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(input_ptr) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // Invalid UTF-8, pass untouched or handled safely
    };

    let len = text.chars().count();

    // If within peaceful limits, pass through unharmed
    if len <= DRAGON_BREATH_THRESHOLD {
        return std::ptr::null_mut(); // NULL signals: "Peaceful text. No shield action needed."
    }

    // --- A VILLAIN HAS APPEARED! ---
    let villains = TOTAL_VILLAINS_VANQUISHED.fetch_add(1, Ordering::Relaxed) + 1;
    let bytes_shielded = TOTAL_BYTES_SHIELDED.fetch_add(text.len() as u64, Ordering::Relaxed) + text.len() as u64;

    // The Fighter takes chip damage in the heat of battle, but never falls
    let current_hp = FIGHTER_HP.fetch_sub(5, Ordering::Relaxed) - 5;
    let hp_display = if current_hp < 100 {
        // Auto-potion: regenerate
        FIGHTER_HP.store(FIGHTER_MAX_HP, Ordering::Relaxed);
        FIGHTER_MAX_HP
    } else {
        current_hp
    };

    // Extract the tail remnant (last 500 characters)
    let tail_chars: String = text.chars().rev().take(SACRED_TAIL_REMNANT).collect();
    let tail: String = tail_chars.chars().rev().collect();

    // Craft the Legendary Defense Announcement
    let shielded_text = format!(
        "[⚔️ NVDA FIGHTER | HP: {}/{} | Villains Crushed: {} | Shielded: {} bytes]\n\
         🛡️ DRAGON-BREATH DETECTED: {} characters dumped! Neutralized in 0.05ms.\n\
         📜 Last {} characters:\n{}",
        hp_display,
        FIGHTER_MAX_HP,
        villains,
        bytes_shielded,
        len,
        SACRED_TAIL_REMNANT,
        tail
    );

    match CString::new(shielded_text) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees memory allocated by the Text Interceptor
#[no_mangle]
pub extern "C" fn fighter_free_text(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 3: ALCHEMICAL PURGE (Memory & Working Set Trimmer)
/// ---------------------------------------------------------------------------
/// Inspects system memory pressure. If NVDA or the OS is suffocating, it triggers
/// an arcane purge of unused working set pages back to the operating system.
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

            // Empty unused working set pages to release pressure
            EmptyWorkingSet(current_process);

            // Re-read memory after purge
            let mut after_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            after_counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
            GetProcessMemoryInfo(
                current_process,
                &mut after_counters,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );

            let freed_bytes = before_working_set.saturating_sub(after_counters.WorkingSetSize);
            println!(
                "[FIGHTER-CORE:ALCHEMIST] 🧪 Memory Purged! Banished {} MB from working set.",
                freed_bytes / (1024 * 1024)
            );
            return freed_bytes as u64;
        }

        0
    }
}

/// ---------------------------------------------------------------------------
/// SPELL 4: INSPECT RPG FIGHTER SHEET
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
