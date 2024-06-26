;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; elf64 utilities
;;;
(core:intern :arm "elf64-write-bytes"
   (:lambda (stream bytes)
      (prelude:%mapc
       (:lambda (byte)
          (:if (prelude:charp byte)
               (prelude:write-char byte stream)
               (:if (prelude:fixnump byte)
                    (prelude:write-byte byte stream)
                    ((:lambda ()
                     ;;; (prelude:raise prelude:stringp byte 'prelude:elf64-write-bytes "untranslated type")
                     (prelude:%map-vector
                      (:lambda (ch)
                         (prelude:write-char ch stream))
                      byte))))))
       bytes)))

(core:intern :arm "elf64-read-bytes"
   (:lambda (nbytes stream)
      ((:lambda (list)
          (core:vector-list :byte (core:cdr list)))
       (core:fix
        (:lambda (loop)
           ((:lambda (nbytes list)
               (:if (prelude:zerop nbytes)
                    loop
                    (core:cons
                     (core:fx-sub nbytes 1)
                     `(,@list ,@(prelude:read-byte stream :t ())))))
            (core:car loop)
            (core:cdr loop)))
        (core:cons nbytes ())))))

(core:intern :arm "elf64-read"
   (:lambda (type stream)
      (:if (core:eq type :half)
           (core:fx-add
            (prelude:read-byte stream t ())
            (core:ash (prelude:read-byte stream :t ()) 8))
           (:if (core:eq type :word)
                (core:fx-add
                 (prelude:elf64-read :half stream)
                 (core:ash (prelude:elf64-read :half stream) 16))
                (:if (core:eq type :u64)
                     (core:fx-add
                      (prelude:elf64-read :word stream)
                      (core:ash (prelude:elf64-read :word stream) 32))
                     (:if (core:eq type :tag)
                          (core:%repr :t (prelude:elf64-read-bytes 8 stream))
                          (prelude:raise size 'prelude:elf64-read "not a legal type")))))))

(core:intern :arm "elf64-write"
   (:lambda (type n stream)
      (:if (core:eq type :half)
           (prelude:elf64-write-bytes
            stream
            (core:logand n #xff)
            (core:logand (core:ash n -8) #xff))
           (:if (core:eq type :word)
                ((:lambda ()
                 (prelude:elf64-write :half n stream)
                 (prelude:elf64-write :half (core:ash n -16) stream)))
                (:if (core:eq type :u64)
                     ((:lambda ()
                      (prelude:elf64-write :half n stream)
                      (prelude:elf64-write :half (core:ash n -32) stream)))
                     (:if (core:eq type :tag)
                          (prelude:elf64-write :u64 (core:tag-of n) stream)
                          (t (prelude:raise size 'el64-write "not a legal type"))))))))

(core:intern :arm "elf64-write-header"
   (:lambda (out size)
      ;;; (prelude:raise prelude:streamp out 'elf64-write-header "not a stream designator")
      ;;; header 64 bit little endian current version
      (prelude:elf64-write-bytes `(,out #x7f "ELF" 2 1 1 0 0 0 0 0 0 0 0 0))
                                            ; 0 e_ident
      (prelude:elf64-write :half 0 out)     ; 16 e_type
      (prelude:elf64-write :half 0 out)     ; 18 e_machine
      (prelude:elf64-write :half 1 out)     ; 20 e_version
      (prelude:elf64-write :half 0 out)     ; 22 e_entry
      (prelude:elf64-write :u64 0 out)      ; 24 e_phoff
      (prelude:elf64-write :u64 56 out)     ; 32 e_shoff
      (prelude:elf64-write :word 0 out)     ; 40 e_flags
      (prelude:elf64-write :half 0 out)     ; 42 e_ehsize
      (prelude:elf64-write :half 0 out)     ; 44 e_phentsize
      (prelude:elf64-write :half 0 out)     ; 46 e_phnum
      (prelude:elf64-write :half 0 out)     ; 48 e_shentsize
      (prelude:elf64-write :half 0 out)     ; 50 e_shnum
      (prelude:elf64-write :half 0 out)     ; 52 e_shstrndx
      (prelude:elf64-write :word 0 out)     ; 56 sh_name
      (prelude:elf64-write :word 0 out)     ; 60 sh_type
      (prelude:elf64-write :u64 0 out)      ; 68 sh_flags
      (prelude:elf64-write :u64 0 out)      ; 76 sh_addr
      (prelude:elf64-write :u64 0 out)      ; 84 sh_offset
      (prelude:elf64-write :u64 size out)   ; 92 sh_size
      (prelude:elf64-write :word 0 out)     ; 100 sh_link
      (prelude:elf64-write :word 0 out)     ; 102 sh_info
      (prelude:elf64-write :u64 0 out)      ; 110 sh_addralign
      (prelude:elf64-write :u64 0 out)))    ; 118 sh_entsize

(core:intern :arm "elf64-read-header"
   (:lambda (stream)
      (core:unwind-protect
       (:lambda (ex)
          (prelude:%exceptf
           core:error-output
           "elf64-read-header exception: ~A on ~A by ~S from ~A~%"
           ()
           ex))
       (:lambda ()
          `(,(core:cons "magic" (prelude:elf64-read :u64 elf64))
            ,(core:cons "class" (prelude:read-byte elf64 :t ()))
            ,(core:cons "data-encoding" (prelude:read-byte elf64 :t ()))
            ,(core:cons "file-version" (prelude:read-byte elf64 t ()))
            ,(core:cons "OS-ABI" (prelude:read-byte elf64 :t ()))
            ,(core:cons "ABI-version" (prelude:read-byte elf64 :t ()))
            ,(core:cons "e_value" (prelude:elf64-read :half elf64))
            ,(core:cons "e_machine" (prelude:elf64-read :half elf64))
            ,(core:cons "e_version" (prelude:elf64-read :half elf64))
            ,(core:cons "e_entry" (prelude:elf64-read :half elf64))
            ,(core:cons "e_phoff" (prelude:elf64-read :u64 elf64))
            ,(core:cons "e_shoff" (prelude:elf64-read :u64 elf64))
            ,(core:cons "e_flags" (prelude:elf64-read :word elf64))
            ,(core:cons "e_ehsize" (prelude:elf64-read :half elf64))
            ,(core:cons "e_phentsize" (prelude:elf64-read :half elf64))
            ,(core:cons "e_phnum" (prelude:elf64-read :half elf64))
            ,(core:cons "e_shentsize" (prelude:elf64-read :half elf64))
            ,(core:cons "e_shnum" (prelude:elf64-read :half elf64))
            ,(core:cons "e_shstrndx" (prelude:elf64-read :half elf64))
            ,(core:cons "sh_name" (prelude:elf64-read :word elf64))
            ,(core:cons "sh_type" (prelude:elf64-read :word elf64))
            ,(core:cons "sh_flags" (prelude:elf64-read :u64 elf64))
            ,(core:cons "sh_addr" (prelude:elf64-read :u64 elf64))
            ,(core:cons "sh_offset" (prelude:elf64-read :u64 elf64))
            ,(core:cons "sh_size" (prelude:elf64-read :u64 elf64))
            ,(core:cons "sh_link" (prelude:elf64-read :word elf64))
            ,(core:cons "sh_info" (prelude:elf64-read :word elf64))
            ,(core:cons "sh_addralign" (prelude:elf64-read :u64 elf64))
            ,(core:cons "sh_entsize" (prelude:elf64-read :u64 elf64)))))))

(core:intern :arm "elf64-print-header"
  (:lambda (elf64-header)
     ;;; (prelude:raise prelude:consp elf64-header 'prelude:elf64-print-header "not an association list")
     (prelude:%mapc
      (:lambda (entry)
         (prelude:format t "    ~A: ~A~%" `(,(core:car entry) ,(core:cdr entry))))
      elf64-header)))

#|
(core:intern :arm "elf64-dump"
   (:lambda (path)
      ;;; (prelude:raise prelude:stringp path 'prelude:elf64-dump "not a pathname")
      ((:lambda (elf64)
          ((:lambda (magic)
              ;;; (prelude:raise prelude:null magic 'prelude:elf64-dump "error reading magic")
              ;;; (prelude:raise (:lambda (magic) (core:eq #x7f magic)) magic 'prelude:elf64-dump "wrong magic")
              (prelude:format :t "~A: elf64 file (magic #x~X ~A~A~A)~%"
                           `(path
                             magic
                             (prelude:read-char elf64 :t ())
                             (prelude:read-char elf64 :t ())
                             (prelude:read-char elf64 :t ())))
              (prelude:format :t "~A: elf64 file header~%" `(path))
              (prelude:elf64-print-header (prelude:elf64-read-header elf64))
              (prelude:format t "~A: body:~%" path)
              (core:fix
               (:lambda (loop)
                  (:if (prelude:eof elf64)
                       loop
                       ((:lambda (tag)
                           (prelude:format :t "[~A]~%" `(,(prelude:type-of tag) ,tag))
                           (prelude:null loop))
                        (prelude:elf64-read :tag elf64))))
               ()))
             (prelude:read-byte elf64 () ()))
(core:open :file :input path)))))
|#
