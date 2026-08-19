.intel_syntax noprefix
.global print
.text
print:
  mov r15,rdi

  mov rax, 1 
  mov rdi,  1
  lea rsi, [r15]
  mov rdx, 14
  syscall
  ret

