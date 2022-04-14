mov cl,1
print_hi:
  mov ah,0x0e
  mov al, 'h'
  int 0x10
  mov al, 'i'
  int 0x10
  mov al, '!'
  int 0x10
  mov al, 10
  int 0x10
  mov al, 13
  int 0x10
  sub cl,1
  jnz print_hi

loop:
  mov ah,0x00
  int 0x16
  mov ah,0x0e
  int 0x10
  jmp loop
  