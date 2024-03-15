;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(mu:intern :prelude "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (mu:boundp symbol)
                   (prelude:format stream "~A~T~A~T~S~%"
                                `(,(mu:sy-name symbol)
                                  ,(prelude:type-of (mu:sy-val symbol))
                                  ,(mu:sy-val symbol)))
                   (prelude:format stream "unbound~Tunbound~T~A~%" `(,(mu:sy-name symbol)))))
           (mu:ns-syms :list :prelude)))
       (mu:open :file :output file-name))
      :t))
