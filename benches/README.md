# Assembler
```asm
benches::raw_sstatic:
 mov     eax, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rdi, +, 8], eax
 mov     rax, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rdi], rax
 ret
```

```asm
benches::raw_static:
 push    rbx
 mov     rbx, rdi
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB7_1
 mov     ecx, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rax, +, 8], ecx
 mov     rcx, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rax], rcx
 mov     qword, ptr, [rbx], rax
 movaps  xmm0, xmmword, ptr, [rip, +, .LCPI7_0]
 movups  xmmword, ptr, [rbx, +, 8], xmm0
 pop     rbx
 ret
.LBB7_1:
 mov     edi, 12
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
```

```asm
benches::raw_dyn:
 push    rbx
 sub     rsp, 32
 mov     rbx, rdi
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB8_1
 mov     qword, ptr, [rsp, +, 8], rax
 mov     qword, ptr, [rsp, +, 16], 12
 mov     ecx, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rax, +, 8], ecx
 mov     rcx, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rax], rcx
 mov     qword, ptr, [rsp, +, 24], 12
 mov     rax, qword, ptr, [rsp, +, 8]
 mov     qword, ptr, [rbx], rax
 mov     rax, qword, ptr, [rsp, +, 16]
 mov     qword, ptr, [rbx, +, 8], rax
 mov     qword, ptr, [rbx, +, 16], 12
 add     rsp, 32
 pop     rbx
 ret
.LBB8_1:
 mov     edi, 12
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
```

```asm
benches::ibuffer:
 push    r14
 push    rbx
 sub     rsp, 40
 mov     rbx, rdi
 mov     r14d, 12
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB9_1
 mov     qword, ptr, [rsp], rax
 movq    xmm0, r14
 movdqu  xmmword, ptr, [rsp, +, 8], xmm0
 mov     qword, ptr, [rsp, +, 32], 12
 mov     ecx, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rax, +, 8], ecx
 mov     rcx, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rax], rcx
 mov     qword, ptr, [rsp, +, 24], 12
 mov     rax, qword, ptr, [rsp]
 mov     qword, ptr, [rbx], rax
 mov     rax, qword, ptr, [rsp, +, 8]
 mov     qword, ptr, [rbx, +, 8], rax
 mov     qword, ptr, [rbx, +, 16], 12
 add     rsp, 40
 pop     rbx
 pop     r14
 ret
.LBB9_1:
 mov     edi, 12
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
```

```asm
benches::buffer_bytes:
 push    r14
 push    rbx
 push    rax
 mov     r14, rdi
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB9_1
 mov     rbx, rax
 mov     edi, 12
 call    qword, ptr, [rip, +, _ZN5bytes9bytes_mut25original_capacity_to_repr17hdee3bb1ec6cf6c4fE@GOTPCREL]
 lea     rax, [4*rax, +, 1]
 mov     qword, ptr, [r14], rbx
 mov     qword, ptr, [r14, +, 16], 12
 mov     qword, ptr, [r14, +, 24], rax
 mov     rax, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rbx], rax
 mov     eax, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rbx, +, 8], eax
 mov     qword, ptr, [r14, +, 8], 12
 add     rsp, 8
 pop     rbx
 pop     r14
 ret
.LBB9_1:
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, _ZN5alloc5alloc18handle_alloc_error17hb4f79dda046419e4E@GOTPCREL]
 ud2
```
