;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern core "boundp" mu:boundp)

(mu:intern core "fboundp"
  (:lambda (symbol)
    (:if (core:boundp symbol)
         (core:functionp (mu:symbol-value symbol))
         ())))

(mu:intern core "uninternedp"
   (:lambda (sym)
      (core:null (core:symbol-namespace sym))))

(mu:intern core "keywordp"
   (:lambda (t)
      (mu:eq (core:type-of t) 'keyword)))

(mu:intern core "symbolp"
   (:lambda (t)
      (core:%orf (core:%orf (core:keywordp t) (mu:eq () t))
                (mu:eq 'symbol (core:type-of t)))))

;;;
;;; symbol components
;;;
(mu:intern core "symbol-name" mu:symbol-name)

(mu:intern core "symbol-namespace"
   (:lambda (sym)
      (mu:symbol-namespace sym)))

(mu:intern core "symbol-value" mu:symbol-value)

;;;
;;; utilities
;;;
(mu:intern core "make-keyword"
   (:lambda (name)
      (:if (core:stringp name)
           (:if (mu:less-than (mu:vector-len name) 8)
                (mu:intern (mu:find-namespace "keyword") name ())
                (core:raise name 'core:make-keyword "not a legal name"))
           (core:raise name 'core:make-keyword "not a legal name"))))
            
(mu:intern core "%gensym-counter"
   (mu:open :string :bidir "0"))

(mu:intern core "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (core:1+ counter) () core:%gensym-counter)
          (mu:make-symbol (core:format () "g~X" `(,counter))))
       (mu:read core:%gensym-counter () ()))))

(mu:intern core "genkeyword"
   (:lambda (prefix)
      (:if (core:%orf (core:null prefix) (core:charp prefix))
           ((:lambda (counter)
              (mu:write (core:1+ counter) () core:%gensym-counter)
              (core:make-keyword
               (:if (core:charp prefix)
                    (core:format () "~A~X" `(,prefix ,counter))
                    (core:format () "<~X>" `(,counter)))))
            (mu:read core:%gensym-counter () ()))
           (core:raise name 'core:genkeyword "not a legal prefix"))))
