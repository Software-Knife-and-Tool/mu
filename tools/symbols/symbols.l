;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(lib:intern :prelude "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (lib:sy-ns symbol))
                   (prelude:format stream "~A~%" `(,(lib:sy-name symbol)))
                   (prelude:format stream "prelude:~A~%" `(,(lib:sy-name symbol)))))
           (lib:ns-syms :list :prelude))
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (lib:sy-ns symbol))
                   (prelude:format stream "~A~%" `(,(lib:sy-name symbol)))
                   (prelude:format stream "lib:~A~%" `(,(lib:sy-name symbol)))))
           (lib:ns-syms :list :lib)))
       (lib:open :file :output file-name))
      :t))
