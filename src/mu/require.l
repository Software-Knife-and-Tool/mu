;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mu require
;;;
(defconst lib::require-ns (lib:ns (symbol-name (gensym)) (env:find-ns "mu")))
(lib:in-namespace lib::require-ns)

(defmacro lib:require-lib (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib: not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (lib:ns-find lib::require-ns :extern lib)
      ()
      (let ((lib-path (lib:format () "~A/~A:~A.l" (list lib::require-path "eko" lib))))
        (lib:intern lib::require-ns :extern lib)
        (lib:load lib-path () ())))))

(defmacro lib:require-lib-verbose (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (lib:ns-find lib::require-ns :extern lib)
      ()
      (let ((lib-path (lib:format () "~A/~A:~A.l" (list lib::require-path "eko" lib))))
        (lib:intern lib::require-ns :extern lib)
        (lib:load lib-path :t ())))))

