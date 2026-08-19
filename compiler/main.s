.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	leaq str(%rip), %rdi
	callq print
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.data
.balign 8
str:
	.ascii "hello world\n"
	.byte 0
/* end data */

.section .note.GNU-stack,"",@progbits
