;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(lib:intern :prelude "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (lib:boundp symbol)
                   (prelude:format stream "~A~T~A~T~S~%"
                                `(,(lib:sy-name symbol)
                                  ,(prelude:type-of (lib:sy-val symbol))
                                  ,(lib:sy-val symbol)))
                   (prelude:format stream "unbound~Tunbound~T~A~%" `(,(lib:sy-name symbol)))))
           (lib:ns-syms :list :prelude)))
       (lib:open :file :output file-name))
      :t))
