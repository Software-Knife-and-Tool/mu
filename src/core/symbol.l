;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern core "%uninternedp"
   (:lambda (sym)
      (core:null (mu:symbol-namespace sym))))

(mu:intern core "symbolp"
   (:lambda (t)
     (core:logor
      (core:logor (core:keywordp t) (mu:eq () t))
      (mu:eq 'symbol (core:type-of t)))))

;;;
;;; utilities
;;;
(mu:intern core "%make-keyword"
   (:lambda (name)
     (mu:intern (mu:find-namespace "keyword") name ())))
            
(mu:intern core "%gensym-counter%"
   (mu:open :string :bidir "0"))

(mu:intern core "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (mu:add counter 1) () core:%gensym-counter%)
          (mu:make-symbol (core:%format () "g~X" (mu:cons counter ()))))
       (mu:read core:%gensym-counter% () ()))))

(mu:intern core "%genkey"
   (:lambda (prefix)
      (:if (core:logor (core:null prefix) (core:charp prefix))
           ((:lambda (counter)
              (mu:write (mu:add counter 1) () core:%gensym-counter%)
              (core:%make-keyword
               (:if (core:charp prefix)
                    (core:%format () "~A~X" (core:%list2 prefix counter))
                    (core:%format () "<~X>" (mu:cons counter ())))))
            (mu:read core:%gensym-counter% () ()))
           (core:raise name 'core:%genkey "not a legal prefix"))))
