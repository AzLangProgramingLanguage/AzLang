.intel_syntax noprefix
.global write 
.text

write:
    mov r10, rsi
    mov rsi, rdi   
    mov rax, 1      
    mov rdi, 1      
    mov rdx, r10 
    syscall
    ret

