[bits 64]
[default rel]

trampoline:
        stc
        jmp .hook

        .hook:
        ;; allocate space for xmm regs
        lea rsp, [rsp - 272]

        ;; write xmm regs
        movups [rsp + 8 + 0*16],  xmm0
        movups [rsp + 8 + 1*16],  xmm1
        movups [rsp + 8 + 2*16],  xmm2
        movups [rsp + 8 + 3*16],  xmm3
        movups [rsp + 8 + 4*16],  xmm4
        movups [rsp + 8 + 5*16],  xmm5
        movups [rsp + 8 + 6*16],  xmm6
        movups [rsp + 8 + 7*16],  xmm7
        movups [rsp + 8 + 8*16],  xmm8
        movups [rsp + 8 + 9*16],  xmm9
        movups [rsp + 8 + 10*16], xmm10
        movups [rsp + 8 + 11*16], xmm11
        movups [rsp + 8 + 12*16], xmm12
        movups [rsp + 8 + 13*16], xmm13
        movups [rsp + 8 + 14*16], xmm14
        movups [rsp + 8 + 15*16], xmm15

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

        ;; import_id        
        mov rdi, qword [import_id]

        ;; is_prefix (determined by carry flag)
        sbb esi, esi
        neg esi

        ;; address of register snapshot
        mov rdx, rsp

        ;; return address passed in
        mov rcx, qword [rsp + 0x188]

        ;; pass in slot address
        lea r8, [rsp + 0x188]

        ;; store is_prefix so we know what to do next and align the stack for the call
        push rsi
        sub rsp, 8

        ;; dispatcher call
        call qword [dispatcher_address]

        ;; restore the stack
        add rsp, 8

        ;; is_prefix
        pop rsi

        ;; if rax is -1 then we have to skip calling the original function
        cmp rax, -1
        je .restore

        ;; write what we got in rax to the return slot
        mov [rsp + 0x188], rax

        ;; set carry again
        neg rsi

        ;; start the restore
        jmp .restore

        .original:
        ;; make sure we overwrite the return address
        add rsp, 8

        ;; call original
        call qword [original_function]

        ;; sub from rsp to align it again
        sub rsp, 8

        ;; now we jump to .hook again but with carry cleared
        clc
        jmp .hook

        .restore:
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
        movdqu xmm0, [rsp + 8 + 0*16]
        movdqu xmm1, [rsp + 8 + 1*16]
        movdqu xmm2, [rsp + 8 + 2*16]
        movdqu xmm3, [rsp + 8 + 3*16]
        movdqu xmm4, [rsp + 8 + 4*16]
        movdqu xmm5, [rsp + 8 + 5*16]
        movdqu xmm6, [rsp + 8 + 6*16]
        movdqu xmm7, [rsp + 8 + 7*16]
        movdqu xmm8, [rsp + 8 + 8*16]
        movdqu xmm9, [rsp + 8 + 9*16]
        movdqu xmm10, [rsp + 8 + 10*16]
        movdqu xmm11, [rsp + 8 + 11*16]
        movdqu xmm12, [rsp + 8 + 12*16]
        movdqu xmm13, [rsp + 8 + 13*16]
        movdqu xmm14, [rsp + 8 + 14*16]
        movdqu xmm15, [rsp + 8 + 15*16]

        ;; restore rsp (dealloc)
        lea rsp, [rsp + 272]

        ;; if carry then we should call the original
        jc .original

        ;; now we return
        ret

;; reference table
align 8
import_id: dq 0xdeadbeefdeadbeef
dispatcher_address: dq 0xdeadbeefdeadbeef
original_function: dq 0xdeadbeefdeadbeef
