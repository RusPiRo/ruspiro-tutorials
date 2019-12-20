/***********************************************************************************************************************
 * linker script to define memory addresse / sections and symbols of the binary to be build
 * known constraints:
 * Entry point address need to be 0x80000 in 64bit mode, the binarry is not allowed to start prior to this address
 * Stack pointer need to be at least 8Bit and heap pointer 16Bit aligned
 *
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

ENTRY(__boot)

SECTIONS
{
	/* start memory address for RPi modules in RAM in 64bit mode*/
	. = 0x80000;
	    
    .text : { KEEP(*(.text.boot)) *(.text*) }
    .rodata : { *(.rodata*) }
	PROVIDE(_data = .);
    .data : { *(.data*) }
    
	. = ALIGN(8);
    .init_array : {
		__init_start = .;
		*(.init_array)
		*(.init_array.*)
		__init_end = .;
	}
    
    /* bss section, contains all static variables of the c code */
    .bss : { 
    	__bss_start__ = .;
    	*(.bss*)
    	*(COMMON)
    	__bss_end__ = .;
    }

	/* fill the binary to always have an aligned binary size - needed for the bootloader on RPi to work properly */
	.fill : {
		FILL(0xDEADBEEF)
		. += 1;
		. = ALIGN(64) - 1;
		BYTE(0xAA) 
	}

	/* after the code the stack pointers will start */
	/* they cannot start from the end of usable memory as the memory */
	/* is split betwean arm cpu and gpu and the gpu memory size is not known during compile time */
	. = ALIGN(16);
	__stack_end__ = .;
	. += 0x10000;
	__stack_top_core3__ = .;
	. += 0x10000;
	__stack_top_core2__ = .;
	. += 0x10000;
	__stack_top_core1__ = .;
	. += 0x01000; /* stack space EL2 mode 4kB - stack goes backwards from memory end towards begin*/
	__stack_top_EL2__ = .;
	. += 0x0E000;
	__stack_top_EL1__ = .;
	. += 0x01000;
	__stack_top_EL0__ = .;
	__stack_top_core0__ = .;
	__stack_top__ = .;
	/* the heap memory address space starts where the executable and the static variables ends (aligned to 4kB to fit into MMU page)*/
    . = ALIGN(4096);
	__heap_start = .;
    /* heap end is defined by the usage split of arm/gpu - set this to a fixed value for the time beeing */
	__heap_end = 0x3E000000;
}
