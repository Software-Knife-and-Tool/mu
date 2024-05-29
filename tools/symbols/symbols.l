;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(crux:intern (crux:find-ns "prelude") "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(crux:symbol-name symbol)))
             (prelude:format stream "prelude:~A~%" `(,(crux:symbol-name symbol))))
           (crux:symbols (crux:find-ns "prelude")))
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "~A~%" `(,(crux:symbol-name symbol)))
             (prelude:format stream "crux:~A~%" `(,(crux:symbol-name symbol))))
           (crux:symbols (crux:find-ns "crux"))))
       (crux:open :file :output file-name))
      :t))
