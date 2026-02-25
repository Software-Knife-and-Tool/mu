;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; elf64 utilities
;;;
(mu:intern core "%elf64-write-bytes"
   (:lambda (stream list)
     (core:mapc
      (cond
        ((charp bytes) (write-byte (ch-ansi bytes) stream))
        ((fixnump bytes) (write-byte bytes stream))
        ((stringp bytes)
         (do* ((i 0 (1+ i)))
              ((fixnum< (1- (length bytes)) i) ())
           (write-byte (ch-ansi (schar bytes i)) stream)))
        (t (error bytes "elf64-write-bytes: untranslated type"))))
     bytes))

(mu:intern core "%elf64-read"
   (:lambda (type elf64)
  (let ((elf64-read (%lambda (type stream) (funcall core:elf64-read type stream))))
    (cond
      ((eq type :half)
       (fixnum+ (read-byte elf64 t ()) (ash (read-byte elf64 t ()) 8)))
      ((eq type :word)
       (fixnum+ (elf64-read :half elf64) (ash (elf64-read :half elf64) 16)))
      ((eq type :u64)
       (fixnum+ (elf64-read :word elf64) (ash (elf64-read :word elf64) 32)))
      ((eq type :tag)
       (core:of-tag (elf64-read :u64 elf64)))
      (t (error size "elf64-read: not a legal type")))))

(mu:intern core elf64-write (type n stream)
  (let ((elf64-write (%lambda (type n stream) (funcall core:elf64-write type n stream))))
    (cond
      ((eq type :half)
       (elf64-write-bytes stream
                    (logand n #xff)
                    (logand (ash n -8) #xff)))
      ((eq type :word)
       (elf64-write :half n stream)
       (elf64-write :half (ash n -16) stream))
      ((eq type :u64)
       (elf64-write :half n stream)
       (elf64-write :half (ash n -32) stream))
      ((eq type :tag)
       (elf64-write :u64 (core:tag-of n) stream))
      (t (error size "el64-write: not a legal type")))))

(mu:intern core elf64-write-header (out size)
  (errorp-unless streamp out "elf64-write-header: not a stream")
  ;;; header 64 bit little endian current version
  (elf64-write-bytes out #x7f "ELF" 2 1 1 0 0 0 0 0 0 0 0 0) ;;; 0 e_ident
  (elf64-write :half 0 out)  ;;; 16 e_type
  (elf64-write :half 0 out)  ;;; 18 e_machine
  (elf64-write :half 1 out)  ;;; 20 e_version
  (elf64-write :half 0 out)  ;;; 22 e_entry
  (elf64-write :u64 0 out)   ;;; 24 e_phoff
  (elf64-write :u64 56 out)  ;;; 32 e_shoff
  (elf64-write :word 0 out)  ;;; 40 e_flags
  (elf64-write :half 0 out)  ;;; 42 e_ehsize
  (elf64-write :half 0 out)  ;;; 44 e_phentsize
  (elf64-write :half 0 out)  ;;; 46 e_phnum
  (elf64-write :half 0 out)  ;;; 48 e_shentsize
  (elf64-write :half 0 out)  ;;; 50 e_shnum
  (elf64-write :half 0 out)  ;;; 52 e_shstrndx
  (elf64-write :word 0 out)  ;;; 56 sh_name
  (elf64-write :word 0 out)  ;;; 60 sh_type
  (elf64-write :u64 0 out)   ;;; 68 sh_flags
  (elf64-write :u64 0 out)   ;;; 76 sh_addr
  (elf64-write :u64 0 out)   ;;; 84 sh_offset
  (elf64-write :u64 size out)   ;;; 92 sh_size
  (elf64-write :word 0 out)  ;;; 100 sh_link
  (elf64-write :word 0 out)  ;;; 102 sh_info
  (elf64-write :u64 0 out)   ;;; 110 sh_addralign
  (elf64-write :u64 0 out))  ;;; 118 sh_entsize

(mu:intern core elf64-read-header (elf64)
  (core:unwind-protect
      (%lambda (ex)
        (core:%format t "error: exception while reading header~%")
        (print-except ex "read-header")
        ())
    (%lambda ()
      (let ((top
             (core:list
              (mu:cons "magic" (elf64-read :u64 elf64))
              (mu:cons "class" (read-byte elf64 t ()))
              (mu:cons "data-encoding" (read-byte elf64 t ()))
              (mu:cons "file-version" (read-byte elf64 t ()))
              (mu:cons "OS-ABI" (read-byte elf64 t ()))
              (mu:cons "ABI-version" (read-byte elf64 t ())))))
        (dotimes (i 6) (read-byte elf64 t ()))
        (append top
                (core:list
                 (mu:cons "e_object" (elf64-read :half elf64))
                 (mu:cons "e_machine" (elf64-read :half elf64))
                 (mu:cons "e_version" (elf64-read :half elf64))
                 (mu:cons "e_entry" (elf64-read :half elf64))
                 (mu:cons "e_phoff" (elf64-read :u64 elf64))
                 (mu:cons "e_shoff" (elf64-read :u64 elf64))
                 (mu:cons "e_flags" (elf64-read :word elf64))
                 (mu:cons "e_ehsize" (elf64-read :half elf64))
                 (mu:cons "e_phentsize" (elf64-read :half elf64))
                 (mu:cons "e_phnum" (elf64-read :half elf64))
                 (mu:cons "e_shentsize" (elf64-read :half elf64))
                 (mu:cons "e_shnum" (elf64-read :half elf64))
                 (mu:cons "e_shstrndx" (elf64-read :half elf64))
                 (mu:cons "sh_name" (elf64-read :word elf64))
                 (mu:cons "sh_type" (elf64-read :word elf64))
                 (mu:cons "sh_flags" (elf64-read :u64 elf64))
                 (mu:cons "sh_addr" (elf64-read :u64 elf64))
                 (mu:cons "sh_offset" (elf64-read :u64 elf64))
                 (mu:cons "sh_size" (elf64-read :u64 elf64))
                 (mu:cons "sh_link" (elf64-read :word elf64))
                 (mu:cons "sh_info" (elf64-read :word elf64))
                 (mu:cons "sh_addralign" (elf64-read :u64 elf64))
                 (mu:cons "sh_entsize" (elf64-read :u64 elf64))))))))

(mu:intern core elf64-print-header (elf64-header)
  (errorp-unless consp elf64-header "print-header: not an association list")
  (mapc
   (%lambda (entry)
     (core:%format t "    ~A: ~A~%" (car entry) (cdr entry)))
   elf64-header))

(mu:intern core elf64-dump (path)
  (errorp-unless stringp path "elf64-dump: not a pathname")
  (with-open-file elf64 path :input
     (let ((magic (read-byte elf64 () ())))
       (%if (or (null magic) (eofp elf64) (not (eq #x7f magic)))
           (core:%format t "~A: not a elf64 file, magic #x~X~%" path magic)            
           (progn
             (core:%format t "~A: elf64 file (magic #x~X ~A~A~A)~%" path magic
                     (ansi-ch (core:read-byte elf64 t ()))
                     (ansi-ch (core:read-byte elf64 t ()))
                     (ansi-ch (core:read-byte elf64 t ())))
             (core:%format t "~A: elf64 file header~%" path)
             (elf64-print-header (elf64-read-header elf64))
             (core:%format t "~A: body:~%" path)
             (do* ((tag (elf64-read :tag elf64) (elf64-read :tag elf64)))
                  ((eofp elf64) t)
               (core:%format :t "[~A]~%" (type-of tag) tag)))))))
