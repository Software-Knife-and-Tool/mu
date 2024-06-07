;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(mu:intern (mu:find-ns "prelude") "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(mu:symbol-name symbol)))
             (prelude:format stream "prelude:~A~%" `(,(mu:symbol-name symbol))))
           (mu:symbols (mu:find-ns "prelude")))
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(mu:symbol-name symbol)))
             (prelude:format stream "mu:~A~%" `(,(mu:symbol-name symbol))))
           (mu:symbols (mu:find-ns "mu"))))
       (mu:open :file :output file-name))
      :t))
