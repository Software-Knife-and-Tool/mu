;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; core fasl module
;;;

;;;
;;; type compilers
;;;
(mu:intern core "%fasl-tag-type"
   (:lambda (sexpr)
     (mu:cdr
      (core:%assq
       (mu:logand 7 (mu:svref (mu:repr :vector sexpr) 0))
       '((0  . :direct)
         (1  . :cons)
         (2  . :func)
         (3  . :stream)
         (4  . :struct)
         (5  . :symbol)
         (6  . :keyword)
         (90 . :vector) ; :t
         (91 . :vector) ; :char
         (92 . :vector) ; :byte
         (93 . :vector) ; :fixnum
         (94 . :vector) ; :float
         )))))

(mu:intern core "%fasl-id-type"
   (:lambda (id)
     (mu:cdr
      (core:%assq
       id
       '((0  . :direct)
         (1  . :cons)
         (2  . :func)
         (3  . :stream)
         (4  . :struct)
         (5  . :symbol)
         (6  . :keyword)
         (90 . :vector) ; :t
         (91 . :vector) ; :char
         (92 . :vector) ; :byte
         (93 . :vector) ; :fixnum
         (94 . :vector) ; :float
         )))))

(mu:intern core "%fasl-vector-type"
   (:lambda (id)
     (mu:cdr
      (core:%assq
       type
       '((90 . :t)
         (91 . :char)
         (92 . :byte)
         (93 . :fixnum)
         (94 . :float))))))

(mu:intern core "%fasl-direct-compiler"
   (:lambda (tag stream)
     (core:%format stream "0 8" `())
     (core:%map-vector
      (:lambda (byte)
        (core:%format stream " ~A" `(,byte)))
      (mu:repr :vector tag))
     (core:%format stream "~%" `())))
             
(mu:intern core "%fasl-string-compiler"
   (:lambda (string stream)
     (:if (mu:eq :direct (core:%fasl-tag-type string))
          (core:%fasl-direct-compiler string stream)
          ((:lambda ()
             (core:%format stream "91 ~A" `(,(mu:vector-length string)))
             (core:%map-vector
              (:lambda (char)
                (core:%format
                 stream
                 " ~A"
                 `(,(mu:svref (mu:repr :vector char) 1))))
                string)
              (core:%format stream "~%" ()))))))

(mu:intern core "%fasl-symbol-compiler"
   (:lambda (symbol stream)
     (core:%format stream "5 ~A" `(,(mu:vector-length (mu:symbol-name symbol))))
     (core:%map-vector
      (:lambda (char)
        (core:%format
         stream
         " ~A"
         `(,(mu:svref (mu:repr :vector char) 1))))
      (mu:symbol-name symbol))
     (core:%format stream "~%" ())))

;;;
;;; compile-file
;;;
(mu:intern core "%fasl-type-compiler-map"
   `(,(mu:cons 'string core:%fasl-string-compiler)
      ,(mu:cons 'char core:%fasl-direct-compiler)
      ,(mu:cons 'fixnum core:%fasl-direct-compiler)
      ,(mu:cons 'keyword core:%fasl-direct-compiler)
      ,(mu:cons 'stream core:%fasl-direct-compiler)
      ,(mu:cons 'symbol core:%fasl-symbol-compiler)
      ,(mu:cons 'float core:%fasl-direct-compiler)))

(mu:intern core "compile-file"
   (:lambda (in-path out-path)
      ((:lambda (in-stream out-stream)
         (core:%format out-stream "((:fasl . \"0.0.1\")~%  (:bytes . #(:byte " ())
         ((:lambda ()
            (mu:fix
             (:lambda (loop)
               (:if (mu:eq core:%eof% loop)
                    ((:lambda ()
                       (mu:close in-stream)
                       core:%eof%))
                    ((:lambda (sexpr)
                       (:if (mu:eq core:%eof% sexpr)
                            core:%eof%
                            ((:lambda ()
                               (core:apply
                                (mu:cdr
                                 (core:%assq
                                  (core:type-of sexpr)
                                  core:%fasl-type-compiler-map))
                                `(,sexpr ,out-stream))
                               (core:null loop)))))
                     (core:read in-stream () core:%eof%))))
             ())
            (core:%format out-stream ")))~%" '())
            (mu:close out-stream)
            :t)))
       (mu:open :file :input in-path)
       (mu:open :file :output out-path))))
