;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol table
;;;
(mu:intern :prelude "symbols"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (mu:sy-ns symbol))
                   (prelude:format stream "~A~%" `(,(mu:sy-name symbol)))
                   (prelude:format stream "prelude:~A~%" `(,(mu:sy-name symbol)))))
           (mu:ns-syms :list :prelude))
          (prelude:mapc
           (:lambda (symbol)
              (:if (prelude:null (mu:sy-ns symbol))
                   (prelude:format stream "~A~%" `(,(mu:sy-name symbol)))
                   (prelude:format stream "mu:~A~%" `(,(mu:sy-name symbol)))))
           (mu:ns-syms :list :mu))
          (prelude:mapc
           (:lambda (symbol)
             (prelude:format stream "sys:~A~%" `(,(mu:sy-name symbol))))
           (mu:ns-syms :list :sys)))
       (mu:open :file :output file-name))
      :t))
