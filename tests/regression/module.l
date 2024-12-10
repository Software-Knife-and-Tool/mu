;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; common test
;;;
(mu:intern mu:%null-ns% "eval-module-form"
   (:lambda (form-str)
     (mu:eval
      (mu:compile
       (mu:read
        (mu:open :string :input form-str :t) :t ())))))
