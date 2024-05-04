;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(lib:intern prelude "boundp" lib:boundp)

(lib:intern prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (lib:symbol-value symbol))
         ())))

(lib:intern prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(lib:intern prelude "keywordp"
   (:lambda (t)
      (lib:eq (prelude:type-of t) 'keyword)))

(lib:intern prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (lib:eq () t))
                (lib:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(lib:intern prelude "symbol-name" lib:symbol-name)

(lib:intern prelude "symbol-ns"
   (:lambda (sym)
      (lib:symbol-ns sym)))

(lib:intern prelude "symbol-value" lib:symbol-value)


;;;
;;; utilities
;;;
(lib:intern prelude "make-keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (lib:fx-lt (lib:sv-len name) 8)
                (lib:intern (lib:find-ns "keyword") name ())
                (prelude:raise name 'prelude:make-keyword "not a legal name"))
           (prelude:raise name 'prelude:make-keyword "not a legal name"))))
            
(lib:intern prelude "%gensym-counter"
   (lib:open :string :bidir "0"))

(lib:intern prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (lib:write (prelude:1+ counter) () prelude:%gensym-counter)
          (lib:make-symbol (prelude:format () "g~X" `(,counter))))
       (lib:read prelude:%gensym-counter () ()))))

(lib:intern prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (lib:write (prelude:1+ counter) () prelude:%gensym-counter)
              (prelude:make-keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (lib:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
