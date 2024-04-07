;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(lib:intern :prelude "%read-keywordp"
  (:lambda (name)
      (lib:eq #\: (prelude:schar name 0))))

(lib:intern :prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (lib:eq #\: (prelude:schar name (lib:fx-add 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(lib:intern :prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (lib:keyword (prelude:substr name 0 (lib:fx-sub colon 1)))))
     (prelude:string-position #\: name))))

(lib:intern :prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                          (:if (lib:eq #\: (prelude:schar name (lib:fx-add 1 colon)))
                               (lib:fx-add 2 colon)
                               (lib:fx-add 1 colon))
                          (:if (lib:eq #\: (prelude:schar name (lib:fx-add 1 colon)))
                               (lib:fx-add colon (lib:fx-sub (lib:sv-len name) colon))
                               (lib:fx-add colon (lib:fx-sub (lib:sv-len name) (lib:fx-add 1 colon)))))))
     (prelude:string-position #\: name))))

(lib:intern :prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  (lib:unbound () name)
                  (lib:unbound ns name)))
           (prelude:%read-symbol-ns symbol)
           (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(lib:intern :prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (lib:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(lib:intern :prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (lib:eq :symbol (lib:type-of symbol))
           (lib:ns-find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(lib:intern :prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (lib:eq :symbol (lib:type-of symbol))
           (lib:intern prelude:%symbol-macro-ns% (lib:sy-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
