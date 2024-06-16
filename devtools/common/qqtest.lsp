;;; SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
;;; SPDX-License-Identifier: MIT
;;; SPDX-Filename: common/qqtest.lsp

;;;
;;; quasiquote test forms
;;;

;;; if this compiles, then our mu test
;;; cases are all right.
(defvar test-cases
  '(`"abc"
    `#(1 2 3)
    `#\a
    `*standard-output*
    `1234
    `(,(cons 'satisfies ()))
    `("abc")
    `(#(1 2 3))
    `(#\a)
    `(())
    `((1234) 1234 symbol)
    `((a b) c)
    `()
    `(,(+ 1 2))
    `(,1 (2) 3)
    `(,1234 ,@'(a b c))
    `(,@'(a b c))
    `(,@(+ 1 2))
    `(0 ,@'(a b c) 1)
    `(1 2 ,@3)
    `(1 2 3)
    `(1.0 b (2))
    `(1234 symbol)
    `(1234)
    `(a b c)
    `,(+ 1 2)
    `,(type-of 'symbol)
    `,1234
    `,`"abc"
    `,`#(1 2 3)
    `,`#\a
    `,`*standard-output*
    `,`1234
    `,`(1234)
    ``1234))
