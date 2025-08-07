# Tests

This project uses several testing roms to verify the functionality of the emulator. The tests are located in the `tests` directory and are organized into subdirectories based on the type of test.

## Running Tests

You must provide the test ROMs in the `test_roms` directory. The test ROMs are not included in the repository due to their size and licensing issues.
Some of them can be automatically downloaded by running the `fetch_test_roms.sh` script.

To run the tests, use the following command:

```bash
cargo test --workspace --release
```

This will run all tests in the `tests` directory. The tests are organized into subdirectories based on the type of test.

## Test ROMs

### [Blaarg's test ROMs](https://github.com/retrio/gb-test-roms)

Also refered as `gb-test-roms`.

| Test ROM               | Status |
|------------------------|--------|
| cpu_instrs.gb          | :+1:   |
| instr_timing.gb        |        |
| interrupt_time.gb      |        |
| mem_timing.gb          |        |
| mem_timing-2.gb        |        |
| cgb_sound.gb           |        |
| dmg_sound.gb           |        |
| oam_bug.gb             |        |
| halt_bug.gb            |        |

### [Nitro's test ROMs](https://github.com/nitro2k01/little-things-gb)

Also refered as `little-things-gb`.

| Test ROM                      | Status |
|-------------------------------|--------|
| double-halt-cancel-gbconly.gb |        |

### [Mooneye's test Suite](https://github.com/Gekkio/mooneye-test-suite/)

Also refered as `mts`.

#### acceptance/
| Test ROM                   | Status |
|----------------------------|--------|
| add_sp_e_timing.gb         |        |
| boot_div-S.gb              |        |
| boot_div-dmg0.gb           |        |
| boot_div-dmgABCmgb.gb      |        |
| boot_div2-S.gb             |        |
| boot_hwio-S.gb             |        |
| boot_hwio-dmg0.gb          |        |
| boot_hwio-dmgABCmgb.gb     |        |
| boot_regs-dmg0.gb          |        |
| boot_regs-dmgABC.gb        |        |
| boot_regs-mgb.gb           |        |
| boot_regs-sgb.gb           |        |
| boot_regs-sgb2.gb          |        |
| call_cc_timing.gb          |        |
| call_cc_timing2.gb         |        |
| call_timing.gb             |        |
| call_timing2.gb            |        |
| di_timing-GS.gb            |        |
| div_timing.gb              |        |
| ei_sequence.gb             |        |
| ei_timing.gb               |        |
| halt_ime0_ei.gb            |        |
| halt_ime0_nointr_timing.gb |        |
| halt_ime1_timing.gb        |        |
| halt_ime1_timing2-GS.gb    |        |
| if_ie_registers.gb         |        |
| intr_timing.gb             |        |
| jp_cc_timing.gb            |        |
| jp_timing.gb               |        |
| ld_hl_sp_e_timing.gb       |        |
| oam_dma_restart.gb         |        |
| oam_dma_start.gb           |        |
| oam_dma_timing.gb          |        |
| pop_timing.gb              |        |
| push_timing.gb             |        |
| rapid_di_ei.gb             |        |
| ret_cc_timing.gb           |        |
| ret_timing.gb              |        |
| reti_intr_timing.gb        |        |
| reti_timing.gb             |        |
| rst_timing.gb              |        |

#### acceptance/bits
| Test ROM               | Status |
|------------------------|--------|
| mem_oam.gb             |        |
| reg_f.gb               |        |
| unused_hwio-GS.gb      |        |

#### acceptance/instr
| Test ROM               | Status |
|------------------------|--------|
| daa.gb                 | :+1:   |

#### acceptance/interrupts
| Test ROM               | Status |
|------------------------|--------|
| ie_push.gb             |        |

#### acceptance/oam_dma
| Test ROM               | Status |
|------------------------|--------|
| basic.gb               |        |
| reg_read.gb            |        |
| sources-GS.gb          |        |

#### acceptance/ppu
| Test ROM                       | Status |
|--------------------------------|--------|
| hblank_ly_scx_timing-GS.gb     |        |
| intr_1_2_timing-GS.gb          |        |
| intr_2_0_timing.gb             |        |
| intr_2_mode0_timing.gb         |        |
| intr_2_mode0_timing_sprites.gb |        |
| intr_2_mode3_timing.gb         |        |
| intr_2_oam_ok_timing.gb        |        |
| lcdon_timing-GS.gb             |        |
| lcdon_write_timing-GS.gb       |        |
| stat_irq_blocking.gb           |        |
| stat_lyc_onoff.gb              |        |
| vblank_stat_intr-GS.gb         |        |

#### acceptance/serial
| Test ROM                     | Status |
|------------------------------|--------|
| boot_sclk_align-dmgABCmgb.gb |        |

#### acceptance/timer
| Test ROM                | Status |
|-------------------------|--------|
| div_write.gb            |        |
| rapid_toggle.gb         |        |
| tim00.gb                |        |
| tim00_div_trigger.gb    |        |
| tim01.gb                |        |
| tim01_div_trigger.gb    |        |
| tim10.gb                |        |
| tim10_div_trigger.gb    |        |
| tim11.gb                |        |
| tim11_div_trigger.gb    |        |
| tima_reload.gb          |        |
| tima_write_reloading.gb |        |
| tma_write_reloading.gb  |        |

### [Mattcurrie's Acid2 Test Roms](https://github.com/mattcurrie/cgb-acid2)
| Test ROM               | Status |
|------------------------|--------|
| dmg-acid2.gb           | :+1:   |
| cgb-acid2.gb           | :+1:   |
