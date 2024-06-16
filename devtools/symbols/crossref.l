;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(mu:intern (mu:find-ns "prelude") "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (prelude:mapc
           (:lambda (symbol)
              (:if (mu:boundp symbol)
                   (prelude:format stream "~A~T~A~T~S~%"
                                `(,(mu:symbol-name symbol)
                                  ,(prelude:type-of (mu:symbol-value symbol))
                                  ,(mu:symbol-value symbol)))
                   (prelude:format stream "unbound~Tunbound~T~A~%" `(,(mu:symbol-name symbol)))))
           (mu:symbols (mu:find-ns "prelude"))))
       (mu:open :file :output file-name))
      :t))
