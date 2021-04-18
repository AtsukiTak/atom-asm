section .text
global start

start:
  mov rax, 0x2000000 + 4    ;sys_write
  mov rdi, 1
  mov rsi, msg
  mov rdx, len
  syscall

  mov rax, 0x2000000 + 1    ;sys_exit
  mov rdi, 0
  syscall

section .data
  msg db  'hello, world',0x0A
  len equ $ - msg
