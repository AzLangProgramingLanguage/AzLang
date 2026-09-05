.data
.balign 8
c:
	.ascii "Hello World"
	.byte 0
/* end data */

.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	movl $11, %edx
	leaq c(%rip), %rsi
	movl $1, %edi
	callq write
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.section .note.GNU-stack,"",@progbits
