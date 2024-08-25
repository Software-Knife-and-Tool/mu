;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern common "boundp" mu:boundp)

(mu:intern common "fboundp"
  (:lambda (symbol)
    (:if (common:boundp symbol)
         (common:functionp (mu:symbol-value symbol))
         ())))

(mu:intern common "uninternedp"
   (:lambda (sym)
      (common:null (common:symbol-namespace sym))))

(mu:intern common "keywordp"
   (:lambda (t)
      (mu:eq (common:type-of t) 'keyword)))

(mu:intern common "symbolp"
   (:lambda (t)
      (common:%orf (common:%orf (common:keywordp t) (mu:eq () t))
                (mu:eq 'symbol (common:type-of t)))))

;;;
;;; symbol components
;;;
(mu:intern common "symbol-name" mu:symbol-name)

(mu:intern common "symbol-namespace"
   (:lambda (sym)
      (mu:symbol-namespace sym)))

(mu:intern common "symbol-value" mu:symbol-value)

;;;
;;; utilities
;;;
(mu:intern common "make-keyword"
   (:lambda (name)
      (:if (common:stringp name)
           (:if (mu:less-than (mu:vector-len name) 8)
                (mu:intern (mu:find-namespace "keyword") name ())
                (common:raise name 'common:make-keyword "not a legal name"))
           (common:raise name 'common:make-keyword "not a legal name"))))
            
(mu:intern common "%gensym-counter"
   (mu:open :string :bidir "0"))

(mu:intern common "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (common:1+ counter) () common:%gensym-counter)
          (mu:make-symbol (common:format () "g~X" `(,counter))))
       (mu:read common:%gensym-counter () ()))))

(mu:intern common "genkeyword"
   (:lambda (prefix)
      (:if (common:%orf (common:null prefix) (common:charp prefix))
           ((:lambda (counter)
              (mu:write (common:1+ counter) () common:%gensym-counter)
              (common:make-keyword
               (:if (common:charp prefix)
                    (common:format () "~A~X" `(,prefix ,counter))
                    (common:format () "<~X>" `(,counter)))))
            (mu:read common:%gensym-counter () ()))
           (common:raise name 'common:genkeyword "not a legal prefix"))))
