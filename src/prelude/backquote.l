;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote
;;;
(mu:intern :prelude "%list"
   (:lambda (form)
     `(,form)))

(mu:intern :prelude "%qq-func-map"
   (:lambda (key map)
     (mu:cdr (prelude:assoc key map))))

(mu:intern :prelude "%qq-read-list"
   (:lambda (stream)
     ((:lambda (syntax-map)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (eol expansion)
                (:if eol
                     loop
                     ((:lambda (syntax)
                        (prelude:warn syntax "prelude:%qq-read-list")
                        ((:lambda (type expr)
                           (:if (prelude:null syntax)
                                (prelude:error stream "qquote: early end of file~%" ())
                                ((:lambda (type-fn)
                                   (mu:apply type-fn `(,expr ,expansion)))
                                 (prelude:%qq-func-map type syntax-map))))
                         (mu:car syntax)
                         (mu:cdr syntax)))
                      (prelude:%qq-read stream))))
              (mu:car loop)
              (mu:cdr loop)))
           ())))
      `(,(mu:cons :comma   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:comma ,expr))))))
         ,(mu:cons :comma@ (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:comma@ ,expr))))))
         ,(mu:cons :atom   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:atom ,expr))))))
         ,(mu:cons :list   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:atom ,expr))))))
         ,(mu:cons :list-  (:lambda (expr expansion) (mu:cons :t expansion)))
         ,(mu:cons :quasi  (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion `(,(prelude:%qq-parse stream))))))))))

(mu:intern :prelude "%qq-read"
   (:lambda (stream)
     ((:lambda (ch syntax-map)
        (:if (prelude:null ch)
             (prelude:error stream "backquote: early end of file~%" ())
             ((:lambda (type-map)
                (:if (prelude:null type-map)
                     `(:atom ,(prelude:%read-dispatch ch stream))
                     (mu:apply (mu:cdr type-map) ())))
                (prelude:assoc ch syntax-map))))
        (prelude:%read-consume-ws stream)
        `(,(mu:cons #\( (:lambda () `(:list ,(prelude:%qq-read-list stream))))
           ,(mu:cons #\) (:lambda () `(:list-)))
           ,(mu:cons #\, (:lambda ()
                          ((:lambda (ch)
                             (:if (prelude:null ch)
                                  (prelude:error stream "backquote: early end of file~%" ())
                                  (:if (mu:eq #\@ ch)
                                       `(:comma@ ,(prelude:read stream () ()))
                                       ((:lambda ()
                                          (prelude:unread-char ch stream)
                                          `(:form ,(prelude:read stream () ())))))))
                             (prelude:read-char stream () ()))))
           ,(mu:cons #\` (:lambda () `(:atom ,(prelude:read stream () ()))))))))

(mu:intern :prelude "%qq-compile"
   (:lambda (qq-expr)
     (prelude:warn qq-expr "%qq-compile")
     ((:lambda (type expr compiler-map)
        ((:lambda (type-fn)
           (prelude:warn type-fn "type function")
           (:if (prelude:null type-fn)
                (prelude:error type "backquote: unmapped type ~A~%" `(,type))
                (mu:apply type-fn `(,expr))))
         (prelude:%qq-func-map type compiler-map)))
      (mu:nth 0 qq-expr)
      (mu:nth 1 qq-expr)
      `(,(mu:cons :comma (:lambda (expr) `(mu:cons ,expr ())))
         ,(mu:cons :comma@ (:lambda (expr) expr))
         ,(mu:cons :form (:lambda (expr) expr))
         ,(mu:cons :list (:lambda (expr)
                           ((:lambda (loop)
                              (mu:apply loop `(,loop ,expr)))
                            (:lambda (loop list)
                              (:if (prelude:null list)
                                   list
                                   `(mu:append
                                     ,(prelude:%qq-compile (mu:car list))
                                     ,(mu:apply loop `(,loop ,(mu:cdr list)))))))))
         ,(mu:cons :atom (:lambda (expr)
                           (prelude:warn expr ":atom")
                           expr))
         ,(mu:cons :quasi (:lambda (expr)
                            ((:lambda (type)
                               (:if (mu:eq :form type)
                                    (mu:cdr expr)
                                    (:if (mu:eq :quote type)
                                         (prelude:%qq-compile expr)
                                         ())))
                             (mu:car expr))))
         ,(mu:cons :quote (:lambda (expr) qq-expr))))))

(mu:intern :prelude "%qq-reader"
   (:lambda (char stream)
     (prelude:warn (prelude:%qq-compile (prelude:%qq-read stream)) "%qq-reader")))