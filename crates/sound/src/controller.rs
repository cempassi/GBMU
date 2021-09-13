pub struct SoundController {
    controller: SoundControlReg,
    so1: u8,
    so2: u8,
    ch1: SoundChannel1,
    ch2: SoundChannel2,
    ch3: SoundChannel3,
    ch4: SoundChannel4,
}

/// Sound Control Registers
struct SoundControlReg {
    /// FF24 - NR50 - Channel control / ON-OFF / Volume (R/W)
    ///  The volume bits specify the “Master Volume” for Left/Right sound output. SO2 goes to the left headphone, and SO1 goes to the right.
    ///  Bit 7   - Output Vin to SO2 terminal (1=Enable)
    ///  Bit 6-4 - SO2 output level (volume)  (0-7)
    ///  Bit 3   - Output Vin to SO1 terminal (1=Enable)
    ///  Bit 2-0 - SO1 output level (volume)  (0-7)
    channel_control: u8,
    /// FF25 - NR51 - Selection of Sound output terminal (R/W)
    ///  Each channel can be panned hard left, center, or hard right.
    ///  Bit 7 - Output sound 4 to SO2 terminal
    ///  Bit 6 - Output sound 3 to SO2 terminal
    ///  Bit 5 - Output sound 2 to SO2 terminal
    ///  Bit 4 - Output sound 1 to SO2 terminal
    ///  Bit 3 - Output sound 4 to SO1 terminal
    ///  Bit 2 - Output sound 3 to SO1 terminal
    ///  Bit 1 - Output sound 2 to SO1 terminal
    ///  Bit 0 - Output sound 1 to SO1 terminal
    output_terminal: u8,
    /// FF26 - NR52 - Sound on/off
    ///  If your GB programs don’t use sound then write 00h to this register.
    ///  Disabling the sound controller by clearing Bit 7 destroys the contents of all sound registers.
    ///  Also, it is not possible to access any sound registers (execpt FF26) while the sound controller is disabled.
    ///   Bit 7 - All sound on/off  (0: stop all sound circuits) (Read/Write)
    ///   Bit 3 - Sound 4 ON flag (Read Only)
    ///   Bit 2 - Sound 3 ON flag (Read Only)
    ///   Bit 1 - Sound 2 ON flag (Read Only)
    ///   Bit 0 - Sound 1 ON flag (Read Only)
    ///  Bits 0-3 of this register are read only status bits, writing to these bits does NOT enable/disable sound.
    ///  The flags get set when sound output is restarted by setting the Initial flag (Bit 7 in NR14-NR44),
    ///  the flag remains set until the sound length has expired (if enabled).
    ///  A volume envelopes which has decreased to zero volume will NOT cause the sound flag to go off.
    sound_enable: u8,
}

