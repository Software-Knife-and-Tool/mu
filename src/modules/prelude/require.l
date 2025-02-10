;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mu require
;;;
(defconst core::require-ns (core:ns (symbol-name (gensym)) (env:find-ns "mu")))
(core:in-namespace core::require-ns)

(defmacro core:require-lib (lib-sym)
  (errorp-unless symbolp lib-sym "require-core: not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (%if (core:ns-find core::require-ns :extern lib)
      ()
      (let ((lib-path (core:%format () "~A/~A:~A.l" (list core::require-path "eko" lib))))
        (core:intern core::require-ns :extern lib)
        (core:load lib-path () ())))))

(defmacro core:require-lib-verbose (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (%if (core:ns-find core::require-ns :extern lib)
      ()
      (let ((lib-path (core:%format () "~A/~A:~A.l" (list core::require-path "eko" lib))))
        (core:intern core::require-ns :extern lib)
        (core:load lib-path :t ())))))

