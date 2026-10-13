format ELF64 executable
entry _start

segment readable executable

include "/home/prestgg/projects/AzLang/sdk/src/exit.s"
include "/home/prestgg/projects/AzLang/sdk/src/write.s"



azfn_modadd:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov [rbp-8], rdi
    mov [rbp-16], rsi
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    mov rcx, rax
    pop rax
    add rax, rcx
    leave
    ret
azfn_mymod_write:
    push rbp
    mov rbp, rsp
    mov rax, 1
    push rax
    lea rax, [mymod_str0]
    push rax
    mov rax, 3
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    leave
    ret

azfn_add:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov [rbp-8], rdi
    mov [rbp-16], rsi
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    mov rcx, rax
    pop rax
    add rax, rcx
    leave
    ret
azfn_fib:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    mov [rbp-8], rdi
    mov rax, [rbp-8]
    push rax
    mov rax, 2
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx eax, al
    test rax, rax
    jz .br2
    mov rax, [rbp-8]
    leave
    ret
    jmp .end1
.br2:
.end1:
    mov rax, [rbp-8]
    push rax
    mov rax, 1
    mov rcx, rax
    pop rax
    sub rax, rcx
    push rax
    pop rdi
    call azfn_fib
    push rax
    mov rax, [rbp-8]
    push rax
    mov rax, 2
    mov rcx, rax
    pop rax
    sub rax, rcx
    push rax
    pop rdi
    call azfn_fib
    mov rcx, rax
    pop rax
    add rax, rcx
    leave
    ret
_start:
    push rbp
    mov rbp, rsp
    sub rsp, 120
    mov rax, 5
    mov [rbp-8], rax
    mov rax, 0
    mov [rbp-16], rax
    mov rax, 0
    mov [rbp-24], rax
    lea rax, [test_all_str0]
    mov [rbp-32], rax
    lea rax, [test_all_str1]
    mov [rbp-32], rax
.w3:
    mov rax, [rbp-16]
    push rax
    mov rax, 5
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx eax, al
    test rax, rax
    jz .we4
    mov rax, [rbp-16]
    push rax
    mov rax, 1
    mov rcx, rax
    pop rax
    add rax, rcx
    mov [rbp-16], rax
    mov rax, [rbp-16]
    push rax
    mov rax, 3
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br6
    jmp .w3
    jmp .end5
.br6:
.end5:
    mov rax, [rbp-24]
    push rax
    mov rax, [rbp-16]
    mov rcx, rax
    pop rax
    add rax, rcx
    mov [rbp-24], rax
    jmp .w3
.we4:
.l7:
    mov rax, [rbp-24]
    push rax
    mov rax, 100
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setg al
    movzx eax, al
    test rax, rax
    jz .br10
    jmp .le8
    jmp .end9
.br10:
.end9:
    mov rax, [rbp-24]
    push rax
    mov rax, 10
    mov rcx, rax
    pop rax
    add rax, rcx
    mov [rbp-24], rax
    jmp .l7
.le8:
    mov rax, [rbp-24]
    push rax
    mov rax, 20
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setg al
    movzx eax, al
    test rax, rax
    jz .br12
    mov rax, 1
    push rax
    lea rax, [test_all_str2]
    push rax
    mov rax, 3
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end11
.br12:
    mov rax, [rbp-24]
    push rax
    mov rax, 5
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setg al
    movzx eax, al
    test rax, rax
    jz .br13
    mov rax, 1
    push rax
    lea rax, [test_all_str3]
    push rax
    mov rax, 3
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end11
.br13:
    mov rax, 1
    push rax
    lea rax, [test_all_str4]
    push rax
    mov rax, 5
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
.end11:
    mov rax, [rbp-8]
    push rax
    mov rax, 3
    push rax
    pop rsi
    pop rdi
    call azfn_add
    mov [rbp-40], rax
    mov rax, [rbp-40]
    push rax
    mov rax, 8
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br15
    mov rax, 1
    push rax
    lea rax, [test_all_str5]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end14
.br15:
.end14:
    mov rax, 20
    push rax
    mov rax, 22
    push rax
    pop rsi
    pop rdi
    call azfn_modadd
    mov [rbp-48], rax
    mov rax, [rbp-48]
    push rax
    mov rax, 42
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br17
    mov rax, 1
    push rax
    lea rax, [test_all_str6]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end16
.br17:
.end16:
    mov rax, 10
    push rax
    pop rdi
    call azfn_fib
    mov [rbp-56], rax
    mov rax, [rbp-56]
    push rax
    mov rax, 55
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br19
    mov rax, 1
    push rax
    lea rax, [test_all_str7]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end18
.br19:
.end18:
    mov rax, [rbp-16]
    push rax
    mov rax, 5
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br21
    mov rax, 1
    push rax
    lea rax, [test_all_str8]
    push rax
    mov rax, 8
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end20
.br21:
.end20:
    mov rax, [rbp-24]
    push rax
    mov rax, 102
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br23
    mov rax, 1
    push rax
    lea rax, [test_all_str9]
    push rax
    mov rax, 8
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end22
.br23:
.end22:
    mov rax, 17
    push rax
    mov rax, 5
    mov rcx, rax
    pop rax
    cqo
    idiv rcx
    mov [rbp-64], rax
    mov rax, 17
    push rax
    mov rax, 5
    mov rcx, rax
    pop rax
    cqo
    idiv rcx
    mov rax, rdx
    mov [rbp-72], rax
    mov rax, [rbp-64]
    push rax
    mov rax, 3
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br25
    mov rax, [rbp-72]
    push rax
    mov rax, 2
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br27
    mov rax, 1
    push rax
    lea rax, [test_all_str10]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end26
.br27:
.end26:
    jmp .end24
.br25:
.end24:
    mov rax, 2
    push rax
    mov rax, 3
    mov rcx, rax
    pop rax
    imul rax, rcx
    push rax
    mov rax, 4
    mov rcx, rax
    pop rax
    add rax, rcx
    push rax
    mov rax, 10
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br29
    mov rax, 1
    push rax
    lea rax, [test_all_str11]
    push rax
    mov rax, 8
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end28
.br29:
.end28:
    mov rax, 1
    mov [rbp-80], rax
    mov rax, [rbp-80]
    test rax, rax
    sete al
    movzx eax, al
    test rax, rax
    jz .br31
    mov rax, 1
    push rax
    lea rax, [test_all_str12]
    push rax
    mov rax, 6
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end30
.br31:
.end30:
    mov rax, [rbp-80]
    test rax, rax
    jz .br33
    mov rax, 1
    push rax
    lea rax, [test_all_str13]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end32
.br33:
.end32:
    mov rax, 10
    push rax
    mov rax, 9
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setg al
    movzx eax, al
    push rax
    mov rax, 1
    push rax
    mov rax, 2
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx eax, al
    mov rcx, rax
    pop rax
    and rax, rcx
    test rax, rax
    jz .br35
    mov rax, 1
    push rax
    lea rax, [test_all_str14]
    push rax
    mov rax, 8
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end34
.br35:
.end34:
    mov rax, 0
    mov [rbp-88], rax
    mov rax, 0
    mov [rbp-96], rax
.w36:
    mov rax, [rbp-88]
    push rax
    mov rax, 4
    mov rcx, rax
    pop rax
    cmp rax, rcx
    setl al
    movzx eax, al
    test rax, rax
    jz .we37
    mov rax, [rbp-96]
    push rax
    mov rax, 1
    mov rcx, rax
    pop rax
    add rax, rcx
    mov [rbp-96], rax
    mov rax, [rbp-88]
    push rax
    mov rax, 1
    mov rcx, rax
    pop rax
    add rax, rcx
    mov [rbp-88], rax
    jmp .w36
.we37:
    mov rax, [rbp-96]
    push rax
    mov rax, 4
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br39
    mov rax, 1
    push rax
    lea rax, [test_all_str15]
    push rax
    mov rax, 9
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end38
.br39:
.end38:
    mov rax, [rbp-32]
    push rax
    lea rax, [test_all_str1]
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br41
    mov rax, 1
    push rax
    lea rax, [test_all_str16]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end40
.br41:
.end40:
    mov rax, 0
    push rax
    mov rax, 7
    mov rcx, rax
    pop rax
    sub rax, rcx
    mov [rbp-104], rax
    mov rax, [rbp-104]
    push rax
    mov rax, -7
    mov rcx, rax
    pop rax
    cmp rax, rcx
    sete al
    movzx eax, al
    test rax, rax
    jz .br43
    mov rax, 1
    push rax
    lea rax, [test_all_str17]
    push rax
    mov rax, 7
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    jmp .end42
.br43:
.end42:
    mov rax, 1
    push rax
    lea rax, [test_all_str18]
    push rax
    mov rax, 5
    push rax
    pop rdx
    pop rsi
    pop rdi
    call write
    mov rax, 0
    push rax
    pop rdi
    call exit
    xor edi, edi
    mov eax, 60
    syscall

mymod_str0 db "mod",0
test_all_str0 db "old",0
test_all_str1 db "new",0
test_all_str2 db "big",0
test_all_str3 db "mid",0
test_all_str4 db "small",0
test_all_str5 db " add-ok",0
test_all_str6 db " mod-ok",0
test_all_str7 db " fib-ok",0
test_all_str8 db " loop-ok",0
test_all_str9 db " math-ok",0
test_all_str10 db " div-ok",0
test_all_str11 db " prec-ok",0
test_all_str12 db " WRONG",0
test_all_str13 db " not-ok",0
test_all_str14 db " bool-ok",0
test_all_str15 db " while-ok",0
test_all_str16 db " str-ok",0
test_all_str17 db " neg-ok",0
test_all_str18 db " DONE",0
