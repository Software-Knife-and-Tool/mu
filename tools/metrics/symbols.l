;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(mu:intern mu:%null-ns% "symbols"
   (:lambda (ns file-name)
      ((:lambda (stream)
          (core:%mapc
           (:lambda (symbol)
             (core:%format stream "~A~%" `(,(mu:symbol-name symbol)))
             (core:%format stream "~A:~A~%" `(,ns ,(mu:symbol-name symbol))))
           (mu:namespace-symbols (mu:find-namespace ns)))
          (mu:close stream))
       (mu:open :file :output file-name :t))
      :t))
