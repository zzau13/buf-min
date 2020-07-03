# Assembler
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
 je      .LBB8_1
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
.LBB8_1:
 mov     edi, 12
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
```

```asm
benches::buffer_bytes:
 push    r14
 push    rbx
 sub     rsp, 56
 mov     r14, rdi
 mov     ebx, 12
 mov     edi, 12
 mov     esi, 1
 call    qword, ptr, [rip, +, __rust_alloc@GOTPCREL]
 test    rax, rax
 je      .LBB8_6
 mov     qword, ptr, [rsp, +, 32], rax
 movq    xmm0, rbx
 movdqu  xmmword, ptr, [rsp, +, 40], xmm0
 mov     rdi, rax
 call    qword, ptr, [rip, +, _ZN5bytes9bytes_mut4vptr17h6d77e3803747e609E@GOTPCREL]
 mov     rbx, rax
 mov     edi, 12
 call    qword, ptr, [rip, +, _ZN5bytes9bytes_mut25original_capacity_to_repr17h50071a2aa30aa092E@GOTPCREL]
 lea     rax, [4*rax, +, 1]
 mov     qword, ptr, [rsp], rbx
 movaps  xmm0, xmmword, ptr, [rip, +, .LCPI8_0]
 movups  xmmword, ptr, [rsp, +, 8], xmm0
 mov     qword, ptr, [rsp, +, 24], rax
 mov     rdi, rsp
 call    qword, ptr, [rip, +, _ZN87_$LT$bytes..bytes_mut..BytesMut$u20$as$u20$core..convert..AsMut$LT$$u5b$u8$u5d$$GT$$GT$6as_mut17h2b93f7932ac5a500E@GOTPCREL]
 mov     rcx, qword, ptr, [rsp, +, 8]
 mov     edx, dword, ptr, [rip, +, .L__unnamed_2+8]
 mov     dword, ptr, [rax, +, rcx, +, 8], edx
 mov     rdx, qword, ptr, [rip, +, .L__unnamed_2]
 mov     qword, ptr, [rax, +, rcx], rdx
 add     qword, ptr, [rsp, +, 8], 12
 movups  xmm0, xmmword, ptr, [rsp, +, 16]
 movups  xmmword, ptr, [r14, +, 16], xmm0
 movups  xmm0, xmmword, ptr, [rsp]
 movups  xmmword, ptr, [r14], xmm0
 add     rsp, 56
 pop     rbx
 pop     r14
 ret
.LBB8_6:
 mov     edi, 12
 mov     esi, 1
 call    alloc::raw_vec::RawVec<T,A>::allocate_in::{{closure}}
 ud2
.LBB8_4:
 mov     rbx, rax
 mov     rdi, rsp
 call    core::ptr::drop_in_place
 mov     rdi, rbx
 call    _Unwind_Resume
 ud2
.LBB8_7:
 mov     rbx, rax
 lea     rdi, [rsp, +, 32]
 call    core::ptr::drop_in_place
 mov     rdi, rbx
 call    _Unwind_Resume
 ud2
```