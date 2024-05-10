;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(core:intern prelude "boundp" core:boundp)

(core:intern prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (core:symbol-value symbol))
         ())))

(core:intern prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(core:intern prelude "keywordp"
   (:lambda (t)
      (core:eq (prelude:type-of t) 'keyword)))

(core:intern prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (core:eq () t))
                (core:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(core:intern prelude "symbol-name" core:symbol-name)

(core:intern prelude "symbol-ns"
   (:lambda (sym)
      (core:symbol-ns sym)))

(core:intern prelude "symbol-value" core:symbol-value)


;;;
;;; utilities
;;;
(core:intern prelude "make-keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (core:less-than (core:vector-len name) 8)
                (core:intern (core:find-ns "keyword") name ())
                (prelude:raise name 'prelude:make-keyword "not a legal name"))
           (prelude:raise name 'prelude:make-keyword "not a legal name"))))
            
(core:intern prelude "%gensym-counter"
   (core:open :string :bidir "0"))

(core:intern prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (core:write (prelude:1+ counter) () prelude:%gensym-counter)
          (core:make-symbol (prelude:format () "g~X" `(,counter))))
       (core:read prelude:%gensym-counter () ()))))

(core:intern prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (core:write (prelude:1+ counter) () prelude:%gensym-counter)
              (prelude:make-keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (core:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
