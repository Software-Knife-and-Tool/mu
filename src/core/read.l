;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; reader
;;;
(mu:intern core "%eof%" (mu:make-symbol "eof"))
(mu:intern core "%reader-stream%" (mu:open :string :output ""))
(mu:intern core "%reader-current-ns%" (mu:open :string :bidir "core"))

(mu:intern core "%reader-get-ns"
   (:lambda ()
     ((:lambda (ns)
        (mu:write ns () core:%reader-current-ns%)
        (mu:find-namespace ns))
      (mu:get-string core:%reader-current-ns%))))

(mu:intern core "%reader-set-ns"
   (:lambda (ns)
      (mu:get-string core:%reader-current-ns%)
      (mu:write ns () core:%reader-current-ns%)))

;;;
;;; readtable
;;;
(mu:intern core "%read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table) (mu:cdr (core:%assq ch read-table)))
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

(mu:intern core "%read-number"
  (:lambda (atom)
    ((:lambda (fx)
        (:if fx
             fx
             ((:lambda (fl)
                (:if fl
                     fl
                     ()))
             (core:parse-float atom))))
      (core:parse-integer atom 10))))

;;;
;;; atom reader
;;;
(mu:intern core "%read-atom"
  (:lambda (ch stream)
    (mu:write-char ch core:%reader-stream%)
    (mu:fix
     (:lambda (loop)
       (:if (core:%consp loop)
            loop
            ((:lambda (ch)
               (:if (mu:eq core:%eof% ch)
                    loop
                    (:if (mu:eq :const (core:%read-char-syntax ch))
                         (core:%prog2
                            (mu:write-char ch core:%reader-stream%)
                            (core:null loop))
                         (core:%prog2
                            (mu:unread-char ch stream)
                            loop))))
             (mu:read-char stream () core:%eof%))))
     ())
    ((:lambda (token)
       ((:lambda (number)
          (:if number
               number
               (core:%symbol-macro-expand (core:%read-symbol token))))
        (core:%read-number token)))
     (mu:get-string core:%reader-stream%))))

;;;
;;; parser
;;;
(mu:intern core "%read-dispatch"
  (:lambda (ch stream)
    ((:lambda (dispatch-table)
       (mu:apply
        (mu:symbol-value (mu:cdr (core:%assq (core:%read-char-syntax ch) dispatch-table)))
        (core:%list2 ch stream)))
     '((:const   . core:%read-atom)
       (:escape  . core:%read-atom)
       (:macro   . core:%read-macro)
       (:tmacro  . core:%read-macro)
       (:mescape . core:%read-atom)))))

;;;
;;; consume whitespace and comments
;;;
(mu:intern core "%read-consume-ws"
   (:lambda (stream)
     (mu:fix
      (:lambda (loop)
        (:if (core:logor (core:%streamp loop) (core:%charp loop))
             loop
             ((:lambda (ch)
                (:if (core:null ch)
                     stream
                     (:if (mu:eq :wspace (core:%read-char-syntax ch))
                          (core:null loop)
                          (:if (mu:eq ch #\#)
                               ((:lambda (ch)
                                  (:if (mu:eq ch #\|)
                                       (core:%prog2
                                          (core:%read-sharp-comment ch stream)
                                          (core:null loop))
                                       #\#))
                                (core:%peek-char stream () ()))
                               (:if (mu:eq ch #\;)
                                    (core:%prog2
                                       (core:%read-line-comment ch stream)
                                       (core:null loop))
                                    ch)))))
              (mu:read-char stream () ()))))
      ())))

;;;
;;; recursive reader
;;;
(mu:intern core "%read"
  (:lambda (stream)
    ((:lambda (ch)
       (:if (core:%streamp ch)
            core:%eof%
            ((:lambda (macro)
               (:if macro
                    (core:%apply (mu:car macro) (core:%list2 stream ch))
                    (core:%read-dispatch ch stream)))
             (core:get-macro-character ch))))
     (core:%read-consume-ws stream))))
