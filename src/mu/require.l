;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mu require
;;;
(defconst mu::require-ns (mu:ns (symbol-name (gensym)) (env:find-ns "mu")))
(core:in-namespace mu::require-ns)

(defmacro mu:require-lib (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib: not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (mu:ns-find mu::require-ns :extern lib)
      ()
      (let ((lib-path (core:format () "~A/~A:~A.l" (list mu::require-path "eko" lib))))
        (mu:intern mu::require-ns :extern lib)
        (core:load lib-path () ())))))

(defmacro mu:require-lib-verbose (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (mu:ns-find mu::require-ns :extern lib)
      ()
      (let ((lib-path (core:format () "~A/~A:~A.l" (list mu::require-path "eko" lib))))
        (mu:intern mu::require-ns :extern lib)
        (core:load lib-path :t ())))))

