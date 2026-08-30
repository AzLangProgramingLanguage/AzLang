.data
.balign 8
str0:
	.ascii "Hello world"
	.byte 0
/* end data */

.text
.balign 16
.globl main
main:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	leaq str0(%rip), %rdi
	callq print
	movl $0, %edi
	callq exit
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.section .note.GNU-stack,"",@progbits
