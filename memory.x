MEMORY {
  /* Instruction RAM (executable code) */
  iram0_0_seg (RX) :        org = 0x40080000, len = 0x20000  /* 128KB IRAM0_MAIN */

  /* Data RAM */
  dram0_0_seg (RW)  :       org = 0x3FFE0000, len = 0x20000  /* 128KB ISRAM0_MAIN */
  rtc_iram_seg (RW) :       org = 0x3FF80000, len = 0x2000   /* 8KB RTC_FAST */
  rtc_slow_seg (RW) :       org = 0x50000000, len = 0x2000   /* 8KB RTC_SLOW */

  /* Flash */
  irom0_0_seg (RX) :        org = 0x400C2000, len = 0xB3E000  /* 11512KB FLASH */
}

/* Required aliases for linker script */
REGION_ALIAS("ROTEXT", iram0_0_seg);
REGION_ALIAS("RODATA", dram0_0_seg);
REGION_ALIAS("RTC_FAST", rtc_iram_seg);
REGION_ALIAS("RTC_SLOW", rtc_slow_seg);