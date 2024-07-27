;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(mu:intern (mu:find-namespace "prelude") "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(mu:symbol-name symbol)))
             (prelude:format stream "prelude:~A~%" `(,(mu:symbol-name symbol))))
           (mu:namespace-symbols (mu:find-namespace "prelude")))
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(mu:symbol-name symbol)))
             (prelude:format stream "mu:~A~%" `(,(mu:symbol-name symbol))))
           (mu:namespace-symbols (mu:find-namespace "mu")))
          (mu:close stream))
       (mu:open :file :output file-name))
      :t))
