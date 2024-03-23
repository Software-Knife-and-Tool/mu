;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(core:intern :prelude "boundp" core:boundp)

(core:intern :prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (core:sy-val symbol))
         ())))

(core:intern :prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(core:intern :prelude "keywordp"
   (:lambda (t)
      (core:eq (prelude:type-of t) 'keyword)))

(core:intern :prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (core:eq () t))
                (core:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(core:intern :prelude "symbol-name" core:sy-name)

(core:intern :prelude "symbol-ns"
   (:lambda (sym)
      (core:sy-ns sym)))

(core:intern :prelude "symbol-value" core:sy-val)

(core:intern :prelude "keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (core:fx-lt (core:sv-len name) 8)
                (core:keyword name)
                (prelude:%make-type "keyword" `(,(core:cons :name name))))
           (prelude:raise name 'prelude:keyword "not a legal name"))))

;;;
;;; utilities
;;;
(core:intern :prelude "%gensym-counter"
   (core:open :string :bidir "0"))

(core:intern :prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (core:write (prelude:1+ counter) () prelude:%gensym-counter)
          (core:symbol (prelude:format () "g~X" `(,counter))))
       (core:read prelude:%gensym-counter () ()))))

(core:intern :prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (core:write (prelude:1+ counter) () prelude:%gensym-counter)
              (core:keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (core:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
