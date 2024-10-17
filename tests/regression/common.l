;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common test
;;;
(mu:intern mu:%null-ns% "eval-common-form"
   (:lambda (form-str)
     (mu:eval
      (core:compile
       (core:read
        (mu:open :string :input form-str) :t ())))))
