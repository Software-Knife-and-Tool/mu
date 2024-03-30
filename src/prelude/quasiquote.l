;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote
;;;
(libcore:intern () "%quasi%" ())

(libcore:intern :prelude "%list"
   (:lambda (form)
     `(,form)))

(libcore:intern :prelude "%qq-func-map"
   (:lambda (key map)
     (libcore:cdr (prelude:assoc key map))))

(libcore:intern :prelude "%qq-read"
   (:lambda (stream)
     ((:lambda (ch syntax-map)
        (:if (prelude:null ch)
             (prelude:error stream "backquote: early end of file~%" ())
             ((:lambda (type-map)
                (:if (prelude:null type-map)
                     `(:form ,(prelude:%read-dispatch ch stream))
                     (libcore:apply (libcore:cdr type-map) ())))
                (prelude:assoc ch syntax-map))))
        (prelude:%read-consume-ws stream)
        `(,(libcore:cons #\( (:lambda () `(:list ,(prelude:%qq-read-list stream))))
           ,(libcore:cons #\) (:lambda () `(:list-)))
           ,(libcore:cons #\, (:lambda ()
                           ((:lambda (ch)
                             (:if (prelude:null ch)
                                  (prelude:error stream "backquote: early end of file~%" ())
                                  (:if (libcore:eq #\@ ch)
                                       `(:comma@ ,(prelude:read stream () ()))
                                       ((:lambda ()
                                          (prelude:unread-char ch stream)
                                          `(:form ,(prelude:read stream () ())))))))
                            (prelude:read-char stream () ()))))
           ,(libcore:cons #\' (:lambda () `(:qquote ,(prelude:%qq-read stream))))
           ,(libcore:cons #\` (:lambda () `(:quasi ,(prelude:read stream () ()))))))))

(libcore:intern :prelude "%qq-read-list"
   (:lambda (stream)
     ((:lambda (syntax-map)
         (libcore:cdr
          (libcore:fix
           (:lambda (loop)
             ((:lambda (eol list)
                (:if eol
                     loop
                     ((:lambda (syntax)
                        ((:lambda (type expr)
                           (:if (prelude:null syntax)
                                (prelude:error stream "qquote: early end of file~%" ())
                                ((:lambda (type-fn)
                                   (libcore:apply type-fn `(,expr ,list)))
                                 (prelude:%qq-func-map type syntax-map))))
                         (libcore:car syntax)
                         (libcore:cdr syntax)))
                      (prelude:%qq-read stream))))
              (libcore:car loop)
              (libcore:cdr loop)))
           ())))
      `(,(libcore:cons :comma   (:lambda (expr list)
                             (libcore:cons () (libcore:append list (prelude:%list `(:l-comma ,(libcore:car expr)))))))
         ,(libcore:cons :comma@ (:lambda (expr list)
                             (libcore:cons () (libcore:append list (prelude:%list `(:comma@ ,(libcore:car expr)))))))
         ,(libcore:cons :form   (:lambda (expr list)
                             (libcore:cons () (libcore:append list (prelude:%list `(:l-form ,(libcore:car expr)))))))
         ,(libcore:cons :list   (:lambda (expr list)
                             (:if (prelude:null (libcore:car expr))
                                  (libcore:cons () (libcore:append list (prelude:%list `(:l-form ()))))
                                  (:if (prelude:consp (libcore:car expr))
                                       ((:lambda (form)
                                          (libcore:cons () (libcore:append list (prelude:%list `(:l-form ,form)))))
                                        (prelude:mapcar (:lambda (expr) (libcore:nth 1 expr)) (libcore:car expr)))
                                       (libcore:cons () (libcore:append list (prelude:%list `(:l-list ,(libcore:car expr)))))))))
         ,(libcore:cons :list-  (:lambda (expr list)
                             (libcore:cons :t list)))
         ,(libcore:cons :quasi  (:lambda (expr list)
                             (libcore:cons () (libcore:append list `(,(prelude:%qq-parse stream))))))))))

(libcore:intern :prelude "%qq-compile"
   (:lambda (qq-expr)
     ((:lambda (type expr compiler-map)
        ((:lambda (type-fn)
           (:if (prelude:null type-fn)
                (prelude:error type "backquote: unmapped type ~A~%" `(,type))
                (libcore:apply type-fn `(,expr))))
         (prelude:%qq-func-map type compiler-map)))
      (libcore:nth 0 qq-expr)
      (libcore:nth 1 qq-expr)
      `(,(libcore:cons :l-comma  (:lambda (expr) `(libcore:cons ,expr ())))
         ,(libcore:cons :l-list  (:lambda (expr)
                              ((:lambda (loop)
                                 (libcore:apply loop `(,loop ,expr)))
                               (:lambda (loop list)
                                 (:if (prelude:null list)
                                      list
                                      `(libcore:append
                                        ,(prelude:%qq-compile (libcore:car list))
                                        ,(libcore:apply loop `(,loop ,(libcore:cdr list)))))))))
         ,(libcore:cons :l-form  (:lambda (expr)
                              ((:lambda (quote)
                               `(libcore:cons ,quote ()))
                               `(:quote ,expr))))
         ,(libcore:cons :comma@ (:lambda (expr) expr))
         ,(libcore:cons :list    (:lambda (list)
                              ((:lambda (loop)
                                 (libcore:apply loop `(,loop ,list)))
                               (:lambda (loop list)
                                 (:if (prelude:null list)
                                      list
                                      `(libcore:append
                                        ,(prelude:%qq-compile (libcore:car list))
                                        ,(libcore:apply loop `(,loop ,(libcore:cdr list)))))))))
         ,(libcore:cons :form    (:lambda (expr)
                              `(:quote ,expr)))
         ,(libcore:cons :quasi   (:lambda (expr)
                              ((:lambda (type)
                                 (:if (libcore:eq :form type)
                                      (libcore:cdr expr)
                                      (:if (libcore:eq :quote type)
                                           (prelude:%qq-compile expr)
                                           ())))
                               (libcore:car expr))))
         ,(libcore:cons :qquote   (:lambda (expr)
                               `(:quote ,(prelude:%qq-compile expr))))))))

(libcore:intern :prelude "%qq-reader"
   (:lambda (char stream)
     `(%quasi% ,(prelude:%qq-read stream))))