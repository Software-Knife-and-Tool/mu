;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(mu:intern prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(mu:intern prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (mu:eq ch #\#)
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

(mu:intern prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (mu:make-vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(mu:intern prelude "%read-byte-bits"
   (:lambda (stream)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (mu:cons 0 `(,(mu:difference 8 nbits) ,@(mu:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (mu:eq :tmacro syntax) (mu:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (mu:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (mu:cons 0 `(,(mu:difference 8 nbits) ,@(mu:ash byte nbits)))))
                                 (:if (prelude:%orf (mu:eq #\0 ch) (mu:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(mu:logor
                                           (:if (mu:eq #\1 ch) 1 0)
                                           (mu:product byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (mu:car loop)
          (mu:cdr loop)))
        '(8 . 0)))))
             
(mu:intern prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (mu:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (mu:make-vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(mu:sum length (mu:product 8 (mu:length loop))))))
                       (mu:car byte-descriptor)
                       (mu:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(mu:intern prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (mu:eval (prelude:compile (prelude:read stream () ())))))

(mu:intern prelude "%read-sharp-colon"
   (:lambda (ch stream)
     ((:lambda (ch)
        (:if ch
             ((:lambda (symbol)
                (:if (mu:eq :symbol (mu:type-of symbol))
                     (mu:make-symbol (mu:symbol-name symbol))
                     (prelude:raise symbol 'prelude:%read-sharp-colon "not a symbol")))
              (prelude:%read-atom ch stream))              
             (prelude:raise () 'prelude:%read-sharp-colon "early eof")))
     (prelude:read-char stream () ()))))

(mu:intern prelude "%read-sharp-number"
  (:lambda (base stream)
    (mu:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (mu:eq base #\x)
                          (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 16)
                          (:if (mu:eq base #\b)
                               (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 2)
                               (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (mu:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (mu:eq base #\x)
                                     (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 16)
                                     (:if (mu:eq base #\b)
                                          (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (mu:get-string prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(mu:intern prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (mu:apply
        (mu:symbol-value (mu:cdr (prelude:%assq ch sharp-table)))
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
(mu:intern prelude "%read-list-eol%" (mu:make-symbol "eol"))

(mu:intern prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(mu:intern prelude "%read-list"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (mu:eq el prelude:%read-list-eol%)
                 loop
                 (:if (mu:eq :symbol (mu:type-of el))
                      (:if (mu:eq (mu:symbol-name el) ".")
                           ((:lambda (dotted)
                              (:if (mu:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr mu:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(mu:intern prelude "%read-string"
   (:lambda (ch stream)
     (mu:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (mu:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(mu:intern prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(mu:intern prelude "%read-line-comment"
    (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(mu:intern prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (mu:apply
            (mu:symbol-value (mu:cdr (prelude:%assq ch macro-table)))
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
(mu:intern prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (mu:find prelude:%char-macro-ns% (mu:make-vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(mu:intern prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (mu:intern prelude:%char-macro-ns% (mu:make-vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
