.text
.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	movl $50, %edi
	callq exit
	movl $0, %edi
	callq exit
	leave
	ret
.type main, @function
.size main, .-main
/* end function main */

.section .note.GNU-stack,"",@progbits
