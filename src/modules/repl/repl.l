;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl namespace
;;;
(mu:intern mu:%null-ns% "repl" (mu:make-namespace "repl"))
(mu:intern repl "+version+" "0.0.1")

(core:provide
 "repl"
 '("break.l"
   "loop.l"))
