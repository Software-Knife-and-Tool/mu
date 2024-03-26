;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(libcore:intern :prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(libcore:intern :prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (libcore:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (libcore:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (libcore:eq ch #\#)
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

(libcore:intern :prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (libcore:vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(libcore:intern :prelude "%read-byte-bits"
   (:lambda (stream)
     (libcore:cdr
      (libcore:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (libcore:cons 0 `(,(libcore:fx-sub 8 nbits) ,@(libcore:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (libcore:eq :tmacro syntax) (libcore:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (libcore:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (libcore:cons 0 `(,(libcore:fx-sub 8 nbits) ,@(libcore:ash byte nbits)))))
                                 (:if (prelude:%orf (libcore:eq #\0 ch) (libcore:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(libcore:logor
                                           (:if (libcore:eq #\1 ch) 1 0)
                                           (libcore:fx-mul byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (libcore:car loop)
          (libcore:cdr loop)))
        '(8 . 0)))))
             
(libcore:intern :prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (libcore:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (libcore:vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(libcore:fx-add length (libcore:fx-mul 8 (libcore:length loop))))))
                       (libcore:car byte-descriptor)
                       (libcore:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(libcore:intern :prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (libcore:eval (prelude:compile (prelude:read stream () ())))))

(libcore:intern :prelude "%read-sharp-symbol"
   (:lambda (ch stream)
     ((:lambda (symbol)
          (:if (libcore:eq :symbol (libcore:type-of symbol))
               (libcore:symbol (libcore:sy-name symbol))
               (prelude:raise symbol 'prelude:%read-sharp-symbol "not a symbol")))
       (prelude:%read-atom ch stream))))

(libcore:intern :prelude "%read-sharp-number"
  (:lambda (base stream)
    (libcore:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (libcore:eq base #\x)
                          (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 16)
                          (:if (libcore:eq base #\b)
                               (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 2)
                               (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (libcore:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (libcore:eq base #\x)
                                     (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 16)
                                     (:if (libcore:eq base #\b)
                                          (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (libcore:get-str prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(libcore:intern :prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (libcore:apply
        (libcore:sy-val (libcore:cdr (prelude:%assoc ch sharp-table)))
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
(libcore:intern :prelude "%read-list-eol%" (libcore:symbol "eol"))

(libcore:intern :prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(libcore:intern :prelude "%read-list"
   (:lambda (ch stream)
      (libcore:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (libcore:eq el prelude:%read-list-eol%)
                 loop
                 (:if (libcore:eq :symbol (libcore:type-of el))
                      (:if (libcore:eq (libcore:sy-name el) ".")
                           ((:lambda (dotted)
                              (:if (libcore:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr libcore:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(libcore:intern :prelude "%read-string"
   (:lambda (ch stream)
     (libcore:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (libcore:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(libcore:intern :prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(libcore:intern :prelude "%read-line-comment"
    (:lambda (ch stream)
      (libcore:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (libcore:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(libcore:intern :prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (libcore:apply
            (libcore:sy-val (libcore:cdr (prelude:%assoc ch macro-table)))
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
(libcore:intern :prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (libcore:ns-find prelude:%char-macro-ns% (libcore:vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(libcore:intern :prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (libcore:intern prelude:%char-macro-ns% (libcore:vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
