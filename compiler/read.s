section .data
    filename db "/etc/os-release", 0      ; Null-terminated filename string

section .bss
    buffer   resb 512               ; Reserve a 512-byte buffer for the file contents
    fd       resq 1                 ; Reserve 8 bytes to store the file descriptor

section .text
    global _start

_start:
    ; 1. OPEN THE FILE (sys_open)
    mov rax, 2                      ; Syscall ID for sys_open
    mov rdi, filename               ; Pointer to the filename string
    mov rsi, 0                      ; Flags: O_RDONLY (0)
    mov rdx, 0                      ; Mode: Not creating a file, so 0 is fine
    syscall                         ; Invoke kernel
    
    ; Check if open failed (negative value means error)
    cmp rax, 0
    jl error_exit
    mov [fd], rax                   ; Save the returned file descriptor

    ; 2. READ FROM THE FILE (sys_read)
    mov rax, 0                      ; Syscall ID for sys_read
    mov rdi, [fd]                   ; File descriptor we saved earlier
    mov rsi, buffer                 ; Pointer to our memory buffer
    mov rdx, 512                    ; Max bytes to read
    syscall                         ; Invoke kernel

    ; Check if read failed
    cmp rax, 0
    jl error_exit
    
    ; Save the actual number of bytes read to r12 for the write syscall
    mov r12, rax                    

    ; 3. WRITE TO STDOUT (sys_write - so we can see the text)
    mov rax, 1                      ; Syscall ID for sys_write
    mov rdi, 1                      ; File descriptor: STDOUT (1)
    mov rsi, buffer                 ; Pointer to the text buffer
    mov rdx, r12                    ; Exact count of bytes read from the file
    syscall                         ; Invoke kernel

    ; 4. CLOSE THE FILE (sys_close)
    mov rax, 3                      ; Syscall ID for sys_close
    mov rdi, [fd]                   ; File descriptor to close
    syscall                         ; Invoke kernel

normal_exit:
    ; 5. EXIT PROGRAM SUCCESSFULLY (sys_exit)
    mov rax, 60                     ; Syscall ID for sys_exit
    mov rdi, 0                      ; Return status 0
    syscall

error_exit:
    ; EXIT PROGRAM WITH ERROR STATUS
    mov rax, 60                     ; Syscall ID for sys_exit
    mov rdi, 1                      ; Return status 1 (Error)
    syscall
