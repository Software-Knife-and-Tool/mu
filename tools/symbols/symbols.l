;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(lib:intern (lib:find-ns "prelude") "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (lib:symbol-ns symbol))
                   (prelude:format stream "~A~%" `(,(lib:symbol-name symbol)))
                   (prelude:format stream "prelude:~A~%" `(,(lib:symbol-name symbol)))))
           (lib:symbols (lib:find-ns "prelude")))
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (lib:symbol-ns symbol))
                   (prelude:format stream "~A~%" `(,(lib:symbol-name symbol)))
                   (prelude:format stream "lib:~A~%" `(,(lib:symbol-name symbol)))))
           (lib:symbols (lib:find-ns "lib"))))
       (lib:open :file :output file-name))
      :t))
