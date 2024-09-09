;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(mu:intern core "%read-sharp-char"
   (:lambda (ch stream)
      (mu:read-char stream () ())))

(mu:intern core "%read-sharp-comment"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (core:%streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (core:null ch)
                                 (core:raise stream 'core:%read-sharp-comment "unexpected end of file")
                                 (:if (mu:eq ch #\#)
                                      stream
                                      (core:null loop))))
                          (mu:read-char stream () ())))
                       (:if (core:null ch)
                            (core:raise stream 'core:%read-sharp-comment "unexpected end of file")
                            (mu:read-char stream () ())))
                      (core:null loop)))
               (:if (core:null ch)
                    (core:raise stream 'core:%read-sharp-comment "unexpected end of file")
                    (mu:read-char stream () ())))))
       ())))

(mu:intern core "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (core:%keywordp type)
               (mu:make-vector type (core:%read-list #\( stream))
               (core:raise type 'core:%read-sharp-vector "not a type keyword")))
          (core:%read stream))))

(mu:intern core "%read-byte-bits"
   (:lambda (stream)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (mu:eq 0 nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (core:null ch)
                         (mu:cons 0 `(,(mu:sub 8 nbits) ,@(mu:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (core:%orf (mu:eq :tmacro syntax) (mu:eq :wspace syntax))
                                 (core:%arg2
                                    (:if (mu:eq :tmacro syntax)
                                         (mu:unread-char ch stream)
                                         ())
                                    (mu:cons 0 `(,(mu:sub 8 nbits) ,@(mu:ash byte nbits))))
                                 (:if (core:%orf (mu:eq #\0 ch) (mu:eq #\1 ch))
                                      `(,(mu:sub nbits 1)
                                        ,@(mu:logor
                                           (:if (mu:eq #\1 ch) 1 0)
                                           (mu:mul byte 2)))
                                      (core:raise ch 'core:%read-sharp-bit-vector "not a bit designator"))))
                          (core:%read-char-syntax ch))))
                  (mu:read-char stream () ()))))
          (mu:car loop)
          (mu:cdr loop)))
        '(8 . 0)))))
             
(mu:intern core "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (mu:fix
       (:lambda (loop)
         (:if (core:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (core:%consp byte-descriptor)
                      ((:lambda (length byte)
                         (core:%make-vector
                          (mu:make-vector :byte (core:%reverse `(,byte ,@loop)))
                          ()
                          `(,(mu:add length (mu:mul 8 (mu:length loop))))))
                       (mu:car byte-descriptor)
                       (mu:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (core:%read-byte-bits stream))))
       ())))

(mu:intern core "%read-sharp-dot"
   (:lambda (ch stream)
      (mu:eval (core:compile (core:read stream () ())))))

(mu:intern core "%read-sharp-colon"
   (:lambda (ch stream)
     ((:lambda (ch)
        (:if ch
             ((:lambda (symbol)
                (:if (mu:eq :symbol (mu:type-of symbol))
                     (mu:make-symbol (mu:symbol-name symbol))
                     (core:raise symbol 'core:%read-sharp-colon "not a symbol")))
              (core:%read-atom ch stream))              
             (core:raise () 'core:%read-sharp-colon "early eof")))
     (mu:read-char stream () ()))))

(mu:intern core "%read-sharp-number"
  (:lambda (base stream)
    (mu:fix
      (:lambda (loop)
        (:if (core:%numberp loop)
             loop
             ((:lambda (ch)
                (:if (core:null ch)
                     (:if (mu:eq base #\x)
                          (core:parse-integer (mu:get-string core:%reader-stream%) 16)
                          (:if (mu:eq base #\b)
                               (core:parse-integer (mu:get-string core:%reader-stream%) 2)
                               (core:parse-integer (mu:get-string core:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (mu:eq syntax-type :const)
                             (core:%arg2
                                (mu:write-char ch core:%reader-stream%)
                                (core:null loop))
                             (core:arg2
                                (mu:unread-char ch stream)
                                (:if (mu:eq base #\x)
                                     (core:parse-integer (mu:get-string core:%reader-stream%) 16)
                                     (:if (mu:eq base #\b)
                                          (core:parse-integer (mu:get-string core:%reader-stream%) 2)
                                          (core:parse-integer (mu:get-string core:%reader-stream%) 10))))))
                     (core:%read-char-syntax ch))))
              (mu:read-char stream () ()))))
      ())))

(mu:intern core "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (mu:apply
        (mu:symbol-value (mu:cdr (core:%assq ch sharp-table)))
        `(,ch ,stream)))
     (mu:read-char stream () ())
     '((#\| . core:%read-sharp-comment)
       (#\( . core:%read-sharp-vector)
       (#\* . core:%read-sharp-bit-vector)
       (#\. . core:%read-sharp-dot)
       (#\\ . core:%read-sharp-char)
       (#\b . core:%read-sharp-number)
       (#\x . core:%read-sharp-number)
       (#\d . core:%read-sharp-number)
       (#\: . core:%read-sharp-colon)))))

;;;
;;; list reader
;;;
(mu:intern core "%read-list-eol%" (mu:make-symbol "eol"))

(mu:intern core "%read-list-eol"
   (:lambda (ch stream)
      core:%read-list-eol%))

(mu:intern core "%read-list"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (mu:eq el core:%read-list-eol%)
                 loop
                 (:if (mu:eq :symbol (mu:type-of el))
                      (:if (mu:eq (mu:symbol-name el) ".")
                           ((:lambda (dotted)
                              (:if (mu:eq (core:%read stream) core:%read-list-eol%)
                                   (core:%arg2
                                      (mu:unread-char #\) stream)
                                      dotted)
                                   (core:raise () 'core:%read-list "malformed dotted list")))
                            (core:%foldr mu:cons (core:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (core:%read stream)))
     ())))

;;;
;;; string reader
;;;
(mu:intern core "%read-string"
   (:lambda (ch stream)
     (mu:fix
      (:lambda (loop)
        (:if (core:stringp loop)
             loop
             ((:lambda (ch)
                (:if (mu:eq ch #\")
                     (mu:get-string core:%reader-stream%)
                     (core:%arg2
                        (mu:write-char ch core:%reader-stream%)
                        (core:null loop))))
              (mu:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(mu:intern core "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(core:%read stream))))

;;;
;;; line comment
;;;
(mu:intern core "%read-line-comment"
    (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (core:%streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\linefeed)
                      stream
                      (core:null loop)))
               (mu:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(mu:intern core "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (mu:apply
            (mu:symbol-value (mu:cdr (core:%assq ch macro-table)))
            `(,ch ,stream)))
       '((#\" . core:%read-string)
         (#\# . core:%read-sharp)
         (#\' . core:%read-quote)
         (#\` . core:%read-quasi)
         (#\( . core:%read-list)
         (#\) . core:%read-list-eol)
         (#\; . core:%read-line-comment)))))

;;;
;;; get/set-macro-character
;;;
(mu:intern core "get-macro-character"
   (:lambda (char)
     (:if (core:%charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (mu:symbol-value macro-symbol)
                   ()))
           (mu:find core:%char-macro-ns% (mu:make-vector :char `(,char))))
          (core:raise char 'core:get-macro-character "not a character"))))

(mu:intern core "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (core:%charp char)
         (:if (core:functionp fn)
              (mu:intern core:%char-macro-ns% (mu:make-vector :char `(char)) `(,fn ,@non-terminating))
              (core:raise fn 'core:set-macro-character "not a function"))
         (core:raise char 'core:set-macro-character "not a character"))
      :t))
