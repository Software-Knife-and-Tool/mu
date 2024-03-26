;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(libcore:intern :prelude "%read-keywordp"
  (:lambda (name)
      (libcore:eq #\: (prelude:schar name 0))))

(libcore:intern :prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (libcore:eq #\: (prelude:schar name (libcore:fx-add 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(libcore:intern :prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (libcore:keyword (prelude:substr name 0 (libcore:fx-sub colon 1)))))
     (prelude:string-position #\: name))))

(libcore:intern :prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                          (:if (libcore:eq #\: (prelude:schar name (libcore:fx-add 1 colon)))
                               (libcore:fx-add 2 colon)
                               (libcore:fx-add 1 colon))
                          (:if (libcore:eq #\: (prelude:schar name (libcore:fx-add 1 colon)))
                               (libcore:fx-add colon (libcore:fx-sub (libcore:sv-len name) colon))
                               (libcore:fx-add colon (libcore:fx-sub (libcore:sv-len name) (libcore:fx-add 1 colon)))))))
     (prelude:string-position #\: name))))

(libcore:intern :prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  (libcore:untern () name)
                  (libcore:untern ns name)))
           (prelude:%read-symbol-ns symbol)
           (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(libcore:intern :prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (libcore:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(libcore:intern :prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (libcore:eq :symbol (libcore:type-of symbol))
           (libcore:ns-find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(libcore:intern :prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (libcore:eq :symbol (libcore:type-of symbol))
           (libcore:intern prelude:%symbol-macro-ns% (libcore:sy-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
