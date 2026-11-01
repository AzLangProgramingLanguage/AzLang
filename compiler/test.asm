format ELF64 executable
segment readable executable

entry main
ret:
ret 5
main:
call ret
mov rax, 60
xor rdi, rdi
syscall
