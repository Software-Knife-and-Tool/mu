;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; reader
;;;
(lib:intern prelude "%eof%" (lib:make-symbol "eof"))
(lib:intern prelude "%reader-stream%" (lib:open :string :output ""))
(lib:intern prelude "%reader-current-ns%" (lib:open :string :bidir "prelude"))

(lib:intern prelude "%in-namespace"
   (:lambda (ns)
      ((:lambda ()
          (lib:make-ns ns)
          (prelude:%reader-set-ns ns)
          ns))))

(lib:intern prelude "%reader-get-ns"
   (:lambda ()
     ((:lambda (ns)
        (lib:write ns () prelude:%reader-current-ns%)
        (lib:find-ns ns))
      (prelude:get-output-stream-string prelude:%reader-current-ns%))))

(lib:intern prelude "%reader-set-ns"
   (:lambda (ns)
      (prelude:get-output-stream-string prelude:%reader-current-ns%)
      (lib:write ns () prelude:%reader-current-ns%)))

;;;
;;; readtable
;;;
(lib:intern prelude "%read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table) (lib:cdr (prelude:%assoc ch read-table)))
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

(lib:intern prelude "%read-number"
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
(lib:intern prelude "%read-atom"
  (:lambda (ch stream)
    (prelude:write-char ch prelude:%reader-stream%)
    (lib:fix
     (:lambda (loop)
       (:if (prelude:consp loop)
            loop
            ((:lambda (ch)
               (:if (lib:eq prelude:%eof% ch)
                    loop
                    ((:lambda (syntax-type)
                       (:if (lib:eq :const syntax-type)
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
(lib:intern prelude "%read-dispatch"
  (:lambda (ch stream)
    ((:lambda (dispatch-table)
       (lib:apply
        (lib:symbol-value (lib:cdr (prelude:%assoc (prelude:%read-char-syntax ch) dispatch-table)))
        `(,ch ,stream)))
     '((:const   . prelude:%read-atom)
       (:escape  . prelude:%read-atom)
       (:macro   . prelude:%read-macro)
       (:tmacro  . prelude:%read-macro)
       (:mescape . prelude:%read-atom)))))

;;;
;;; consume whitespace and comments
;;;
(lib:intern prelude "%read-consume-ws"
   (:lambda (stream)
    (lib:fix
     (:lambda (loop)
       (:if (prelude:%orf (prelude:streamp loop) (prelude:charp loop))
            loop
            ((:lambda (ch)
               (:if (prelude:null ch)
                    stream
                    (:if (lib:eq :wspace (prelude:%read-char-syntax ch))
                         (prelude:null loop)
                         (:if (lib:eq ch #\#)
                              ((:lambda (ch)
                                 (:if (lib:eq ch #\|)
                                      ((:lambda ()
                                          (prelude:%read-sharp-comment ch stream)
                                          (prelude:null loop)))
                                      #\#))
                                (prelude:peek-char stream () ()))
                              (:if (lib:eq ch #\;)
                                   ((:lambda ()
                                       (prelude:%read-line-comment ch stream)
                                       (prelude:null loop)))
                                   ch)))))
             (prelude:read-char stream () ()))))
     ())))

;;;
;;; recursive reader
;;;
(lib:intern prelude "%read"
  (:lambda (stream)
    ((:lambda (ch)
       (:if (prelude:null ch)
            prelude:%eof%
            ((:lambda (macro)
               (:if macro
                    (prelude:apply (lib:car macro) `(,stream ,ch))
                    (prelude:%read-dispatch ch stream)))
             (prelude:get-macro-character ch))))
     (prelude:%read-consume-ws stream))))
