.intel_syntax noprefix
.global print
.text

print:
    mov rsi, rdi   
    mov rax, 1      
    mov rdi, 1      
    mov rdx, 12    
    syscall
    ret
