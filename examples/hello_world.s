.data
.balign 8
str0:
	.ascii "Sample Hello World"
	.byte 0
/* end data */

.text
.balign 16
.globl _start
_start:
	endbr64
	pushq %rbp
	movq %rsp, %rbp
	movl $12, %edx
	leaq str0(%rip), %rsi
	movl $1, %edi
	callq write
	movl $0, %edi
	callq exit
	leave
	ret
.type _start, @function
.size _start, .-_start
/* end function _start */

.section .note.GNU-stack,"",@progbits
