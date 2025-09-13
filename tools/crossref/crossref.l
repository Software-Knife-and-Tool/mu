;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; cross reference
;;;
(mu:intern mu:*null/* "crossref"
   (:lambda (file-name)
      ((:lambda (stream)
          (core:%mapc
           (:lambda (symbol)
              (:if (mu:boundp symbol)
                   (core:%format stream "~A~T~A~T~S~%"
                                `(,(mu:symbol-name symbol)
                                  ,(core:type-of (mu:symbol-value symbol))
                                  ,(mu:symbol-value symbol)))
                   (core:%format stream "unbound~Tunbound~T~A~%" `(,(mu:symbol-name symbol)))))
           (feature/core:ns-symbols (mu:find-namespace "core")))
         (mu:close stream))
       (mu:open :file :output file-name :t))
      :t))
