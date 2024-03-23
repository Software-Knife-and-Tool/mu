;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(core:intern :prelude "%read-keywordp"
  (:lambda (name)
      (core:eq #\: (prelude:schar name 0))))

(core:intern :prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (core:eq #\: (prelude:schar name (core:fx-add 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(core:intern :prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (core:keyword (prelude:substr name 0 (core:fx-sub colon 1)))))
     (prelude:string-position #\: name))))

(core:intern :prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                          (:if (core:eq #\: (prelude:schar name (core:fx-add 1 colon)))
                               (core:fx-add 2 colon)
                               (core:fx-add 1 colon))
                          (:if (core:eq #\: (prelude:schar name (core:fx-add 1 colon)))
                               (core:fx-add colon (core:fx-sub (core:sv-len name) colon))
                               (core:fx-add colon (core:fx-sub (core:sv-len name) (core:fx-add 1 colon)))))))
     (prelude:string-position #\: name))))

(core:intern :prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  (core:untern () name)
                  (core:untern ns name)))
           (prelude:%read-symbol-ns symbol)
           (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(core:intern :prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (core:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(core:intern :prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (core:eq :symbol (core:type-of symbol))
           (core:ns-find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(core:intern :prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (core:eq :symbol (core:type-of symbol))
           (core:intern prelude:%symbol-macro-ns% (core:sy-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
