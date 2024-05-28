;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(crux:intern prelude "boundp" crux:boundp)

(crux:intern prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (crux:symbol-value symbol))
         ())))

(crux:intern prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(crux:intern prelude "keywordp"
   (:lambda (t)
      (crux:eq (prelude:type-of t) 'keyword)))

(crux:intern prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (crux:eq () t))
                (crux:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(crux:intern prelude "symbol-name" crux:symbol-name)

(crux:intern prelude "symbol-ns"
   (:lambda (sym)
      (crux:symbol-ns sym)))

(crux:intern prelude "symbol-value" crux:symbol-value)


;;;
;;; utilities
;;;
(crux:intern prelude "make-keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (crux:less-than (crux:vector-len name) 8)
                (crux:intern (crux:find-ns "keyword") name ())
                (prelude:raise name 'prelude:make-keyword "not a legal name"))
           (prelude:raise name 'prelude:make-keyword "not a legal name"))))
            
(crux:intern prelude "%gensym-counter"
   (crux:open :string :bidir "0"))

(crux:intern prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (crux:write (prelude:1+ counter) () prelude:%gensym-counter)
          (crux:make-symbol (prelude:format () "g~X" `(,counter))))
       (crux:read prelude:%gensym-counter () ()))))

(crux:intern prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (crux:write (prelude:1+ counter) () prelude:%gensym-counter)
              (prelude:make-keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (crux:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
