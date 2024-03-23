;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(core:intern :prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(core:intern :prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (core:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (core:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (core:eq ch #\#)
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

(core:intern :prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (core:vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(core:intern :prelude "%read-byte-bits"
   (:lambda (stream)
     (core:cdr
      (core:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (core:cons 0 `(,(core:fx-sub 8 nbits) ,@(core:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (core:eq :tmacro syntax) (core:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (core:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (core:cons 0 `(,(core:fx-sub 8 nbits) ,@(core:ash byte nbits)))))
                                 (:if (prelude:%orf (core:eq #\0 ch) (core:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(core:logor
                                           (:if (core:eq #\1 ch) 1 0)
                                           (core:fx-mul byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (core:car loop)
          (core:cdr loop)))
        '(8 . 0)))))
             
(core:intern :prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (core:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (core:vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(core:fx-add length (core:fx-mul 8 (core:length loop))))))
                       (core:car byte-descriptor)
                       (core:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(core:intern :prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (core:eval (prelude:compile (prelude:read stream () ())))))

(core:intern :prelude "%read-sharp-symbol"
   (:lambda (ch stream)
     ((:lambda (symbol)
          (:if (core:eq :symbol (core:type-of symbol))
               (core:symbol (core:sy-name symbol))
               (prelude:raise symbol 'prelude:%read-sharp-symbol "not a symbol")))
       (prelude:%read-atom ch stream))))

(core:intern :prelude "%read-sharp-number"
  (:lambda (base stream)
    (core:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (core:eq base #\x)
                          (prelude:parse-integer (core:get-str prelude:%reader-stream%) 16)
                          (:if (core:eq base #\b)
                               (prelude:parse-integer (core:get-str prelude:%reader-stream%) 2)
                               (prelude:parse-integer (core:get-str prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (core:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (core:eq base #\x)
                                     (prelude:parse-integer (core:get-str prelude:%reader-stream%) 16)
                                     (:if (core:eq base #\b)
                                          (prelude:parse-integer (core:get-str prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (core:get-str prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(core:intern :prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (core:apply
        (core:sy-val (core:cdr (prelude:%assoc ch sharp-table)))
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
(core:intern :prelude "%read-list-eol%" (core:symbol "eol"))

(core:intern :prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(core:intern :prelude "%read-list"
   (:lambda (ch stream)
      (core:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (core:eq el prelude:%read-list-eol%)
                 loop
                 (:if (core:eq :symbol (core:type-of el))
                      (:if (core:eq (core:sy-name el) ".")
                           ((:lambda (dotted)
                              (:if (core:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr core:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(core:intern :prelude "%read-string"
   (:lambda (ch stream)
     (core:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (core:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(core:intern :prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(core:intern :prelude "%read-line-comment"
    (:lambda (ch stream)
      (core:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (core:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(core:intern :prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (core:apply
            (core:sy-val (core:cdr (prelude:%assoc ch macro-table)))
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
(core:intern :prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (core:ns-find prelude:%char-macro-ns% (core:vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(core:intern :prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (core:intern prelude:%char-macro-ns% (core:vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
