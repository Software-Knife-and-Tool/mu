;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common namespace
;;;
(mu:intern mu:%null-ns% "common" (mu:make-namespace "common"))
(mu:intern common "+version+" "0.0.1")

(core:provide
 "common"
 '("boole.l"
   "defun.l"
   "describe.l"
   "fixnum.l"
   "list.l"
   "macros.l"
   "predicates.l"
   "print.l"
   "sequence.l"
   "stream.l"
   "string.l"
   "symbol.l"
   "time.l"))
