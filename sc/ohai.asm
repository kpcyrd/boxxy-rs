global _start

section .text

_start:
    jmp message

proc:
    pop rsi
    mov rax, 1      ; write
    mov rdi, rax    ; stdout
    mov rdx, 5
    syscall

    mov rax, 60     ; exit
    xor rdi, rdi
    syscall

message:
    call proc
    msg db 'ohai', 0xa

section .data
