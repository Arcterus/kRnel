%macro gdt_entry 3
   dq (((%1) << 32) & 0xFF00000000000000) | \
      (((%3) << 40) & 0x00F0FF0000000000) | \
      (((%2) << 32) & 0x000F000000000000) | \
      (((%1) << 8)  & 0x000000FF00000000) | \
      (((%1) << 16) & 0x00000000FFFF0000) | \
      ((%2)         & 0x000000000000FFFF)
%endmacro

%define GRANULARITY_BIT(val) (val << 15)
%define SIZE_BIT(val)        (val << 14)
%define LONG_BIT(val)        (val << 13)

%define PRESENT_BIT(val)     (val << 7)
%define PRIVILEGE_BITS(val)  (val << 5)
%define EXECUTABLE_BIT(val)  (val << 3)
%define DIRECTION_BIT(val)   (val << 2)
%define CONFORMING_BIT(val)  DIRECTION_BIT(val)
%define READABLE_BIT(val)    (val << 1)
%define WRITABLE_BIT(val)    READABLE_BIT(val)
%define ACCESSED_BIT(val)    (val)

%define ACCESS_FIELD(vals)   ((vals) | (1 << 4))

%define NULL_GDT_ENTRY gdt_entry 0, 0, 0

use32

[global start]

[extern main]

[section .mbhdr]
[extern _loadStart]
[extern _loadEnd]
[extern _bssEnd]

align 8

MbHdr:
   dd 0xE85250D6
   dd 0
   dd HdrEnd - MbHdr
   dq -(0xE85250D6 + 0 + (HdrEnd - MbHdr))

align 8

AddrTag:
   dw 2
   dw 0
   dd AddrTagEnd - AddrTag
   dd MbHdr
   dd _loadStart
   dd _loadEnd
   dd _bssEnd
AddrTagEnd:

align 8

MbiTag:
   dw 1
   dw 0
   dd MbiTagEnd - MbiTag
   dd 6
MbiTagEnd:

align 8

EntryTag:
   dw 3
   dw 0
   dd EntryTagEnd - EntryTag
   dd start
EntryTagEnd:

align 8

FlagsTag:
   dw 4
   dw 0
   dd FlagsTagEnd - FlagsTag
   dd (1 << 0)
FlagsTagEnd:

align 8

FrmBufTag:
   dw 5
   dw 0
   dd FrmBufTagEnd - FrmBufTag
   dd 80
   dd 25
   dd 0
FrmBufTagEnd:

align 8

EndTags:
   dw 0
   dw 0
   dd EndTagsEnd - EndTags
EndTagsEnd:
HdrEnd:

[section .boot]
[extern Stack]
start:
   cmp eax, 0x36d76289  ; if this matches, we're good
   jne $
   mov ecx, ebx         ; save a copy in case we need it later
   mov edx, [ebx]       ; size of multiboot structure
   add edx, ebx         ; maximum address for multiboot structure
   add ebx, 8           ; total_size and reserved are both u32
.find_mem_map:
   cmp ebx, edx         ; if equal, we don't know how much memory
   jge $
   cmp DWORD [ebx], 6   ; check if we've found the memory map
   je .found_mem_map
   mov eax, [ebx + 4]
   add eax, 7
   and eax, ~7
   add ebx, eax
   jmp .find_mem_map
.found_mem_map:         ; yay, memory map found :)
   mov [MemMap], ebx    ; found memory map, so store location for parsing

.load_gdt1:
   mov eax, Gdtr1
   lgdt [eax]

   push 0x08
   push .GdtReady
   retf

.GdtReady:
   mov eax, 0x10
   mov ds, ax
   mov ss, ax
   mov esp, Stack

   call setup_paging_and_long_mode

   mov eax, Gdtr2
   lgdt [Gdtr2]

   push 0x08
   push .Gdt2Ready
   retf

use64

.Gdt2Ready:
   mov eax, 0x10
   mov ds, ax
   mov es, ax
   mov ss, ax

   mov rsp, Stack + 0xFFFFFFFF80000000

   mov rax, Gdtr3
   lgdt [rax]

   mov [gs:0x30], dword 0

   mov rdi, [MemMap]

   call main

   cli
   jmp $

use32
[extern Pml4]
[extern Pdpt]
[extern Pd]

setup_paging_and_long_mode:
   mov eax, Pdpt
   or eax, 1
   mov [Pml4], eax
   mov [Pml4 + 0xFF8], eax

   mov eax, Pd
   or eax, 1
   mov [Pdpt], eax
   mov [Pdpt + 0xFF0], eax

   mov dword [Pd], 0x000083
   mov dword [Pd + 8], 0x200083
   mov dword [Pd + 16], 0x400083
   mov dword [Pd + 24], 0x600083

   mov eax, Pml4
   mov cr3, eax

   mov eax, cr4
   or eax, 1 << 5
   mov cr4, eax

   mov ecx, 0xC0000080
   rdmsr
   or eax, 1 << 8
   wrmsr

   mov eax, cr0
   or eax, 1 << 31
   mov cr0, eax

   ret

TmpGdt:
   NULL_GDT_ENTRY
   gdt_entry 0, 0xFFFFF, ACCESS_FIELD(GRANULARITY_BIT(1) | SIZE_BIT(1) | \
                                      PRESENT_BIT(1) | EXECUTABLE_BIT(1) | \
                                      READABLE_BIT(1))
   gdt_entry 0, 0xFFFFF, ACCESS_FIELD(GRANULARITY_BIT(1) | SIZE_BIT(1) | \
                                      PRESENT_BIT(1) | READABLE_BIT(1))
   NULL_GDT_ENTRY
   gdt_entry 0, 0, ACCESS_FIELD(SIZE_BIT(1) | LONG_BIT(1) | PRESENT_BIT(1) | \
                                EXECUTABLE_BIT(1) | READABLE_BIT(1))
   gdt_entry 0, 0, ACCESS_FIELD(SIZE_BIT(1) | LONG_BIT(1) | PRESENT_BIT(1) | \
                                READABLE_BIT(1))

Gdtr1:
   dw 23
   dd TmpGdt

Gdtr2:
   dw 23
   dd TmpGdt + 24
   dd 0

Gdtr3:
   dw 23
   dq TmpGdt + 24 + 0xFFFFFFFF80000000

MemMap:
   dd 0
