.data
.balign 8
str0:
	.ascii "2222"
	.byte 0
/* end data */

.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	movl $4, %esi
	leaq str0(%rip), %rdi
	callq write
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.section .note.GNU-stack,"",@progbits
