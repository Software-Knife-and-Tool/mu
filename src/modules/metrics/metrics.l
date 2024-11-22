;;;
;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; metrics namespace
;;;
(mu:intern mu:%null-ns% "metrics" (mu:make-namespace "metrics"))
(mu:intern metrics "+version+" "0.0.1")

(core:provide
 "metrics"
 '("room.l"
   "time.l"))
