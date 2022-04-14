bits 16
org 0x7c00

boot:
  cli

  ; print test
  mov si,hello
  mov ah,0x0e
  .loop:
    lodsb
    or al,al
    jz finish_msg
    int 0x10
    jmp .loop
  finish_msg:
    xor ax,ax       ; set ax register to 0
    mov ds,ax       ; clear ds and es registers
    mov es,ax       ; since they might have nonzero values
    mov ss,ax       ; move stack to a safe place
    mov sp,0x8000   ; want SS:SP to be 0x8000:0x0000
    mov dh,17       ; read 17 sectors of kernel
    mov bx,0x9000   ; ES:BX is where kernel is loaded

  call disk_load

disk_load:
  push dx
  
  mov al,dh
  mov ch,0x00   ; cylinder 0
  mov dh,0x00   ; head 0
  mov cl,0x02   ; second sector; first sector is boot
  mov ah,0x02   ; read interrupt

  int 0x13      ; call interrupt
  jc error      ; carry flag is set if error occurs
  pop dx
  cmp dh,al     ; check number of sectors read
  jne error
  ret

ERROR_MESSAGE: db 'ERROR ENCOUNTERED', 10, 13, 0
hello: db "Hello world!", 10, 13, 0
error:
  ; mov bx, ERROR_MESSAGE
  ; call print_string
  mov bx, 'Q'
  int 0x10
  jmp $

BOOT_DRIVE: db 0

times 510-($-$$) db 0
db 0x55
db 0xaa
