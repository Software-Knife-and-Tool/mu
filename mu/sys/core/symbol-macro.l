;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(mu:intern core "%read-keywordp"
  (:lambda (name)
      (mu:eq #\: (mu:svref name 0))))

(mu:intern core "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (core:null colon)
            ()
            (mu:find-namespace (core:%substr name 0 (mu:sub colon 1)))))
     (core:%string-position #\: name))))

(mu:intern core "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (core:null colon)
             name
             (core:%substr name
                (:if (mu:eq #\: (mu:svref name (mu:add 1 colon)))
                     (mu:add 2 colon)
                     (mu:add 1 colon))
                (:if (mu:eq #\: (mu:svref name (mu:add 1 colon)))
                     (mu:add colon (mu:sub (mu:vector-length name) colon))
                     (mu:add colon (mu:sub (mu:vector-length name) (mu:add 1 colon)))))))
      (core:%string-position #\: name))))

(mu:intern core "%read-symbol"
   (:lambda (symbol)
     (:if (core:%read-keywordp symbol)
          (core:%make-keyword (core:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (core:null ns)
                  ((:lambda (symbol)
                     (:if (core:null symbol)
                          (mu:intern mu:*null/* name ())
                          symbol))
                   (mu:find mu:*null/* name))
                  ((:lambda (symbol)
                     (:if (core:null symbol)
                          (mu:intern ns name ())
                          symbol))
                   (mu:find ns name))))
             (core:%read-symbol-ns symbol)
             (core:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(mu:intern core "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (core:%or (core:null symbol) (core:keywordp symbol))
          symbol
          (:if (core:%uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (mu:eval (mu:symbol-value sym))
                       symbol))
                (core:%get-symbol-macro symbol))
               symbol))))

(mu:intern core "%get-symbol-macro"
   (:lambda (symbol)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:find core:*symbol-macros/* (mu:symbol-name symbol))
           (core:%raise symbol 'core:%get-symbol-macro "not a symbol"))))

(mu:intern core "define-symbol-macro"
   (:lambda (symbol form)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:intern core:*symbol-macros/* (mu:symbol-name symbol) form)
           (core:%raise symbol 'core:define-symbol-macro "not a symbol"))))

(core:define-symbol-macro 't :t)
(core:define-symbol-macro 'nil :nil)
