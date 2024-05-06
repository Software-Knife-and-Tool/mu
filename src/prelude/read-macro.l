;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(lib:intern prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(lib:intern prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (lib:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (lib:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (lib:eq ch #\#)
                                      stream
                                      (prelude:null loop))))
                          (prelude:read-char stream () ())))
                       (:if (prelude:null ch)
                            (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                            (prelude:read-char stream () ())))
                      (prelude:null loop)))
               (:if (prelude:null ch)
                    (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                    (prelude:read-char stream () ())))))
       ())))

(lib:intern prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (lib:make-vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(lib:intern prelude "%read-byte-bits"
   (:lambda (stream)
     (lib:cdr
      (lib:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (lib:cons 0 `(,(lib:fx-sub 8 nbits) ,@(lib:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (lib:eq :tmacro syntax) (lib:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (lib:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (lib:cons 0 `(,(lib:fx-sub 8 nbits) ,@(lib:ash byte nbits)))))
                                 (:if (prelude:%orf (lib:eq #\0 ch) (lib:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(lib:logor
                                           (:if (lib:eq #\1 ch) 1 0)
                                           (lib:fx-mul byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (lib:car loop)
          (lib:cdr loop)))
        '(8 . 0)))))
             
(lib:intern prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (lib:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (lib:make-vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(lib:fx-add length (lib:fx-mul 8 (lib:length loop))))))
                       (lib:car byte-descriptor)
                       (lib:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(lib:intern prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (lib:eval (prelude:compile (prelude:read stream () ())))))

(lib:intern prelude "%read-sharp-symbol"
   (:lambda (ch stream)
     ((:lambda (symbol)
          (:if (lib:eq :symbol (lib:type-of symbol))
               (lib:make-symbol (lib:symbol-name symbol))
               (prelude:raise symbol 'prelude:%read-sharp-symbol "not a symbol")))
       (prelude:%read-atom ch stream))))

(lib:intern prelude "%read-sharp-number"
  (:lambda (base stream)
    (lib:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (lib:eq base #\x)
                          (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 16)
                          (:if (lib:eq base #\b)
                               (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 2)
                               (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (lib:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (lib:eq base #\x)
                                     (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 16)
                                     (:if (lib:eq base #\b)
                                          (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (lib:get-string prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(lib:intern prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (lib:apply
        (lib:symbol-value (lib:cdr (prelude:%assoc ch sharp-table)))
        `(,ch ,stream)))
     (prelude:read-char stream () ())
     '((#\| . prelude:%read-sharp-comment)
       (#\( . prelude:%read-sharp-vector)
       (#\* . prelude:%read-sharp-bit-vector)
       (#\. . prelude:%read-sharp-dot)
       (#\\ . prelude:%read-sharp-char)
       (#\b . prelude:%read-sharp-number)
       (#\x . prelude:%read-sharp-number)
       (#\d . prelude:%read-sharp-number)
       (#\: . prelude:%read-sharp-symbol)))))

;;;
;;; list reader
;;;
(lib:intern prelude "%read-list-eol%" (lib:make-symbol "eol"))

(lib:intern prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(lib:intern prelude "%read-list"
   (:lambda (ch stream)
      (lib:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (lib:eq el prelude:%read-list-eol%)
                 loop
                 (:if (lib:eq :symbol (lib:type-of el))
                      (:if (lib:eq (lib:symbol-name el) ".")
                           ((:lambda (dotted)
                              (:if (lib:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr lib:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(lib:intern prelude "%read-string"
   (:lambda (ch stream)
     (lib:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (lib:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(lib:intern prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(lib:intern prelude "%read-line-comment"
    (:lambda (ch stream)
      (lib:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (lib:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(lib:intern prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (lib:apply
            (lib:symbol-value (lib:cdr (prelude:%assoc ch macro-table)))
            `(,ch ,stream)))
       '((#\" . prelude:%read-string)
         (#\# . prelude:%read-sharp)
         (#\' . prelude:%read-quote)
         (#\` . prelude:%qq-reader)
         (#\( . prelude:%read-list)
         (#\) . prelude:%read-list-eol)
         (#\; . prelude:%read-line-comment)))))

;;;
;;; get/set-macro-character
;;;
(lib:intern prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (lib:find prelude:%char-macro-ns% (lib:make-vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(lib:intern prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (lib:intern prelude:%char-macro-ns% (lib:make-vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
