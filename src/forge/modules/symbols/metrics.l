;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(mu:make-namespace "symbols")
((:lambda (symbols)
   (mu:intern symbols "metrics"
      (:lambda (ns file-name)
        ((:lambda (stream)
           (core:mapc
            (:lambda (symbol)
              (core:%format stream "~A~%" `(,(mu:symbol-name symbol)))
              (core:%format stream "~A:~A~%" `(,ns ,(mu:symbol-name symbol))))
            (feature/core:ns-symbols (mu:find-namespace ns)))
           (mu:close stream))
         (mu:open :file :output file-name :t))
        :t)))
 (mu:find-namespace "symbols"))
