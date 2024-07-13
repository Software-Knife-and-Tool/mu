;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; reader
;;;
(mu:intern prelude "%eof%" (mu:make-symbol "eof"))
(mu:intern prelude "%reader-stream%" (mu:open :string :output ""))
(mu:intern prelude "%reader-current-ns%" (mu:open :string :bidir "prelude"))

(mu:intern prelude "%in-namespace"
   (:lambda (ns)
      ((:lambda ()
          (mu:make-namespace ns)
          (prelude:%reader-set-ns ns)
          ns))))

(mu:intern prelude "%reader-get-ns"
   (:lambda ()
     ((:lambda (ns)
        (mu:write ns () prelude:%reader-current-ns%)
        (mu:find-namespace ns))
      (prelude:get-output-stream-string prelude:%reader-current-ns%))))

(mu:intern prelude "%reader-set-ns"
   (:lambda (ns)
      (prelude:get-output-stream-string prelude:%reader-current-ns%)
      (mu:write ns () prelude:%reader-current-ns%)))

;;;
;;; readtable
;;;
(mu:intern prelude "%read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table) (mu:cdr (prelude:%assq ch read-table)))
      '((#\return . :wspace) (#\linefeed . :wspace) (#\page . :wspace)
        (#\tab . :wspace) (#\space . :wspace)
        (#\0 . :const)  (#\1 . :const)  (#\2 . :const)  (#\3 . :const)
        (#\4 . :const)  (#\5 . :const)  (#\6 . :const)  (#\7 . :const)
        (#\8 . :const)  (#\9 . :const)  (#\: . :const)  (#\< . :const)
        (#\> . :const)  (#\= . :const)  (#\? . :const)  (#\! . :const)
        (#\@ . :const)  (#\; . :tmacro) (#\" . :tmacro) (#\# . :macro)
        (#\' . :tmacro) (#\( . :tmacro) (#\) . :tmacro) (#\` . :tmacro)
        (#\, . :tmacro) (#\\ . :escape) (#\| . :mescape) (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\v . :const)
        (#\w . :const)  (#\x . :const)  (#\y . :const)  (#\z . :const)
        (#\[ . :const)  (#\] . :const)  (#\$ . :const)  (#\* . :const)
        (#\{ . :const)  (#\} . :const)  (#\+ . :const)  (#\- . :const)
        (#\/ . :const)  (#\~ . :const)  (#\. . :const)  (#\% . :const)
        (#\& . :const)  (#\^ . :const)  (#\_ . :const)  (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\u . :const)
        (#\v . :const)  (#\w . :const)  (#\x . :const)  (#\y . :const)
        (#\z . :const)))))

(mu:intern prelude "%read-number"
  (:lambda (atom)
    ((:lambda (fx)
        (:if fx
             fx
             ((:lambda (fl)
                (:if fl
                     fl
                     ()))
             (prelude:parse-float atom))))
      (prelude:parse-integer atom 10))))

;;;
;;; atom reader
;;;
(mu:intern prelude "%read-atom"
  (:lambda (ch stream)
    (prelude:write-char ch prelude:%reader-stream%)
    (mu:fix
     (:lambda (loop)
       (:if (prelude:consp loop)
            loop
            ((:lambda (ch)
               (:if (mu:eq prelude:%eof% ch)
                    loop
                    ((:lambda (syntax-type)
                       (:if (mu:eq :const syntax-type)
                            ((:lambda ()
                               (prelude:write-char ch prelude:%reader-stream%)
                               (prelude:null loop)))
                            ((:lambda ()
                               (prelude:unread-char ch stream)
                               loop))))
                     (prelude:%read-char-syntax ch))))
             (prelude:read-char stream () prelude:%eof%))))
     ())
    ((:lambda (token)
       ((:lambda (number)
          (:if number
               number
               (prelude:%symbol-macro-expand (prelude:%read-symbol token))))
        (prelude:%read-number token)))
     (prelude:get-output-stream-string prelude:%reader-stream%))))

;;;
;;; parser
;;;
(mu:intern prelude "%read-dispatch"
  (:lambda (ch stream)
    ((:lambda (dispatch-table)
       (mu:apply
        (mu:symbol-value (mu:cdr (prelude:%assq (prelude:%read-char-syntax ch) dispatch-table)))
        `(,ch ,stream)))
     '((:const   . prelude:%read-atom)
       (:escape  . prelude:%read-atom)
       (:macro   . prelude:%read-macro)
       (:tmacro  . prelude:%read-macro)
       (:mescape . prelude:%read-atom)))))

;;;
;;; consume whitespace and comments
;;;
(mu:intern prelude "%read-consume-ws"
   (:lambda (stream)
    (mu:fix
     (:lambda (loop)
       (:if (prelude:%orf (prelude:streamp loop) (prelude:charp loop))
            loop
            ((:lambda (ch)
               (:if (prelude:null ch)
                    stream
                    (:if (mu:eq :wspace (prelude:%read-char-syntax ch))
                         (prelude:null loop)
                         (:if (mu:eq ch #\#)
                              ((:lambda (ch)
                                 (:if (mu:eq ch #\|)
                                      ((:lambda ()
                                          (prelude:%read-sharp-comment ch stream)
                                          (prelude:null loop)))
                                      #\#))
                                (prelude:peek-char stream () ()))
                              (:if (mu:eq ch #\;)
                                   ((:lambda ()
                                       (prelude:%read-line-comment ch stream)
                                       (prelude:null loop)))
                                   ch)))))
             (prelude:read-char stream () ()))))
     ())))

;;;
;;; recursive reader
;;;
(mu:intern prelude "%read"
  (:lambda (stream)
    ((:lambda (ch)
       (:if (prelude:null ch)
            prelude:%eof%
            ((:lambda (macro)
               (:if macro
                    (prelude:apply (mu:car macro) `(,stream ,ch))
                    (prelude:%read-dispatch ch stream)))
             (prelude:get-macro-character ch))))
     (prelude:%read-consume-ws stream))))
