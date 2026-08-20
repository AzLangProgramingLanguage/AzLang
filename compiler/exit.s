.intel_syntax noprefix
.global exit 
.text

exit:
    mov rax, 60      
    mov rdi, rdi    
    syscall
