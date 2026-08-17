.intel_syntax noprefix
.global print
.text
print:
  mov rax, 1
  mov rdi, 1
  lea rsi, hello_string
  mov rdx, 14
  syscall
  ret
.data
hello_string:
        .asciz  "Hello, world!\n"
