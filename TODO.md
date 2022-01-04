# Things to research
- CPU:
    - Timings for unmapped regions
    - Misc loads/stores with P == 0 && W == 1
    - Modifying reserved bits in the SPSRs
    - SPSR restores in non-exception modes
    - CPSR invalid modes
    - Bus conflict after user-mode LDMs/STMs
    - Interaction between user-mode LDMs/STMs (S bit set, when not loading r15) and writeback (W bit set)
    - MCRR/MRRC, both on the ARM9 and on the ARM7 (even though the latter's architecture doesn't support them)
    - Unpredictable r15 writes (most are still untested)
    - Behavior when setting the T bit with a CPSR MSR
    - ARM7 Thumb BLX reg behavior
    - ARM7 CP14 behavior for opcodes other than MRC/MCR
    - ARM7 carry flag value after multiplications
    - ARM7 LDRD/STRD behavior and timings
    - ARM9 CP15 unpredictable region permission bit combinations
    - Instruction timing:
        - ARM7 undefined instruction timing
        - ARM9 ARM DP instructions r15 write timings (the manual says some take 3 cycles and some take 4)
        - ARM9 Thumb DP instructions timings (they might be different from ARM ones)
        - ARM9 data abort timings
        - ARM9 ARM word/byte transfer scaled offset and postincrement timings (should be the same)
        - ARM9 Thumb data transfer timings with register offsets (since they can't use register-specified shift amounts, they might use a faster path)
        - ARM9 MCR/MRC
        - Empty reg list LDM/STM timing
        - Interaction of LDM/STM bank switches with timing    
- DS slot:
    - AUXSPICNT bit 13
    - IO port behavior when AUXSPICNT bit 15 is 0
    - IO port behavior when the DS slot is allocated to the other CPU
    - Behavior when the DS slot access right are switched to the other CPU mid-transfer
    - Reading/writing from/to DS slot ports while the other CPU has access rights
    - ROM interface:
        - ROMCTRL bit 14, "SE" (and to a lesser extent ROMCTRL bit 30, "WR", which Arisotura's GBATEK addendum documents as disabling all delays)
        - ROM behavior when changing ROMCTRL settings while busy, especially resetting ROMCTRL bit 31
        - ROM response with no card inserted (currently it's an endless stream of 0xFF bytes)
    - ROM chip:
        - Response to unrecognized commands in unencrypted and KEY1 mode
        - How repeated KEY1 commands work on larger/newer carts
        - Address range for secure area transfer commands
        - Response to some commands that don't send a meaningful one, such as "Activate KEY2 encryption mode"
    - SPI interface:
        - Response while busy
        - Behavior when trying to send data while busy
        - Behavior when changing AUXSPICNT while busy
- Flash:
    - Behavior when issuing erase/program/write commands while writes are disabled in the status register
    - Behavior when exceeding 256 bytes in a single write/program: the manual says that only the 256 bytes loaded last are programmed, but it's unclear whether the address should be advanced even for the unwritten bytes or the write should start at the original address, completely ignoring the unwritten bytes
    - Value for high-z responses (GBATEK seems to imply 0xFF)
- Graphics:
    - POWCNT1 bit 0
    - Exact behavior of VCOUNT writes, and especially their interaction with the 3D engine
    - Behavior of the unused DISPCNT bits for engine B
    - Invalid VRAMCNT_x MST values behavior
    - MASTER_BRIGHT mode 3 behavior
- RTC:
    - Response when reading beyond the end of a register
    - Behavior when starting a transfer with a byte read
    - Behavior when changing the data direction in the middle of a byte transfer or command
    - Behavior when CS rises while /SCK is low
- SPI:
    - SPICNT bit 10 (GBATEK says it's bugged as the data register it's 8-bit, but it's unclear what data it should even contain, and how it should work for writes)
    - Behavior when changing SPICNT while busy
    - Response while disabled/busy
    - Behavior when switching between devices while SPICNT bit 11 is set
    - Behavior when accessing device 3
    - Power management device response after writes of any kind
- Audio:
    - PSG format 3 for channels 0..=7
    - Using repeat mode 0 and going out of bounds
    - Hold bit behavior
    - Specifying a loop start + loop size < 16 bytes for PCM modes (GBATEK says they'll hang)
    - Changing the source address while running
    - Changing the format while running (what if the FIFO becomes misaligned?)
    - Research on the sample FIFO in general, since GBATEK barely mentions it
- Timers:
    - Delay (2 cycles on GBA, unknown on DS)
- DMA:
    - Source address control mode 3
    - Transfer delays/timing (on the GBA they seem to take 3 cycles to start at all times)
    - Behavior when specifying both repeat and immediate mode
    - Behavior when changing control bits while running
    - Behavior when using invalid addresses (>= 0x08000000 for "internal memory" DMAs on the ARM7, or addresses in the BIOS region for all ARM7 DMAs): they seem to get masked, but thorough verification is needed
    - Behavior when crossing memory regions or wrapping back to the start of the address space (assuming addresses do get masked)
    - ARM7 DMA open bus exact behavior
- EXMEMCNT bits 13-15
- IO register unused bit behavior

# Missing functionality
- Wi-Fi
- RTC alarms
- KEYCNT
- More accurate direct boot
- ARM7 regular open bus (the ARM9 seems to always return 0)
- Accurate GBA slot open bus values (they're described on GBATEK)
- Keep the ARM9 running while running a DMA and executing code from TCM, though that would require an accurate implementation of bus stalling, which doesn't seem feasible without a large amount of boilerplate and a noticeable performance impact
- Sleep mode
- GBA slot
- Absent SIO (even if there's no actual functionality, all ports should still work)
- GBA mode

# Misc
- Deduplicate DMA code