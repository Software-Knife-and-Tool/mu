;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote
;;;
(mu:intern mu:%null-ns% "%quasi%" ())

(mu:intern core "%read-quasi"
   (:lambda (char stream)
     `(mu:append ,(core:%quasi-compile (core:%quasi-parse stream) ()))))

(mu:intern core "%quasi-map-func"
   (:lambda (key map)
     (mu:cdr (core:%assq key map))))

;;; fix this for comments
(mu:intern core "%quasi-kill-ws"
   (:lambda (stream)
     (mu:fix
      (:lambda (loop)
        (:if (core:%or (core:streamp loop) (core:charp loop))
             loop
             ((:lambda (ch)
                (:if (core:null ch)
                     stream
                     (:if (mu:eq :wspace (core:%read-char-syntax ch))
                          (core:null loop)
                          (mu:unread-char ch stream))))
                 (mu:read-char stream () ()))))
      ())))

(mu:intern core "%quasi-read-syntax"
   (:lambda (stream)
     (core:%quasi-kill-ws stream)
     ((:lambda (ch syntax-map)
        (:if (core:null ch)
             ()
             ((:lambda (type-function)
                (:if type-function
                     (mu:apply type-function ())
                     (core:%prog2
                        (mu:unread-char ch stream)
                        :atom)))
                (core:%quasi-map-func ch syntax-map))))
      (mu:read-char stream () ())
      `(,(mu:cons #\( (:lambda () :list))
         ,(mu:cons #\) (:lambda () :list-))
         ,(mu:cons #\, (:lambda ()
                         ((:lambda (ch)
                            (:if (core:null ch)
                                 (core:error stream "read: -early end of file~%" ())
                                 (:if (mu:eq #\@ ch)
                                      :comma@
                                      (core:%prog2 (mu:unread-char ch stream) :comma))))
                          (mu:read-char stream () ()))))
         ,(mu:cons #\` (:lambda () :quasi))))))

(mu:intern core "%quasi-append-args"
   (:lambda (form)
     (:if form
          `(mu:cons ,(mu:car form) ,(core:%quasi-append-args (mu:cdr form)))
          ())))
     
(mu:intern core "%quasi-compile"
   (:lambda (expr recur)
     ((:lambda (syntax-map)
        ((:lambda (type-function)
            (mu:apply type-function `(,(mu:cdr expr))))
         (core:%quasi-map-func (mu:car expr) syntax-map)))
     `(,(mu:cons :basic (:lambda (form) (core:%quote (core:%list form))))
        ,(mu:cons :comma (:lambda (form) `(mu:cons ,form ())))
        ,(mu:cons :comma@ core:identity)
        ,(mu:cons :list
            (:lambda (expr)
              (:if expr
                   ((:lambda (compiled-list)
                      (:if recur
                           `(mu:cons (mu:append ,(core:%quasi-append-args compiled-list)) ())
                           (core:%quasi-append-args compiled-list)))
                    (core:%mapcar (:lambda (expr) (core:%quasi-compile expr :t)) expr))
                   `(mu:cons () ()))))))))

(mu:intern core "%quasi-parse-list"
   (:lambda (stream)
     (mu:fix
      (:lambda (list)
        ((:lambda (syntax quasi-map)
           (:if (core:null syntax)
                (core:error stream "read: --early end of file~%" ())
                ((:lambda (map-function)
                   ((:lambda (el)
                      (:if (mu:eq :list- syntax)
                           list
                           (mu:append (core:%list2 list (mu:cons el ())))))
                    (mu:apply map-function ())))
                 (core:%quasi-map-func syntax quasi-map))))
         (core:%quasi-read-syntax stream)
         `(,(mu:cons :atom (:lambda () (mu:cons :basic (core:read stream () ()))))
            ,(mu:cons :comma (:lambda () (mu:cons :comma (core:read stream () ()))))
            ,(mu:cons :comma@ (:lambda () (mu:cons :comma@ (core:read stream () ()))))
            ,(mu:cons :list (:lambda () (mu:cons :list (core:%quasi-parse-list stream))))
            ,(mu:cons :list- (:lambda () list))
            ,(mu:cons :quasi (:lambda () (mu:cons :quasi (core:%quasi-parse stream)))))))
      ())))

(mu:intern core "%quasi-parse"
   (:lambda (stream)
     ((:lambda (syntax syntax-map)
        (:if (core:null syntax)
             (core:error stream "read: ---early end of file~%" ())
             (mu:apply (core:%quasi-map-func syntax syntax-map) ())))
      (core:%quasi-read-syntax stream)
      `(,(mu:cons :atom (:lambda () (mu:cons :basic (core:read stream () ()))))
         ,(mu:cons :comma (:lambda () (mu:cons :comma (core:read stream () ()))))
         ,(mu:cons :comma@ (:lambda () (core:error () "read: `,@ at top level~%" ())))  
         ,(mu:cons :list (:lambda ()
                           ((:lambda (form)
                              (:if form
                                   form
                                   (mu:cons :basic ())))
                            (mu:cons :list (core:%quasi-parse-list stream)))))
         ,(mu:cons :list- (:lambda () (core:error () "read: ) at top level~%" ())))
         ,(mu:cons :quasi (:lambda() (mu:cons :basic (core:read stream () ()))))))))
