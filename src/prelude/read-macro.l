;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(crux:intern prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(crux:intern prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (crux:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (crux:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (crux:eq ch #\#)
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

(crux:intern prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (crux:make-vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(crux:intern prelude "%read-byte-bits"
   (:lambda (stream)
     (crux:cdr
      (crux:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (crux:cons 0 `(,(crux:difference 8 nbits) ,@(crux:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (crux:eq :tmacro syntax) (crux:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (crux:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (crux:cons 0 `(,(crux:difference 8 nbits) ,@(crux:ash byte nbits)))))
                                 (:if (prelude:%orf (crux:eq #\0 ch) (crux:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(crux:logor
                                           (:if (crux:eq #\1 ch) 1 0)
                                           (crux:product byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (crux:car loop)
          (crux:cdr loop)))
        '(8 . 0)))))
             
(crux:intern prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (crux:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (crux:make-vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(crux:sum length (crux:product 8 (crux:length loop))))))
                       (crux:car byte-descriptor)
                       (crux:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(crux:intern prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (crux:eval (prelude:compile (prelude:read stream () ())))))

(crux:intern prelude "%read-sharp-colon"
   (:lambda (ch stream)
     ((:lambda (ch)
        (:if ch
             ((:lambda (symbol)
                (:if (crux:eq :symbol (crux:type-of symbol))
                     (crux:make-symbol (crux:symbol-name symbol))
                     (prelude:raise symbol 'prelude:%read-sharp-colon "not a symbol")))
              (prelude:%read-atom ch stream))              
             (prelude:raise () 'prelude:%read-sharp-colon "early eof")))
     (prelude:read-char stream () ()))))

(crux:intern prelude "%read-sharp-number"
  (:lambda (base stream)
    (crux:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (crux:eq base #\x)
                          (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 16)
                          (:if (crux:eq base #\b)
                               (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 2)
                               (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (crux:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (crux:eq base #\x)
                                     (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 16)
                                     (:if (crux:eq base #\b)
                                          (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (crux:get-string prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(crux:intern prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (crux:apply
        (crux:symbol-value (crux:cdr (prelude:%assq ch sharp-table)))
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
       (#\: . prelude:%read-sharp-colon)))))

;;;
;;; list reader
;;;
(crux:intern prelude "%read-list-eol%" (crux:make-symbol "eol"))

(crux:intern prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(crux:intern prelude "%read-list"
   (:lambda (ch stream)
      (crux:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (crux:eq el prelude:%read-list-eol%)
                 loop
                 (:if (crux:eq :symbol (crux:type-of el))
                      (:if (crux:eq (crux:symbol-name el) ".")
                           ((:lambda (dotted)
                              (:if (crux:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr crux:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(crux:intern prelude "%read-string"
   (:lambda (ch stream)
     (crux:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (crux:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(crux:intern prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(crux:intern prelude "%read-line-comment"
    (:lambda (ch stream)
      (crux:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (crux:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(crux:intern prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (crux:apply
            (crux:symbol-value (crux:cdr (prelude:%assq ch macro-table)))
            `(,ch ,stream)))
       '((#\" . prelude:%read-string)
         (#\# . prelude:%read-sharp)
         (#\' . prelude:%read-quote)
         (#\` . prelude:%read-quasi)
         (#\( . prelude:%read-list)
         (#\) . prelude:%read-list-eol)
         (#\; . prelude:%read-line-comment)))))

;;;
;;; get/set-macro-character
;;;
(crux:intern prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (crux:find prelude:%char-macro-ns% (crux:make-vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(crux:intern prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (crux:intern prelude:%char-macro-ns% (crux:make-vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
