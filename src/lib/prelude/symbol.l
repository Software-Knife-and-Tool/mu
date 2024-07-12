;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern prelude "boundp" mu:boundp)

(mu:intern prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (mu:symbol-value symbol))
         ())))

(mu:intern prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(mu:intern prelude "keywordp"
   (:lambda (t)
      (mu:eq (prelude:type-of t) 'keyword)))

(mu:intern prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (mu:eq () t))
                (mu:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(mu:intern prelude "symbol-name" mu:symbol-name)

(mu:intern prelude "symbol-ns"
   (:lambda (sym)
      (mu:symbol-ns sym)))

(mu:intern prelude "symbol-value" mu:symbol-value)

;;;
;;; utilities
;;;
(mu:intern prelude "make-keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (mu:less-than (mu:vector-len name) 8)
                (mu:intern (mu:find-ns "keyword") name ())
                (prelude:raise name 'prelude:make-keyword "not a legal name"))
           (prelude:raise name 'prelude:make-keyword "not a legal name"))))
            
(mu:intern prelude "%gensym-counter"
   (mu:open :string :bidir "0"))

(mu:intern prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (prelude:1+ counter) () prelude:%gensym-counter)
          (mu:make-symbol (prelude:format () "g~X" `(,counter))))
       (mu:read prelude:%gensym-counter () ()))))

(mu:intern prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (mu:write (prelude:1+ counter) () prelude:%gensym-counter)
              (prelude:make-keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (mu:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
