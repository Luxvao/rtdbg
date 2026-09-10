[bits 64]
[default rel]

trampoline:
        ;; allocate space for xmm regs
        sub rsp, 256

        ;; write xmm regs
        movdqu [rsp + 0*16],  xmm0
        movdqu [rsp + 1*16],  xmm1
        movdqu [rsp + 2*16],  xmm2
        movdqu [rsp + 3*16],  xmm3
        movdqu [rsp + 4*16],  xmm4
        movdqu [rsp + 5*16],  xmm5
        movdqu [rsp + 6*16],  xmm6
        movdqu [rsp + 7*16],  xmm7
        movdqu [rsp + 8*16],  xmm8
        movdqu [rsp + 9*16],  xmm9
        movdqu [rsp + 10*16], xmm10
        movdqu [rsp + 11*16], xmm11
        movdqu [rsp + 12*16], xmm12
        movdqu [rsp + 13*16], xmm13
        movdqu [rsp + 14*16], xmm14
        movdqu [rsp + 15*16], xmm15

        ;; write GPRs
        push rax
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15

        ;; function_id
        mov rdi, qword [hook_id]

        ;; is_prefix
        mov rsi, 1

        ;; address of the register snapshot
        mov rdx, rsp

        ;; dispatcher call
        call qword [dispatcher_address]

        ;; pop the registers
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx
        pop rax

        ;; restore xmm
        movdqu xmm0, [rsp + 0*16]
        movdqu xmm1, [rsp + 1*16]
        movdqu xmm2, [rsp + 2*16]
        movdqu xmm3, [rsp + 3*16]
        movdqu xmm4, [rsp + 4*16]
        movdqu xmm5, [rsp + 5*16]
        movdqu xmm6, [rsp + 6*16]
        movdqu xmm7, [rsp + 7*16]
        movdqu xmm8, [rsp + 8*16]
        movdqu xmm9, [rsp + 9*16]
        movdqu xmm10, [rsp + 10*16]
        movdqu xmm11, [rsp + 11*16]
        movdqu xmm12, [rsp + 12*16]
        movdqu xmm13, [rsp + 13*16]
        movdqu xmm14, [rsp + 14*16]
        movdqu xmm15, [rsp + 15*16]

        ;; restore rsp (dealloc)
        add rsp, 256

        ;; align the stack
        sub rsp, 8

        ;; call the original
        call qword [original_function]

        ;; restore
        add rsp, 8

        ;; now we do the same thing again for postfix
         
        ;; allocate space for xmm regs
        sub rsp, 256

        ;; write xmm regs
        movdqu [rsp + 0*16],  xmm0
        movdqu [rsp + 1*16],  xmm1
        movdqu [rsp + 2*16],  xmm2
        movdqu [rsp + 3*16],  xmm3
        movdqu [rsp + 4*16],  xmm4
        movdqu [rsp + 5*16],  xmm5
        movdqu [rsp + 6*16],  xmm6
        movdqu [rsp + 7*16],  xmm7
        movdqu [rsp + 8*16],  xmm8
        movdqu [rsp + 9*16],  xmm9
        movdqu [rsp + 10*16], xmm10
        movdqu [rsp + 11*16], xmm11
        movdqu [rsp + 12*16], xmm12
        movdqu [rsp + 13*16], xmm13
        movdqu [rsp + 14*16], xmm14
        movdqu [rsp + 15*16], xmm15

        ;; write GPRs
        push rax
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15

        ;; function_id
        mov rdi, qword [hook_id]

        ;; is_prefix
        mov rsi, 0

        ;; address of register snapshot
        mov rdx, rsp

        ;; dispatcher call
        call qword [dispatcher_address]

        ;; pop the registers
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx
        pop rax

        ;; restore xmm
        movdqu xmm0, [rsp + 0*16]
        movdqu xmm1, [rsp + 1*16]
        movdqu xmm2, [rsp + 2*16]
        movdqu xmm3, [rsp + 3*16]
        movdqu xmm4, [rsp + 4*16]
        movdqu xmm5, [rsp + 5*16]
        movdqu xmm6, [rsp + 6*16]
        movdqu xmm7, [rsp + 7*16]
        movdqu xmm8, [rsp + 8*16]
        movdqu xmm9, [rsp + 9*16]
        movdqu xmm10, [rsp + 10*16]
        movdqu xmm11, [rsp + 11*16]
        movdqu xmm12, [rsp + 12*16]
        movdqu xmm13, [rsp + 13*16]
        movdqu xmm14, [rsp + 14*16]
        movdqu xmm15, [rsp + 15*16]

        ;; restore rsp (dealloc)
        add rsp, 256

        ;; finally we ret (back to the original caller that did call <plt entry>)
        ret

;; Reference table
align 8
hook_id: dq 0xdeadbeefdeadbeef
dispatcher_address: dq 0xdeadbeefdeadbeef
original_function: dq 0xdeadbeefdeadbeef
