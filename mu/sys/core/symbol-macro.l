;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

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
           (mu:find core:symbol-macros/ (mu:symbol-name symbol))
           (mu:raise-from symbol 'core:%get-symbol-macro :type))))

(mu:intern core "define-symbol-macro"
   (:lambda (symbol form)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:intern core:symbol-macros/ (mu:symbol-name symbol) form)
           (mu:raise-from symbol 'core:define-symbol-macro :type))))

(core:define-symbol-macro 't :t)
(core:define-symbol-macro 'nil :nil)
