;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mu require
;;;
(defconst libcore::require-ns (libcore:ns (symbol-name (gensym)) (env:find-ns "mu")))
(libcore:in-namespace libcore::require-ns)

(defmacro libcore:require-lib (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib: not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (libcore:ns-find libcore::require-ns :extern lib)
      ()
      (let ((lib-path (libcore:format () "~A/~A:~A.l" (list libcore::require-path "eko" lib))))
        (libcore:intern libcore::require-ns :extern lib)
        (libcore:load lib-path () ())))))

(defmacro libcore:require-lib-verbose (lib-sym)
  (errorp-unless symbolp lib-sym "require-lib not a symbol")
  (let ((lib (symbol-name lib-sym)))
    (if (libcore:ns-find libcore::require-ns :extern lib)
      ()
      (let ((lib-path (libcore:format () "~A/~A:~A.l" (list libcore::require-path "eko" lib))))
        (libcore:intern libcore::require-ns :extern lib)
        (libcore:load lib-path :t ())))))

