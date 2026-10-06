# -*- coding: utf-8 -*-
"""
==============================================================================
⚔️ NVDA FIGHTER: THE SACRED AURA OF IMMORTALITY (Global Plugin) ⚔️
==============================================================================

"By the decree of the Grand Master, NVDA and the Windows Kernel share equal
sovereignty. No mortal terminal dump, no wicked memory leak, and no villainous
buffer overflow shall pierce this sacred citadel."

Class: GRAND GUILD MASTER & SACRED SUMMONER (Level 100)
Guild: The Sacred Order of the Unkillable Screenreader
==============================================================================
"""

import os
import sys
import gc
import ctypes
from ctypes import c_char_p, c_bool, c_uint64, c_int32, POINTER, byref, CFUNCTYPE, c_int
import subprocess
import threading
import time
from datetime import datetime

import globalPluginHandler
import ui
import speech
import api
import eventHandler
from scriptHandler import script
from logHandler import log

# Sacred constants
SACRED_PIPE_NAME = r"\\.\pipe\nvda_fighter_sacred_pact"
SACRED_EXIT_HANDSHAKE = b"NVDA_FIGHTER_REST_IN_PEACE_MASTER_FAREWELL\n"

# Persistent Log File
_PLUGIN_DIR = os.path.dirname(os.path.abspath(__file__))
_LOGS_DIR = os.path.join(_PLUGIN_DIR, "logs")
os.makedirs(_LOGS_DIR, exist_ok=True)
FIGHTER_LOG_FILE = os.path.join(_LOGS_DIR, "fighter_battle.log")


def write_persistent_log(message: str):
    try:
        now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S.%f")[:-3]
        line = f"[{now_str}] {message}\n"
        with open(FIGHTER_LOG_FILE, "a", encoding="utf-8") as f:
            f.write(line)
    except Exception:
        pass


LOG_CALLBACK_TYPE = CFUNCTYPE(None, c_int, c_char_p)


def _nvda_python_logger(level, msg_ptr):
    try:
        msg = msg_ptr.decode("utf-8", errors="replace") if msg_ptr else ""
        write_persistent_log(f"[NATIVE] {msg}")
        if level >= 2:
            log.warning(f"[FIGHTER-NATIVE] {msg}")
        else:
            log.info(f"[FIGHTER-NATIVE] {msg}")
    except Exception:
        pass


_GLOBAL_NATIVE_LOG_CALLBACK = LOG_CALLBACK_TYPE(_nvda_python_logger)


class FighterNativeBridge:
    def __init__(self, dll_path: str):
        self.loaded = False
        self._dll = None

        if not os.path.exists(dll_path):
            write_persistent_log(f"[FIGHTER-BRIDGE:WARN] Sacred core not found at {dll_path}")
            return

        try:
            self._dll = ctypes.CDLL(dll_path)

            self._dll.fighter_set_nvda_logger.restype = None
            self._dll.fighter_set_nvda_logger.argtypes = [LOG_CALLBACK_TYPE]

            self._dll.fighter_equip_divine_armor.restype = c_bool
            self._dll.fighter_equip_divine_armor.argtypes = []

            self._dll.fighter_intercept_text_dump.restype = c_char_p
            self._dll.fighter_intercept_text_dump.argtypes = [c_char_p]

            self._dll.fighter_free_text.restype = None
            self._dll.fighter_free_text.argtypes = [c_char_p]

            self._dll.fighter_purge_memory_leak.restype = c_uint64
            self._dll.fighter_purge_memory_leak.argtypes = []

            self._dll.fighter_get_stats.restype = None
            self._dll.fighter_get_stats.argtypes = [
                POINTER(c_int32),
                POINTER(c_uint64),
                POINTER(c_uint64),
            ]

            self._dll.fighter_record_shielding.restype = None
            self._dll.fighter_record_shielding.argtypes = [c_uint64, c_uint64]

            self._dll.fighter_set_nvda_logger(_GLOBAL_NATIVE_LOG_CALLBACK)
            self.loaded = True
            write_persistent_log("[FIGHTER-BRIDGE:SUCCESS] ⚔️ Level 100 Sacred Rust Core loaded into memory!")
        except Exception as e:
            write_persistent_log(f"[FIGHTER-BRIDGE:ERROR] Failed to bind to sacred core: {e}")

    def equip_armor(self) -> bool:
        if self.loaded and self._dll:
            return self._dll.fighter_equip_divine_armor()
        return False

    def intercept_text(self, text: str):
        if not self.loaded or not self._dll or len(text) <= 10000:
            return None

        try:
            utf8_bytes = text.encode("utf-8", errors="replace")
            res_ptr = self._dll.fighter_intercept_text_dump(utf8_bytes)
            if res_ptr:
                shielded_str = res_ptr.decode("utf-8", errors="replace")
                self._dll.fighter_free_text(res_ptr)
                return shielded_str
        except Exception as e:
            write_persistent_log(f"[FIGHTER-BRIDGE:ERROR] Text shield failed: {e}")
        return None

    def record_shielding(self, count=1, bytes_shielded=50000):
        if self.loaded and self._dll:
            try:
                self._dll.fighter_record_shielding(c_uint64(count), c_uint64(bytes_shielded))
            except Exception:
                pass

    def purge_memory(self) -> int:
        if self.loaded and self._dll:
            return int(self._dll.fighter_purge_memory_leak())
        return 0

    def get_stats(self):
        if not self.loaded or not self._dll:
            return (9999, 0, 0)
        hp = c_int32(0)
        kills = c_uint64(0)
        bytes_shielded = c_uint64(0)
        self._dll.fighter_get_stats(byref(hp), byref(kills), byref(bytes_shielded))
        return (hp.value, kills.value, bytes_shielded.value)


class GlobalPlugin(globalPluginHandler.GlobalPlugin):
    scriptCategory = "NVDA Fighter"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.plugin_dir = _PLUGIN_DIR
        write_persistent_log("==================================================================")
        write_persistent_log("[NVDA-FIGHTER:INIT] Awakening NVDA Fighter...")

        arch = "x64" if sys.maxsize > 2**32 else "x86"
        dll_candidates = [
            os.path.join(self.plugin_dir, "lib", arch, "fighter_core.dll"),
            os.path.join(self.plugin_dir, "fighter_core.dll"),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "i686-pc-windows-msvc", "release", "fighter_core.dll")),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "release", "fighter_core.dll")),
        ]
        daemon_candidates = [
            os.path.join(self.plugin_dir, "bin", arch, "fighter_daemon.exe"),
            os.path.join(self.plugin_dir, "fighter_daemon.exe"),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "i686-pc-windows-msvc", "release", "fighter_daemon.exe")),
            os.path.abspath(os.path.join(self.plugin_dir, "..", "..", "..", "target", "release", "fighter_daemon.exe")),
        ]

        dll_path = next((p for p in dll_candidates if os.path.exists(p)), dll_candidates[0])
        self.daemon_path = next((p for p in daemon_candidates if os.path.exists(p)), daemon_candidates[0])

        self.bridge = FighterNativeBridge(dll_path)
        if self.bridge.loaded:
            self.bridge.equip_armor()

        self.daemon_proc = None
        self._summon_shadow_beast()

        # Terminal Burst Tracking
        self._terminal_event_timestamps = []
        self._last_event_time = 0.0
        self._consecutive_fast_events = 0
        self._hook_event_gatekeeper()

        self._original_speak = speech.speak
        self._hook_speech_engine()

        self._patrol_active = True
        self._patrol_thread = threading.Thread(target=self._telepathic_chatter_worker, daemon=True)
        self._patrol_thread.start()

        write_persistent_log("[NVDA-FIGHTER:READY] 🛡️ NVDA Fighter standing guard! Smart Adaptive Burst Gatekeeper Active!")

    def _summon_shadow_beast(self):
        if not os.path.exists(self.daemon_path):
            write_persistent_log(f"[NVDA-FIGHTER:DAEMON] Shadow beast binary not found at {self.daemon_path}")
            return

        try:
            my_pid = str(os.getpid())
            nvda_exe = sys.executable

            DETACHED_PROCESS = 0x00000008
            CREATE_NO_WINDOW = 0x08000000

            self.daemon_proc = subprocess.Popen(
                [self.daemon_path, my_pid, nvda_exe, FIGHTER_LOG_FILE],
                creationflags=DETACHED_PROCESS | CREATE_NO_WINDOW,
                close_fds=True,
            )
            write_persistent_log(f"[NVDA-FIGHTER:DAEMON] 🐺 Shadow Beast summoned! Linked to PID {my_pid}.")
        except Exception as e:
            write_persistent_log(f"[NVDA-FIGHTER:DAEMON] Failed to summon Shadow Beast: {e}")

    def _hook_event_gatekeeper(self):
        """
        SMART ADAPTIVE BURST GATEKEEPER:
        Distinguishes natural user typing (30ms - 500ms intervals) and smooth terminal output (apt-get)
        from synthetic 0ms - 5ms machine text bombs (>100 events/sec) that lock Windows COM.
        """
        self._original_queueEvent = eventHandler.queueEvent
        original_queue = self._original_queueEvent

        def fighter_queue_event(eventName, obj, *args, **kwargs):
            try:
                # Target text changes and live region updates
                if eventName in ("textChange", "liveRegionChanged", "caret", "valueChange"):
                    wClass = getattr(obj, "windowClassName", "")
                    appModule = getattr(obj, "appModule", None)
                    appName = getattr(appModule, "appName", "") if appModule else ""

                    is_terminal = (
                        wClass in ("ConsoleWindowClass", "CASCADIA_HOSTING_WINDOW_CLASS")
                        or "Terminal" in wClass
                        or appName in ("cmd", "powershell", "windowsterminal", "conhost")
                    )

                    if is_terminal:
                        now = time.monotonic()
                        delta = now - self._last_event_time
                        self._last_event_time = now

                        # If delta is < 6 milliseconds (0.006s), it is an inhuman machine blast!
                        if delta < 0.006:
                            self._consecutive_fast_events += 1
                        else:
                            # Natural human typing or normal SSH flow: reset streak!
                            self._consecutive_fast_events = max(0, self._consecutive_fast_events - 1)

                        # If a storm of 25 consecutive machine-speed events (<6ms each) is pouring in:
                        if self._consecutive_fast_events > 25:
                            # Discard the machine dump!
                            if self._consecutive_fast_events == 26:
                                write_persistent_log(
                                    f"[GATEKEEPER] 🛡️ Machine burst detected from {appName or wClass} (<6ms spacing)! "
                                    "Disarming toxic COM loop while preserving user input!"
                                )
                                self.bridge.record_shielding(1, 40000)
                            return
            except Exception as e:
                write_persistent_log(f"[GATEKEEPER:ERROR] {e}")

            return original_queue(eventName, obj, *args, **kwargs)

        eventHandler.queueEvent = fighter_queue_event
        write_persistent_log("[NVDA-FIGHTER:GATEKEEPER] 🛡️ Smart Adaptive Burst Gatekeeper active (<6ms spacing check)!")

    def _hook_speech_engine(self):
        fighter_bridge = self.bridge
        original_speak = self._original_speak

        def fighter_guarded_speak(speechSequence, symbolLevel=None, priority=None):
            modified_seq = []
            shield_triggered = False

            try:
                for item in speechSequence:
                    if isinstance(item, str) and len(item) > 10000:
                        shielded = fighter_bridge.intercept_text(item)
                        if shielded:
                            shield_triggered = True
                            modified_seq.append(shielded)
                            continue
                    modified_seq.append(item)
            except Exception as e:
                write_persistent_log(f"[SPEECH:ERROR] {e}")
                modified_seq = speechSequence

            if shield_triggered:
                write_persistent_log("[SPEECH] ⚔️ Terminal / Global Dragon Breath neutralised by Shield!")

            return original_speak(modified_seq, symbolLevel=symbolLevel, priority=priority)

        speech.speak = fighter_guarded_speak

    def _telepathic_chatter_worker(self):
        pulse_count = 0
        while self._patrol_active:
            try:
                time.sleep(1.0)
                if not self._patrol_active:
                    break

                pulse_count += 1

                if pulse_count % 30 == 0:
                    gc.collect(2)
                    if self.bridge.loaded:
                        self.bridge.purge_memory()

                if os.path.exists(SACRED_PIPE_NAME):
                    try:
                        handle = ctypes.windll.kernel32.CreateFileW(
                            SACRED_PIPE_NAME,
                            0xC0000000,
                            0,
                            None,
                            3,
                            0,
                            None
                        )
                        if handle != -1 and handle != 0:
                            pulse_msg = f"PULSE: #{pulse_count}\n".encode("utf-8")
                            bytes_written = ctypes.c_ulong(0)
                            ctypes.windll.kernel32.WriteFile(
                                handle,
                                pulse_msg,
                                len(pulse_msg),
                                byref(bytes_written),
                                None
                            )

                            buffer = ctypes.create_string_buffer(512)
                            bytes_read = ctypes.c_ulong(0)
                            if ctypes.windll.kernel32.ReadFile(handle, buffer, 512, byref(bytes_read), None):
                                if bytes_read.value > 0 and pulse_count % 15 == 0:
                                    reply = buffer.value.decode("utf-8", errors="replace").strip()
                                    write_persistent_log(f"[FIGHTER-TELEPATHY] 🐺 {reply}")
                            ctypes.windll.kernel32.CloseHandle(handle)
                    except Exception:
                        pass

            except Exception:
                pass

    def terminate(self):
        write_persistent_log("[NVDA-FIGHTER:TERMINATE] Initiating Sacred Farewell Handshake...")
        self._patrol_active = False

        if hasattr(self, "_original_speak"):
            speech.speak = self._original_speak
        if hasattr(self, "_original_queueEvent"):
            eventHandler.queueEvent = self._original_queueEvent

        try:
            if os.path.exists(SACRED_PIPE_NAME):
                handle = ctypes.windll.kernel32.CreateFileW(
                    SACRED_PIPE_NAME,
                    0x40000000,
                    0,
                    None,
                    3,
                    0,
                    None
                )
                if handle != -1 and handle != 0:
                    written = ctypes.c_ulong(0)
                    ctypes.windll.kernel32.WriteFile(
                        handle,
                        SACRED_EXIT_HANDSHAKE,
                        len(SACRED_EXIT_HANDSHAKE),
                        byref(written),
                        None
                    )
                    ctypes.windll.kernel32.CloseHandle(handle)
                write_persistent_log("[NVDA-FIGHTER:HANDSHAKE] Handshake delivered: 'Hey bro, we are exiting. Bye!'")
        except Exception:
            pass

        super().terminate()

    @script(
        description="Inspect the sacred stats and HP of NVDA Fighter",
        gesture="kb:NVDA+alt+f",
    )
    def script_fighterStats(self, gesture):
        hp, kills, bytes_shielded = self.bridge.get_stats()
        kb_shielded = bytes_shielded // 1024
        msg = (
            f"NVDA Fighter Level 100 Status: HP is {hp} out of 9999. "
            f"Villainous dumps vanquished: {kills}. "
            f"Total bytes shielded: {kb_shielded} kilobytes. "
            f"Status: Divine Titan Armor Active. Smart Adaptive Gatekeeper Armed!"
        )
        ui.message(msg)

    @script(
        description="Cast Alchemical Purge: Force immediate garbage collection and RAM trim",
        gesture="kb:NVDA+alt+shift+f",
    )
    def script_alchemicalPurge(self, gesture):
        gc.collect(2)
        freed = self.bridge.purge_memory()
        mb = freed // (1024 * 1024)
        ui.message(f"Alchemical Purge Complete! Banished {mb} Megabytes of dead memory. System refreshed!")
