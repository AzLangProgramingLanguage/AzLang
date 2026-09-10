
.intel_syntax noprefix
.text
.global _start

_start:
    mov rdi, [rsp] 

    mov rsi, [rsp + 8]

    cmp rdi, 1            
    jle no_args

    mov rdx, [rsp + 16]

    mov rax,1
    mov rsi, rdx
    mov rdx, 4
    syscall


no_args:
    mov rax, 60
    xor rdi, rdi
    syscall
