OUTPUT_ARCH( "riscv" )
OUTPUT_FORMAT("elf64-littleriscv", "elf64-littleriscv", "elf64-littleriscv")


MEMORY
{
   MEM_REGION : ORIGIN = 0x80000000 , LENGTH = 0x200000
}
_STACK_SIZE = DEFINED(_STACK_SIZE) ? _STACK_SIZE : 0x10000;

ENTRY(_entry)

SECTIONS
{
.text : ALIGN(64){
   text_start = .;
   KEEP (*(.boot))
   KEEP (*(.vectors))
   *(.text)
   *(.text.*)
   text_end = .;
} > MEM_REGION

.rodata : ALIGN(64) {
   __rodata_start = .;
   *(.rodata)
   *(.rodata.*)
   *(.srodata*)
   __rodata_end = .;
} > MEM_REGION


.data : ALIGN(64) {
   _data = .;
   *(.data)
   *(.data.*)
   *(.sdata)
   *(.sdata.*)
   _edata = .;
} > MEM_REGION

.resource_table : ALIGN(64)
{
   KEEP(*(.resource_table));
} > MEM_REGION

.bss (NOLOAD) : ALIGN(64){
   bss_start = .;
   *(.bss)
   *(.bss.*)
   *(.sbss)
   *(.sbss.*)
   *(COMMON)
   bss_end = .;
} > MEM_REGION

.stack (NOLOAD) : ALIGN(64){
   stack_end = .;
   . += _STACK_SIZE;
   stack_top = .;
} > MEM_REGION

.heap (NOLOAD) :  ALIGN(64){
   _heap_start = .;
} > MEM_REGION
_end = .;
}





