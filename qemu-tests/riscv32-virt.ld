ENTRY(_start)

MEMORY
{
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
  .text : {
    *(.text.init)
    *(.text .text.*)
  } > RAM

  .rodata : {
    *(.rodata .rodata.*)
  } > RAM

  .data : {
    *(.data .data.*)
  } > RAM

  .bss : {
    *(.bss .bss.*)
    *(COMMON)
  } > RAM

  /DISCARD/ : {
    *(.eh_frame)
    *(.eh_frame_hdr)
  }
}
