;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern common "boundp" mu:boundp)

(mu:intern common "fboundp"
  (:lambda (symbol)
    (:if (mu:boundp symbol)
         (core:functionp (mu:symbol-value symbol))
         ())))

(mu:intern common "uninternedp"
   (:lambda (sym)
      (common:null (common:symbol-namespace sym))))

(mu:intern common "keywordp"
   (:lambda (t)
      (mu:eq (core:type-of t) 'keyword)))

(mu:intern common "symbolp"
   (:lambda (t)
      (core:%orf (core:%orf (core:%keywordp t) (mu:eq () t))
                 (mu:eq 'symbol (core:type-of t)))))
