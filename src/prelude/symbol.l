;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(libcore:intern :prelude "boundp" libcore:boundp)

(libcore:intern :prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (libcore:sy-val symbol))
         ())))

(libcore:intern :prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(libcore:intern :prelude "keywordp"
   (:lambda (t)
      (libcore:eq (prelude:type-of t) 'keyword)))

(libcore:intern :prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (libcore:eq () t))
                (libcore:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(libcore:intern :prelude "symbol-name" libcore:sy-name)

(libcore:intern :prelude "symbol-ns"
   (:lambda (sym)
      (libcore:sy-ns sym)))

(libcore:intern :prelude "symbol-value" libcore:sy-val)

(libcore:intern :prelude "keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (libcore:fx-lt (libcore:sv-len name) 8)
                (libcore:keyword name)
                (prelude:%make-type "keyword" `(,(libcore:cons :name name))))
           (prelude:raise name 'prelude:keyword "not a legal name"))))

;;;
;;; utilities
;;;
(libcore:intern :prelude "%gensym-counter"
   (libcore:open :string :bidir "0"))

(libcore:intern :prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (libcore:write (prelude:1+ counter) () prelude:%gensym-counter)
          (libcore:symbol (prelude:format () "g~X" `(,counter))))
       (libcore:read prelude:%gensym-counter () ()))))

(libcore:intern :prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (libcore:write (prelude:1+ counter) () prelude:%gensym-counter)
              (libcore:keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (libcore:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
